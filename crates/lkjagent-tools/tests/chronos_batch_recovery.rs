mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn chronos_invalid_json_files_refuses_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("chronos-invalid-json")?;
    let output = run_batch(
        &workspace,
        r##"[{"path":"stories/chronos-fracture/README.md","content":"# Chronos"}"##,
    )?;

    assert!(output.contains("invalid JSON fs.batch_write files payload"));
    assert!(!workspace
        .join("stories/chronos-fracture/README.md")
        .exists());
    Ok(())
}

#[test]
fn chronos_oversized_readme_refuses_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("chronos-oversized-readme")?;
    std::fs::create_dir_all(workspace.join("stories/chronos-fracture"))?;
    let files = format!(
        "path: stories/chronos-fracture/README.md\ncontent:\n# Chronos Fracture\n\n{}",
        "concrete story bible material. ".repeat(220)
    );
    let output = run_batch(&workspace, &files)?;

    assert!(output.contains("payload too large for fs.batch_write file"));
    assert!(output.contains("stories/chronos-fracture/README.md"));
    assert!(!workspace
        .join("stories/chronos-fracture/README.md")
        .exists());
    Ok(())
}

#[test]
fn chronos_bounded_batch_writes_catalog_and_leaf() -> TestResult<()> {
    let workspace = temp_workspace("chronos-bounded-batch")?;
    let output = run_batch(
        &workspace,
        "path: stories/chronos-fracture/catalog.toml\ncontent:\n[artifact]\nroot = \"stories/chronos-fracture\"\nkind = \"story\"\ntitle = \"Chronos Fracture\"\n\n-- lkjagent-next-file --\npath: stories/chronos-fracture/request/objective.md\ncontent:\n# Objective\n\n## Purpose\n\nRecord the requested story bible scope.\n\nChronos Fracture is a science-fiction story bible, not a manuscript draft.",
    )?;

    assert!(output.contains("files_written=2"));
    assert!(workspace
        .join("stories/chronos-fracture/catalog.toml")
        .is_file());
    assert!(workspace
        .join("stories/chronos-fracture/request/objective.md")
        .is_file());
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
