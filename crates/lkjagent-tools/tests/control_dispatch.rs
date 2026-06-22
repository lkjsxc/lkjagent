mod support;

use std::fs;

use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;
use lkjagent_tools::dispatch::{dispatch, dispatch_with_text};
use support::{action, runtime, state, store, temp_workspace, TestResult};

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
    let vague_ask = dispatch(
        &action("agent.ask", &[("question", "Need input?")]),
        &runtime,
        &mut conn,
        &mut ask_state,
    );
    assert!(is_error(&vague_ask));
    assert!(vague_ask.content.contains("concrete external"));
    let ask = dispatch(
        &action(
            "agent.ask",
            &[("question", "Should the report focus on Rust or SQLite?")],
        ),
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

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        lkjagent_tools::observe::OutputKind::Observation { status } if status == "error"
    )
}
