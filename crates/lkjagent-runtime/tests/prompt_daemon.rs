mod support;

use lkjagent_context::budget::{
    PREFIX_GRAMMAR_REGISTRY, PREFIX_GRAPH_STATE, PREFIX_IDENTITY, PREFIX_MEMORY_DIGEST,
    PREFIX_WORKSPACE_BRIEF,
};
use lkjagent_context::model::{FrameKind, PrefixSection};
use lkjagent_runtime::daemon::{
    build_prefix_from_store, request_shutdown, startup_state, take_daemon_lock, ShutdownDecision,
    ShutdownState, Signal, StartupLock,
};
use lkjagent_runtime::graph_state::{open_owner_case_with_guard, render_state};
use lkjagent_runtime::prompt::{build_prefix, PromptInputs};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::events::read_events;
use lkjagent_store::memory::{save, MemoryKind};
use lkjagent_store::state;
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::count_guard::CountMode;
use support::{prefix, store, temp_workspace, TestResult};

#[test]
fn prompt_is_deterministic_and_within_section_budgets() -> TestResult<()> {
    let inputs = PromptInputs {
        graph_state: "case=1\nphase=planning\nnode=plan".to_string(),
        workspace_brief: "workspace brief".to_string(),
        memory_digest: "memory digest".to_string(),
    };
    let first = build_prefix(&inputs)?;
    let second = build_prefix(&inputs)?;
    assert_eq!(first, second);
    assert!(first.iter().any(|frame| frame
        .content
        .contains("Do not\nact directly from the first owner message")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("bounded graph-maintenance work")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("fs.list, fs.search, fs.stat")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("doc tools, not shell loops")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("shell.run is an")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("doc.audit")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("root means a directory")));
    assert!(first
        .iter()
        .any(|frame| frame.content.contains("line protocol is canonical")));
    for frame in first {
        let cap = match frame.kind {
            FrameKind::Prefix(PrefixSection::Identity) => PREFIX_IDENTITY,
            FrameKind::Prefix(PrefixSection::GrammarRegistry) => PREFIX_GRAMMAR_REGISTRY,
            FrameKind::Prefix(PrefixSection::GraphState) => PREFIX_GRAPH_STATE,
            FrameKind::Prefix(PrefixSection::WorkspaceBrief) => PREFIX_WORKSPACE_BRIEF,
            FrameKind::Prefix(PrefixSection::MemoryDigest) => PREFIX_MEMORY_DIGEST,
            _ => 0,
        };
        assert!(frame.tokens.0 <= cap);
    }
    Ok(())
}

#[test]
fn startup_trims_rendered_memory_digest_to_prefix_budget() -> TestResult<()> {
    let mut conn = store()?;
    let content = "x ".repeat(600);
    for index in 0..40 {
        save(
            &mut conn,
            MemoryKind::Fact,
            &format!("oversized digest row {index}"),
            "startup-memory",
            &content,
            1,
            "2026-06-19T00:00:00Z",
        )?;
    }
    let workspace = temp_workspace("digest-budget")?;
    let prefix = build_prefix_from_store(&conn, &workspace)?;
    let memory = prefix
        .iter()
        .find(|frame| frame.kind == FrameKind::Prefix(PrefixSection::MemoryDigest))
        .ok_or("missing memory digest frame")?;
    assert!(memory.tokens.0 <= PREFIX_MEMORY_DIGEST);
    assert!(memory.content.contains("kind=fact"));
    Ok(())
}

#[test]
fn startup_prefix_renders_count_guard_batch_instruction() -> TestResult<()> {
    let conn = store()?;
    state::set(&conn, "completion guard", "file-count-about:100")?;
    let workspace = temp_workspace("count-guard-prefix")?;

    let prefix = build_prefix_from_store(&conn, &workspace)?;
    let graph = prefix
        .iter()
        .find(|frame| frame.kind == FrameKind::Prefix(PrefixSection::GraphState))
        .ok_or("missing graph state frame")?;

    assert!(graph
        .content
        .contains("completion_guard=file-count-about:100"));
    assert!(graph.content.contains("shell.run is an escape hatch"));
    assert!(graph.content.contains("doc.scaffold"));
    assert!(graph.content.contains("fs.list"));
    assert!(graph.content.contains("fs.stat"));
    assert!(graph.content.contains("doc.audit"));
    assert!(graph.content.contains("fs.batch_write"));
    Ok(())
}

#[test]
fn owner_graph_notice_renders_count_guard_batch_instruction() -> TestResult<()> {
    let conn = store()?;
    let graph = open_owner_case_with_guard(
        &conn,
        "Create about 100 files total.",
        "101",
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate,
        },
    )?;

    let rendered = render_state(&graph);

    assert!(rendered.contains("completion_guard=file-count-about:100"));
    assert!(rendered.contains("shell.run is an escape hatch"));
    assert!(rendered.contains("doc.scaffold"));
    assert!(rendered.contains("fs.list"));
    assert!(rendered.contains("fs.stat"));
    assert!(rendered.contains("doc.audit"));
    assert!(rendered.contains("fs.batch_write"));
    Ok(())
}

#[test]
fn startup_resumes_from_summary_without_raw_replay() -> TestResult<()> {
    let state = startup_state(prefix()?, Some("task summary only".to_string()));
    assert_eq!(state.context.log.len(), 1);
    assert!(state
        .context
        .log
        .first()
        .is_some_and(|frame| frame.content.contains("task summary only")));
    assert!(matches!(state.task, TaskState::Open { .. }));
    Ok(())
}

#[test]
fn daemon_lock_takes_refuses_and_records_reclaim_notice() -> TestResult<()> {
    let conn = store()?;
    assert_eq!(
        take_daemon_lock(&conn, "pid1", "100", "0")?,
        StartupLock::Taken
    );
    assert!(matches!(
        take_daemon_lock(&conn, "pid2", "101", "50")?,
        StartupLock::Refused { .. }
    ));
    assert!(matches!(
        take_daemon_lock(&conn, "pid2", "400", "300")?,
        StartupLock::Reclaimed { .. }
    ));
    let events = read_events(&conn)?;
    assert!(events
        .iter()
        .any(|event| event.content.contains("reclaimed stale daemon lock")));
    Ok(())
}

#[test]
fn shutdown_request_finishes_in_flight_turn_before_exit() {
    let (next, decision) = request_shutdown(
        ShutdownState {
            stop_requested: false,
            in_flight: true,
        },
        Signal::Terminate,
    );
    assert!(next.stop_requested);
    assert_eq!(decision, ShutdownDecision::FinishTurnThenExit);

    let (_, decision) = request_shutdown(
        ShutdownState {
            stop_requested: false,
            in_flight: false,
        },
        Signal::Interrupt,
    );
    assert_eq!(decision, ShutdownDecision::ExitNow);
}
