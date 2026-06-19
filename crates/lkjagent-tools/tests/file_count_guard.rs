mod support;

use std::fs;

use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::count_guard::CountMode;
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
        CompletionGuard::FileCount {
            target: 3,
            mode: CountMode::Exact
        }
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

#[test]
fn approximate_file_count_guard_accepts_tolerance() -> TestResult<()> {
    let workspace = temp_workspace("file-count-about")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create about 10 files total");
    assert_eq!(
        state.control.guard,
        CompletionGuard::FileCount {
            target: 10,
            mode: CountMode::Approximate
        }
    );
    fs::create_dir_all(workspace.join("bundle"))?;
    fs::write(workspace.join("bundle/README.md"), "# Bundle\n")?;
    for index in 1..=8 {
        fs::write(workspace.join(format!("bundle/file-{index}.txt")), "x\n")?;
    }

    let done = dispatch(
        &action("agent.done", &[("summary", "nine files")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(done.content.contains("summary=nine files"));
    Ok(())
}

#[test]
fn approximate_file_count_guard_refuses_outside_tolerance() -> TestResult<()> {
    let workspace = temp_workspace("file-count-about-refuse")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create roughly 10 files total");
    fs::create_dir_all(workspace.join("bundle"))?;
    fs::write(workspace.join("bundle/README.md"), "# Bundle\n")?;
    for index in 1..=6 {
        fs::write(workspace.join(format!("bundle/file-{index}.txt")), "x\n")?;
    }

    let early = dispatch(
        &action("agent.done", &[("summary", "seven files")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&early));
    assert!(early.content.contains("need about 10 files (9-11)"));
    Ok(())
}

#[test]
fn shell_errors_hint_against_hardcoded_workspace_path() -> TestResult<()> {
    let workspace = temp_workspace("shell-workspace-hint")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let output = dispatch(
        &action(
            "shell.run",
            &[("command", "cd /workspace/definitely-missing-lkjagent")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&output));
    assert!(output.content.contains("do not cd /workspace"));
    Ok(())
}

#[test]
fn count_guard_accepts_plain_single_root_without_readme() -> TestResult<()> {
    let workspace = temp_workspace("file-count-plain-root")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create exactly 3 files total");
    fs::create_dir_all(workspace.join("story"))?;
    for index in 1..=3 {
        fs::write(workspace.join(format!("story/chapter-{index}.md")), "x\n")?;
    }

    let done = dispatch(
        &action("agent.done", &[("summary", "plain story root")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(done.content.contains("summary=plain story root"));
    Ok(())
}

#[test]
fn count_guard_accepts_clean_sibling_roots_without_readme() -> TestResult<()> {
    let workspace = temp_workspace("file-count-sibling-roots")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create exactly 3 files total");
    fs::create_dir_all(workspace.join("docs"))?;
    fs::create_dir_all(workspace.join("main"))?;
    fs::write(workspace.join("docs/one.md"), "x\n")?;
    fs::write(workspace.join("main/two.md"), "x\n")?;
    fs::write(workspace.join("main/three.md"), "x\n")?;

    let done = dispatch(
        &action("agent.done", &[("summary", "docs plus main roots")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(done.content.contains("summary=docs plus main roots"));
    Ok(())
}

#[test]
fn count_guard_refuses_plain_roots_mixed_with_top_level_files() -> TestResult<()> {
    let workspace = temp_workspace("file-count-mixed-root")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    state.control.start_task("create exactly 3 files total");
    fs::create_dir_all(workspace.join("docs"))?;
    fs::write(workspace.join("docs/one.md"), "x\n")?;
    fs::write(workspace.join("docs/two.md"), "x\n")?;
    fs::write(workspace.join("loose.md"), "x\n")?;

    let early = dispatch(
        &action("agent.done", &[("summary", "mixed root")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&early));
    assert!(early.content.contains("no README.md candidate found"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
