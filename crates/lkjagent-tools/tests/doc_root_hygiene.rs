mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn audit_reports_markdown_suffix_directory() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-md-dir")?;
    let root = workspace.join("docs");
    fs::create_dir_all(root.join("bad.md"))?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    fs::write(root.join("README.md"), readme("Docs", "bad.md/README.md"))?;
    fs::write(root.join("bad.md/README.md"), readme("Bad", "leaf.md"))?;
    fs::write(root.join("bad.md/leaf.md"), structure_only("Leaf"))?;

    let audit = run_tool(&workspace, "doc.audit", &[("root", "docs")])?;

    assert!(audit.contains("path_hygiene=failed"));
    assert!(audit.contains("markdown_suffix_directory: bad.md"));
    Ok(())
}

#[test]
fn removed_scaffold_does_not_create_combined_multi_topic_filename() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-topic-combined")?;
    let output = scaffold_multi_topic(&workspace)?;

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
fn slug_truncation_keeps_stem_under_limit() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-slug-limit")?;
    let title = "Alpha Beta Gamma Delta Epsilon Zeta Eta Theta Iota Kappa Lambda Mu";
    let output = run_tool(
        &workspace,
        "doc.scaffold",
        &[("root", "docs"), ("title", title)],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs").exists());
    Ok(())
}

#[test]
fn removed_scaffold_creates_no_topic_pages() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-topic-pages")?;
    let output = scaffold_multi_topic(&workspace)?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs/topics/model-endpoint.md").exists());
    Ok(())
}

#[test]
fn structure_only_pages_fail_content_readiness() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-structure-only")?;
    let root = workspace.join("guide");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    fs::write(root.join("README.md"), readme("Guide", "a.md"))?;
    fs::write(root.join("a.md"), structure_only("A"))?;

    let audit = run_tool(
        &workspace,
        "doc.audit",
        &[("root", "guide"), ("count", "3")],
    )?;

    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content"));
    Ok(())
}

fn scaffold_multi_topic(workspace: &Path) -> TestResult<String> {
    run_tool(
        workspace,
        "doc.scaffold",
        &[
            ("root", "docs"),
            (
                "title",
                "Model Endpoint, Minecraft, Windows, Japan, United States",
            ),
        ],
    )
}

fn readme(title: &str, child: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nDocs.\n\n## Table of Contents\n\n- [{child}]({child}): child.\n"
    )
}

fn structure_only(title: &str) -> String {
    format!("# {title}\n\n## Purpose\n\ncontent_state=structure-only\n")
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
