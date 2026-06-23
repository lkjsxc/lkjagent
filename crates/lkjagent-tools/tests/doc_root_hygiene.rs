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
fn scaffold_does_not_create_combined_multi_topic_filename() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-topic-combined")?;
    scaffold_multi_topic(&workspace)?;

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
    run_tool(
        &workspace,
        "doc.scaffold",
        &[("root", "docs"), ("title", title)],
    )?;

    for stem in markdown_stems(&workspace.join("docs"))? {
        assert!(stem.len() <= 48, "long stem {stem}");
    }
    Ok(())
}

#[test]
fn generic_seed_topics_are_separate_pages() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-topic-pages")?;
    scaffold_multi_topic(&workspace)?;

    for path in [
        "topics/model-endpoint.md",
        "topics/minecraft.md",
        "topics/windows.md",
        "topics/japan.md",
        "topics/united-states.md",
    ] {
        assert!(
            workspace.join("docs").join(path).is_file(),
            "missing {path}"
        );
    }
    Ok(())
}

#[test]
fn structure_only_pages_fail_content_readiness() -> TestResult<()> {
    let workspace = temp_workspace("doc-root-structure-only")?;
    run_tool(
        &workspace,
        "doc.scaffold",
        &[("root", "guide"), ("title", "Guide"), ("count", "3")],
    )?;

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

fn markdown_stems(root: &Path) -> TestResult<Vec<String>> {
    let mut stems = Vec::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            stems.extend(markdown_stems(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                stems.push(stem.to_string());
            }
        }
    }
    Ok(stems)
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
