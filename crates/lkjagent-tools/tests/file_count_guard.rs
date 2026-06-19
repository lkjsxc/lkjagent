mod support;

use std::fs;

use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn control_guards_requested_general_file_count() -> TestResult<()> {
    let workspace = temp_workspace("file-count")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create exactly 3 files total");
    assert_eq!(
        state.control.guard,
        CompletionGuard::FileCount { target: 3 }
    );
    fs::create_dir_all(workspace.join("bundle"))?;
    fs::write(workspace.join("bundle/README.md"), "# Bundle\n")?;
    fs::write(workspace.join("bundle/one.txt"), "one\n")?;

    let early = dispatch(
        &action("agent.done", &[("summary", "two files")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&early));
    assert!(early.content.contains("need exactly 3 files"));
    fs::write(workspace.join("bundle/two.txt"), "two\n")?;

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
        OutputKind::Observation { status } if status == "error"
    )
}
