mod support;

use lkjagent_tools::dispatch::dispatch;
use std::fs;
use std::path::Path;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn content_artifact_audit_rejects_scaffold_only_story() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story")?;
    scaffold(
        &workspace,
        &[
            ("root", "stories/long-sf-story"),
            ("title", "Long SF Story"),
            ("kind", "content-artifact"),
        ],
    )?;

    let audit = audit(&workspace, "stories/long-sf-story")?;

    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("failures_omitted="));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content: setting/cosmology.md"));
    assert!(audit.contains("structure_only_content: manuscript/draft-boundary.md"));
    Ok(())
}

#[test]
fn content_artifact_audit_passes_content_bearing_story() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story-pass")?;
    scaffold(
        &workspace,
        &[
            ("root", "stories/long-sf-story"),
            ("title", "Long SF Story"),
            ("kind", "content-artifact"),
        ],
    )?;
    replace_leaves(&workspace.join("stories/long-sf-story"))?;

    let audit = audit(&workspace, "stories/long-sf-story")?;

    assert!(audit.contains("document audit passed"));
    assert!(!audit.contains("scaffold_only_content"));
    Ok(())
}

#[test]
fn content_artifact_audit_accepts_concise_reference_story_pages() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story-concise")?;
    scaffold(
        &workspace,
        &[
            ("root", "stories/concise-sf-story"),
            ("title", "Concise SF Story"),
            ("kind", "content-artifact"),
        ],
    )?;
    replace_leaves_concise(&workspace.join("stories/concise-sf-story"))?;

    let audit = audit(&workspace, "stories/concise-sf-story")?;

    assert!(audit.contains("document audit passed"));
    Ok(())
}

#[test]
fn project_doc_audit_rejects_generated_scaffold() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-project")?;
    scaffold(&workspace, &[("root", "docs"), ("title", "Project Docs")])?;

    let audit = audit(&workspace, "docs")?;

    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content"));
    Ok(())
}

fn replace_leaves(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            replace_leaves(&path)?;
        } else if path.extension().is_some_and(|ext| ext == "md")
            && path.file_name().and_then(|name| name.to_str()) != Some("README.md")
        {
            let title = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("section");
            fs::write(
                &path,
                format!(
                    "# {title}\n\n## Content\n\nThis section contains concrete scene material, named continuity details, sensory description, causal decisions, and verification notes. It names character intent, conflict, setting texture, consequence, and revision evidence so the manuscript can be audited as an actual content artifact instead of a generated scaffold marker.\n"
                ),
            )?;
        }
    }
    Ok(())
}

fn replace_leaves_concise(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            replace_leaves_concise(&path)?;
        } else if path.extension().is_some_and(|ext| ext == "md")
            && path.file_name().and_then(|name| name.to_str()) != Some("README.md")
        {
            let title = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("section");
            fs::write(
                &path,
                format!(
                    "# {title}\n\n## Reference Detail\n\nChronos Fracture records a concrete story bible fact with named temporal pressure, continuity consequences, sensory stakes, faction impact, character intent, and verification notes for audit evidence.\n"
                ),
            )?;
        }
    }
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
    let mut state = state();
    Ok(dispatch(&action(tool, params), &runtime, &mut conn, &mut state).content)
}
