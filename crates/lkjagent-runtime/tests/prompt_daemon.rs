mod support;

use std::fs;

use lkjagent_context::budget::{
    PREFIX_GRAMMAR_REGISTRY, PREFIX_IDENTITY, PREFIX_MEMORY_DIGEST, PREFIX_SKILL_INDEX,
    PREFIX_WORKSPACE_BRIEF,
};
use lkjagent_context::model::{FrameKind, PrefixSection};
use lkjagent_runtime::daemon::{
    request_shutdown, seed_skill_library, startup_state, take_daemon_lock, ShutdownDecision,
    ShutdownState, Signal, StartupLock,
};
use lkjagent_runtime::prompt::{build_prefix, PromptInputs};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::events::read_events;
use support::{prefix, store, temp_workspace, TestResult};

#[test]
fn prompt_is_deterministic_and_within_section_budgets() -> TestResult<()> {
    let inputs = PromptInputs {
        skill_index: "demo-skill: test trigger.".to_string(),
        workspace_brief: "workspace brief".to_string(),
        memory_digest: "memory digest".to_string(),
    };
    let first = build_prefix(&inputs)?;
    let second = build_prefix(&inputs)?;
    assert_eq!(first, second);
    for frame in first {
        let cap = match frame.kind {
            FrameKind::Prefix(PrefixSection::Identity) => PREFIX_IDENTITY,
            FrameKind::Prefix(PrefixSection::GrammarRegistry) => PREFIX_GRAMMAR_REGISTRY,
            FrameKind::Prefix(PrefixSection::SkillIndex) => PREFIX_SKILL_INDEX,
            FrameKind::Prefix(PrefixSection::WorkspaceBrief) => PREFIX_WORKSPACE_BRIEF,
            FrameKind::Prefix(PrefixSection::MemoryDigest) => PREFIX_MEMORY_DIGEST,
            _ => 0,
        };
        assert!(frame.tokens.0 <= cap);
    }
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
fn seed_skill_library_copies_empty_dir_without_overwriting() -> TestResult<()> {
    let root = temp_workspace("seed-copy")?;
    let source = root.join("source");
    let target = root.join("target");
    fs::create_dir_all(&source)?;
    fs::write(source.join("seed.md"), "seed body")?;

    seed_skill_library(&target, &root.join("missing-image"), &source)?;
    assert_eq!(fs::read_to_string(target.join("seed.md"))?, "seed body");

    fs::write(target.join("seed.md"), "custom body")?;
    fs::write(source.join("other.md"), "other body")?;
    seed_skill_library(&target, &root.join("missing-image"), &source)?;
    assert_eq!(fs::read_to_string(target.join("seed.md"))?, "custom body");
    assert!(!target.join("other.md").exists());
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
