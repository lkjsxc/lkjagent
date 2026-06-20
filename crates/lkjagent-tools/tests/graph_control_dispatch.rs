mod support;

use std::fs;

use lkjagent_protocol::{Action, Param};
use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;
use lkjagent_tools::dispatch::{dispatch, dispatch_with_text};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn graph_tools_report_state_and_record_evidence() -> TestResult<()> {
    let workspace = temp_workspace("graph")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_state = Some("case=1\nphase=planning\nmissing=observation".to_string());

    let shown = dispatch(&action("graph.state", &[]), &runtime, &mut conn, &mut state);
    assert!(matches!(shown.kind, OutputKind::Observation { .. }));
    assert!(shown.content.contains("phase=planning"));

    let recorded = dispatch(
        &action(
            "graph.evidence",
            &[
                ("kind", "observation"),
                ("summary", "read README"),
                ("path", "README.md"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(recorded.content.contains("graph evidence recorded"));
    assert_eq!(state.graph_evidence.len(), 1);
    assert_eq!(state.graph_evidence[0].path.as_deref(), Some("README.md"));
    Ok(())
}

#[test]
fn agent_done_refusal_points_to_missing_graph_evidence() -> TestResult<()> {
    let workspace = temp_workspace("graph-done-refusal")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state.graph_state = Some("case=1\nphase=execution".to_string());
    state.graph_completion_ready = false;
    state.graph_missing = vec!["document-structure".to_string()];

    let refused = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&refused));
    assert!(refused.content.contains("graph completion refused"));
    assert!(refused.content.contains("graph.evidence"));
    assert!(refused.content.contains("kind=document-structure"));
    Ok(())
}

#[test]
fn control_tools_close_wait_and_report_errors() -> TestResult<()> {
    let workspace = temp_workspace("control")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut done_state = state();

    let done = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut done_state,
    );
    assert!(done.content.contains("summary=finished"));
    let second_done = dispatch_with_text(
        &action("agent.done", &[("summary", "again")]),
        "done-again",
        &runtime,
        &mut conn,
        &mut done_state,
    );
    assert!(is_error(&second_done));

    let mut ask_state = state();
    let ask = dispatch(
        &action("agent.ask", &[("question", "Need input?")]),
        &runtime,
        &mut conn,
        &mut ask_state,
    );
    assert!(ask.content.contains("waiting"));
    let second_ask = dispatch(
        &action("agent.ask", &[("question", "Again?")]),
        &runtime,
        &mut conn,
        &mut ask_state,
    );
    assert!(is_error(&second_ask));
    assert!(second_ask.content.contains("already outstanding"));
    Ok(())
}

#[test]
fn control_classifies_recursive_knowledge_requests() {
    let mut state = ControlState::default();

    state.start_task("百科事典を作ってください");

    assert_eq!(state.guard, CompletionGuard::RecursiveKnowledge);
}

#[test]
fn control_guards_requested_markdown_file_count() -> TestResult<()> {
    let workspace = temp_workspace("markdown-count")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state
        .control
        .start_task("create exactly 3 markdown files total");
    assert_eq!(
        state.control.guard,
        CompletionGuard::MarkdownCount {
            target: 3,
            mode: CountMode::Exact
        }
    );
    fs::create_dir_all(workspace.join("docs/count"))?;
    fs::write(workspace.join("docs/count/README.md"), "# Count\n")?;
    fs::write(workspace.join("docs/count/one.md"), "# One\n")?;

    let early = dispatch(
        &action("agent.done", &[("summary", "two files")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&early));
    assert!(early.content.contains("need exactly 3 markdown files"));
    fs::write(workspace.join("docs/count/two.md"), "# Two\n")?;

    let done = dispatch(
        &action("agent.done", &[("summary", "three files")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(done.content.contains("summary=three files"));
    Ok(())
}

#[test]
fn dispatcher_reports_validation_and_repeat_notices() -> TestResult<()> {
    let workspace = temp_workspace("dispatch")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new(
        "fs.read",
        vec![Param::new("bogus", "x"), Param::new("bogus", "y")],
    );
    let validation = dispatch(&bad, &runtime, &mut conn, &mut state);
    assert!(matches!(validation.kind, OutputKind::Notice { .. }));
    assert!(validation.content.contains("action params refused"));
    assert!(validation.content.contains("duplicate=bogus"));
    assert!(validation.content.contains("missing=path"));
    assert!(validation.content.contains("unknown=bogus"));
    assert!(validation.content.contains("valid_example:"));

    let unknown = dispatch(
        &Action::new("think", Vec::new()),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(unknown.kind, OutputKind::Notice { .. }));
    assert!(unknown.content.contains("valid tools"));

    let ask = action("agent.ask", &[("question", "Continue?")]);
    let first = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    let second = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    assert!(first.content.contains("waiting"));
    assert!(matches!(second.kind, OutputKind::Notice { .. }));
    assert!(second.content.contains("repeat action refused"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
