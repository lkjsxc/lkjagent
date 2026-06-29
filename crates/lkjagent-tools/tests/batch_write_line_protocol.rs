mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn batch_write_accepts_empty_duplicate_stub_before_content() -> TestResult<()> {
    let workspace = temp_workspace("batch-empty-duplicate-stub")?;
    let files = "path: docs/a.md\ncontent:\n\n-- lkjagent-next-file --\npath: docs/a.md\ncontent:\n# A\n\nConcrete content.";
    let output = run_batch(&workspace, files)?;

    assert!(output.contains("files_written=1"));
    assert!(workspace.join("docs/a.md").is_file());
    Ok(())
}

#[test]
fn batch_write_requires_separator_before_second_path_header() -> TestResult<()> {
    let workspace = temp_workspace("batch-nested-path-header")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let files = "path: stories/a/catalog.toml\ncontent:\npath: stories/a/README.md\ncontent:\n# A";
    let output = dispatch(
        &action("fs.batch_write", &[("files", files)]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("requires -- lkjagent-next-file --"));
    assert!(!workspace.join("stories/a/catalog.toml").exists());
    assert!(!workspace.join("stories/a/README.md").exists());
    Ok(())
}

fn run_batch(workspace: &std::path::Path, files: &str) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action("fs.batch_write", &[("files", files)]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}
