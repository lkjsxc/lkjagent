mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn batch_write_line_protocol_still_works() -> TestResult<()> {
    let workspace = temp_workspace("batch-line")?;
    let output = run_batch(
        &workspace,
        "path: docs/a.md\ncontent:\n# A\n\nConcrete content.",
    )?;

    assert!(output.contains("files_written=1"));
    assert!(output.contains("input_format=line-protocol"));
    assert!(workspace.join("docs/a.md").is_file());
    Ok(())
}

#[test]
fn batch_write_json_array_inside_files_works() -> TestResult<()> {
    let workspace = temp_workspace("batch-json-array")?;
    let output = run_batch(
        &workspace,
        r##"[{"path":"docs/a.md","content":"# A\n\nConcrete content."}]"##,
    )?;

    assert!(output.contains("input_format=json-array"));
    assert!(workspace.join("docs/a.md").is_file());
    Ok(())
}

#[test]
fn batch_write_json_object_files_array_works() -> TestResult<()> {
    let workspace = temp_workspace("batch-json-object")?;
    let output = run_batch(
        &workspace,
        r##"{"files":[{"path":"docs/a.md","content":"# A\n\nConcrete content."}]}"##,
    )?;

    assert!(output.contains("input_format=json-object-files"));
    assert!(workspace.join("docs/a.md").is_file());
    Ok(())
}

#[test]
fn batch_write_path_shaped_unknown_params_normalize() -> TestResult<()> {
    let workspace = temp_workspace("batch-path-params")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[(
                "stories/chronos-fracture/catalog.toml",
                "[artifact]\nname = \"Chronos Fracture\"",
            )],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("path-shaped-params->files"));
    assert!(output.content.contains("files_written=1"));
    assert!(workspace
        .join("stories/chronos-fracture/catalog.toml")
        .is_file());
    Ok(())
}

#[test]
fn batch_write_absolute_path_param_refuses_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-absolute-path-param")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[("/tmp/bad.md", "# Bad\n\nConcrete content.")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("missing=files"));
    assert!(!workspace.join("tmp/bad.md").exists());
    Ok(())
}

#[test]
fn batch_write_duplicate_path_params_refuse_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-duplicate-path-params")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[
                ("docs/a.md", "# A\n\nConcrete content."),
                ("docs/a.md", "# A2\n\nConcrete content."),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("duplicate=docs/a.md"));
    assert!(!workspace.join("docs/a.md").exists());
    Ok(())
}

#[test]
fn batch_write_empty_path_param_content_refuses_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-empty-path-param")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action("fs.batch_write", &[("docs/a.md", "")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("missing=files"));
    assert!(!workspace.join("docs/a.md").exists());
    Ok(())
}

#[test]
fn batch_write_json_missing_path_fails_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-json-missing-path")?;
    let output = run_batch(
        &workspace,
        r##"[{"title":"A","content":"# A\n\nConcrete content."}]"##,
    )?;

    assert!(output.contains("each JSON file needs path and content"));
    assert!(!workspace.join("docs/a.md").exists());
    Ok(())
}

#[test]
fn batch_write_oversized_file_fails_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-oversize")?;
    let content = format!("path: docs/a.md\ncontent:\n# A\n\n{}", "x".repeat(1900));
    let output = run_batch(&workspace, &content)?;

    assert!(output.contains("payload too large for fs.batch_write file"));
    assert!(!workspace.join("docs/a.md").exists());
    Ok(())
}

#[test]
fn batch_write_duplicate_paths_fail_before_mutation() -> TestResult<()> {
    let workspace = temp_workspace("batch-duplicates")?;
    let files = "path: docs/a.md\ncontent:\n# A\n\nConcrete content.\n-- lkjagent-next-file --\npath: docs/a.md\ncontent:\n# A2\n\nConcrete content.";
    let output = run_batch(&workspace, files)?;

    assert!(output.contains("duplicate path: docs/a.md"));
    assert!(!workspace.join("docs/a.md").exists());
    Ok(())
}

fn run_batch(workspace: &Path, files: &str) -> TestResult<String> {
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

#[allow(dead_code)]
fn read(path: &Path) -> TestResult<String> {
    Ok(fs::read_to_string(path)?)
}
