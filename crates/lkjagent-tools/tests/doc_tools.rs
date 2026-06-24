mod support;

use lkjagent_tools::dispatch::dispatch;
use std::fs;
use std::path::Path;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn doc_scaffold_project_docs_uses_semantic_paths() -> TestResult<()> {
    let workspace = temp_workspace("doc-semantic")?;
    let output = scaffold(&workspace, &[("root", "docs"), ("title", "Project Docs")])?;

    assert!(output.contains("profile=ProjectDocs"));
    assert!(workspace.join("docs/README.md").is_file());
    assert!(workspace.join("docs/catalog.toml").is_file());
    assert!(workspace.join("docs/architecture/runtime.md").is_file());
    assert!(workspace.join("docs/operations/verification.md").is_file());
    assert_no_serial_files(&workspace.join("docs"))?;
    assert_readmes(&workspace.join("docs"))?;
    Ok(())
}

#[test]
fn doc_scaffold_exact_count_uses_semantic_roles() -> TestResult<()> {
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

    assert!(output.contains("files=30"));
    assert_eq!(markdown_count(&workspace.join("docs"))?, 30);
    assert!(workspace.join("docs/token-ledger.md").is_file());
    assert!(!workspace.join("docs/part-001.md").exists());
    assert_no_serial_files(&workspace.join("docs"))?;
    Ok(())
}

#[test]
fn doc_scaffold_story_title_uses_manuscript_paths() -> TestResult<()> {
    let workspace = temp_workspace("doc-story")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "stories"),
            ("title", "Long SF Story"),
            ("kind", "documentation"),
        ],
    )?;

    let root = workspace.join("stories");
    assert!(output.contains("profile=NarrativeManuscript"));
    assert!(root.join("project/premise.md").is_file());
    assert!(root.join("characters/protagonist.md").is_file());
    assert!(root.join("manuscript/draft-boundary.md").is_file());
    Ok(())
}

#[test]
fn doc_scaffold_bread_cookbook_uses_recipe_paths() -> TestResult<()> {
    let workspace = temp_workspace("doc-cookbook")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "cookbooks/bread-cookbook"),
            ("title", "Bread Cookbook"),
            ("kind", "content-artifact"),
        ],
    )?;

    assert!(output.contains("profile=BreadCookbook"));
    assert!(workspace
        .join("cookbooks/bread-cookbook/foundations/flour-water-salt-yeast.md")
        .is_file());
    assert!(workspace
        .join("cookbooks/bread-cookbook/recipes/sourdough-country-loaf.md")
        .is_file());
    assert!(!workspace
        .join("cookbooks/bread-cookbook/part-001.md")
        .exists());
    assert_no_serial_files(&workspace.join("cookbooks/bread-cookbook"))?;
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
    scaffold(
        &workspace,
        &[
            ("root", "guide"),
            ("title", "Guide"),
            ("count", "3"),
            ("mode", "exact"),
        ],
    )?;

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
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut state = state();
    Ok(dispatch(
        &action("doc.scaffold", params),
        &runtime,
        &mut conn,
        &mut state,
    )
    .content)
}

fn audit(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut state = state();
    Ok(dispatch(
        &action("doc.audit", params),
        &runtime,
        &mut conn,
        &mut state,
    )
    .content)
}

fn assert_readmes(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            assert!(
                path.join("README.md").is_file(),
                "missing README in {path:?}"
            );
            assert_readmes(&path)?;
        }
    }
    Ok(())
}

fn assert_no_serial_files(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            assert_no_serial_files(&path)?;
        } else {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            assert!(!name.starts_with("part-"), "serial file {name}");
        }
    }
    Ok(())
}

fn markdown_count(root: &Path) -> TestResult<usize> {
    let mut count: usize = 0;
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            count = count.saturating_add(markdown_count(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
