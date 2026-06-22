mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn fs_write_rejects_scaffold_phrase_without_overwrite() -> TestResult<()> {
    let workspace = temp_workspace("fs-write-placeholder")?;
    let existing = "# Recipe\n\nActual ingredients, method, timing, and notes.\n";
    fs::create_dir_all(workspace.join("cookbooks"))?;
    fs::write(workspace.join("cookbooks/bread.md"), existing)?;
    let output = dispatch_action(
        &workspace,
        "fs.write",
        &[
            ("path", "cookbooks/bread.md"),
            (
                "content",
                "Replace this skeleton with real cookbook content before dispatch.",
            ),
        ],
    )?;

    assert!(is_error(&output));
    assert!(output.content.contains("scaffold phrase"));
    assert_eq!(
        fs::read_to_string(workspace.join("cookbooks/bread.md"))?,
        existing
    );
    Ok(())
}

#[test]
fn fs_batch_write_rejects_placeholder_before_any_write() -> TestResult<()> {
    let workspace = temp_workspace("fs-batch-placeholder")?;
    fs::create_dir_all(workspace.join("out"))?;
    fs::write(workspace.join("out/one.md"), "# One\n\nExisting content.\n")?;
    let files = "path: out/one.md\ncontent:\nAdd the requested substance, details, and verification notes here.\n-- lkjagent-next-file --\npath: out/two.md\ncontent:\n# Two\n\nActual content.\n";

    let output = dispatch_action(&workspace, "fs.batch_write", &[("files", files)])?;

    assert!(is_error(&output));
    assert!(output.content.contains("scaffold phrase"));
    assert!(fs::read_to_string(workspace.join("out/one.md"))?.contains("Existing"));
    assert!(!workspace.join("out/two.md").exists());
    Ok(())
}

#[test]
fn fs_write_rejects_generic_coming_soon_placeholder() -> TestResult<()> {
    let workspace = temp_workspace("fs-write-coming-soon")?;
    let output = dispatch_action(
        &workspace,
        "fs.write",
        &[
            ("path", "docs/status.md"),
            (
                "content",
                "# Status\n\nComing soon. This section describes the feature.",
            ),
        ],
    )?;

    assert!(is_error(&output));
    assert!(output.content.contains("scaffold phrase"));
    assert!(!workspace.join("docs/status.md").exists());
    Ok(())
}

#[test]
fn fs_batch_write_rejects_generic_record_prose_atomically() -> TestResult<()> {
    let workspace = temp_workspace("fs-batch-generic-record")?;
    let files = "path: out/one.md\ncontent:\nThis file records the generated documentation tree.\n-- lkjagent-next-file --\npath: out/two.md\ncontent:\n# Two\n\nActual content.\n";

    let output = dispatch_action(&workspace, "fs.batch_write", &[("files", files)])?;

    assert!(is_error(&output));
    assert!(output.content.contains("scaffold phrase"));
    assert!(!workspace.join("out/one.md").exists());
    assert!(!workspace.join("out/two.md").exists());
    Ok(())
}

#[test]
fn fs_batch_write_validates_escape_before_any_write() -> TestResult<()> {
    let workspace = temp_workspace("fs-batch-escape-atomic")?;
    let files = "path: out/one.md\ncontent:\n# One\n-- lkjagent-next-file --\npath: ../two.md\ncontent:\n# Two\n";

    let output = dispatch_action(&workspace, "fs.batch_write", &[("files", files)])?;

    assert!(is_error(&output));
    assert!(output.content.contains("workspace"));
    assert!(!workspace.join("out/one.md").exists());
    Ok(())
}

#[test]
fn fs_batch_write_rejects_large_file_before_any_write() -> TestResult<()> {
    let workspace = temp_workspace("fs-batch-large-file")?;
    let large = "x".repeat(65_537);
    let files = format!(
        "path: out/one.md\ncontent:\n# One\n-- lkjagent-next-file --\npath: out/two.md\ncontent:\n{large}\n"
    );

    let output = dispatch_action(&workspace, "fs.batch_write", &[("files", &files)])?;

    assert!(is_error(&output));
    assert!(output.content.contains("file too large"));
    assert!(!workspace.join("out/one.md").exists());
    Ok(())
}

fn dispatch_action(
    workspace: &std::path::Path,
    tool: &str,
    params: &[(&str, &str)],
) -> TestResult<lkjagent_tools::dispatch::DispatchOutput> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    Ok(dispatch(
        &action(tool, params),
        &runtime,
        &mut conn,
        &mut state(),
    ))
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
