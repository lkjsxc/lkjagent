mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_doc_scaffold_writes_no_project_docs() -> TestResult<()> {
    let workspace = temp_workspace("doc-semantic")?;
    let output = scaffold(&workspace, &[("root", "docs"), ("title", "Project Docs")])?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs/README.md").exists());
    assert!(!workspace.join("docs/architecture/runtime.md").exists());
    Ok(())
}

#[test]
fn removed_doc_scaffold_writes_no_exact_count_tree() -> TestResult<()> {
    let workspace = temp_workspace("doc-count")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "docs"),
            ("title", "Thirty Docs"),
            ("count", "30"),
            ("mode", "exact"),
        ],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs/token-ledger.md").exists());
    assert!(!workspace.join("docs/part-001.md").exists());
    Ok(())
}

#[test]
fn removed_doc_scaffold_writes_no_story_or_cookbook_profiles() -> TestResult<()> {
    let workspace = temp_workspace("doc-profiles")?;
    let story = scaffold(
        &workspace,
        &[
            ("root", "stories"),
            ("title", "Long SF Story"),
            ("kind", "documentation"),
        ],
    )?;
    let cookbook = scaffold(
        &workspace,
        &[
            ("root", "cookbooks/bread-cookbook"),
            ("title", "Bread Cookbook"),
            ("kind", "content-artifact"),
        ],
    )?;

    assert!(story.contains("unknown tool: doc.scaffold"));
    assert!(cookbook.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("stories/project/premise.md").exists());
    assert!(!workspace
        .join("cookbooks/bread-cookbook/recipes/sourdough-country-loaf.md")
        .exists());
    Ok(())
}

#[test]
fn doc_audit_rejects_part_files_and_missing_links() -> TestResult<()> {
    let workspace = temp_workspace("doc-audit")?;
    fs::create_dir_all(workspace.join("docs"))?;
    fs::write(
        workspace.join("docs/README.md"),
        "# Docs\n\n## Purpose\n\nBroken docs.\n\n## Table of Contents\n\n",
    )?;
    fs::write(
        workspace.join("docs/part-001.md"),
        "# Bad\n\n## Purpose\n\nBad.\n",
    )?;

    let audit = audit(&workspace, &[("root", "docs")])?;
    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("serial_filename: part-001.md"));
    assert!(audit.contains("missing_readme_link: part-001.md"));
    Ok(())
}

#[test]
fn doc_audit_rejects_structure_only_generated_content() -> TestResult<()> {
    let workspace = temp_workspace("doc-audit-pass")?;
    let root = workspace.join("guide");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    fs::write(
        root.join("README.md"),
        readme("Guide", &["a.md", "b.md", "catalog.toml"]),
    )?;
    fs::write(root.join("a.md"), structure_only("A"))?;
    fs::write(root.join("b.md"), structure_only("B"))?;

    let audit = audit(
        &workspace,
        &[("root", "guide"), ("count", "3"), ("mode", "exact")],
    )?;
    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content"));
    Ok(())
}

fn scaffold(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    run(workspace, "doc.scaffold", params)
}

fn audit(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    run(workspace, "doc.audit", params)
}

fn run(workspace: &Path, tool: &str, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut state = state();
    Ok(dispatch(&action(tool, params), &runtime, &mut conn, &mut state).content)
}

fn readme(title: &str, links: &[&str]) -> String {
    let items = links
        .iter()
        .map(|link| format!("- [{link}]({link})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("# {title}\n\n## Purpose\n\n{title}.\n\n## Table of Contents\n\n{items}\n")
}

fn structure_only(title: &str) -> String {
    format!("# {title}\n\n## Purpose\n\ncontent_state=structure-only\n")
}
