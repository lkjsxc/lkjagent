mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_scaffold_does_not_create_combined_slug() -> TestResult<()> {
    let workspace = temp_workspace("doc-path-topics")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "docs"),
            (
                "title",
                "Model Endpoint, Minecraft, Windows, Japan, United States",
            ),
        ],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace
        .join("docs/model-endpoint-minecraft-windows-japan-united-states.md")
        .exists());
    assert!(!workspace
        .join("docs/qwen-minecraft-windows-japan-united-states.md")
        .exists());
    Ok(())
}

#[test]
fn audit_reports_long_and_multi_topic_paths() -> TestResult<()> {
    let workspace = temp_workspace("doc-path-audit")?;
    let root = workspace.join("docs");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    let long_name = "model-endpoint-minecraft-windows-japan-united-states-extra.md";
    fs::write(
        root.join("README.md"),
        format!(
            "# Docs\n\n## Purpose\n\nDocs.\n\n## Table of Contents\n\n- [{long_name}]({long_name}): topic.\n- [terms.md](terms.md): terms.\n- [catalog.toml](catalog.toml): catalog.\n"
        ),
    )?;
    fs::write(
        root.join(long_name),
        "# Topic\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;
    fs::write(
        root.join("terms.md"),
        "# Terms\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;

    let audit = audit(&workspace, "docs")?;

    assert!(audit.contains("path_hygiene=failed"));
    assert!(audit.contains("markdown_stem_too_long"));
    assert!(audit.contains("multi_topic_slug"));
    Ok(())
}

fn scaffold(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    run_tool(workspace, "doc.scaffold", params)
}

fn audit(workspace: &Path, root: &str) -> TestResult<String> {
    run_tool(workspace, "doc.audit", &[("root", root)])
}

fn run_tool(workspace: &Path, tool: &str, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action(tool, params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}
