mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn content_artifact_audit_rejects_structure_only_story() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story")?;
    seed_story(&workspace, "stories/long-sf-story", false)?;

    let audit = audit(&workspace, "stories/long-sf-story")?;

    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content"));
    Ok(())
}

#[test]
fn content_artifact_audit_passes_content_bearing_story() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story-pass")?;
    seed_story(&workspace, "stories/long-sf-story", true)?;

    let audit = audit(&workspace, "stories/long-sf-story")?;

    assert!(audit.contains("document audit passed"));
    assert!(!audit.contains("scaffold_only_content"));
    Ok(())
}

#[test]
fn content_artifact_audit_accepts_concise_reference_story_pages() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story-concise")?;
    seed_story(&workspace, "stories/concise-sf-story", true)?;

    let audit = audit(&workspace, "stories/concise-sf-story")?;

    assert!(audit.contains("document audit passed"));
    Ok(())
}

#[test]
fn story_artifact_audit_ignores_readme_link_and_purpose_topology() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-story-links")?;
    let root = workspace.join("stories/story-links");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("README.md"),
        "# Story Links\n\nRecord story facts.\n",
    )?;
    fs::write(root.join("premise.md"), story_text("Premise"))?;
    fs::write(root.join("timeline.md"), story_text("Timeline"))?;

    let audit = audit(&workspace, "stories/story-links")?;

    assert!(audit.contains("document audit passed"));
    assert!(!audit.contains("missing_readme_link"));
    assert!(!audit.contains("missing_purpose"));
    Ok(())
}

#[test]
fn project_doc_audit_rejects_generated_scaffold() -> TestResult<()> {
    let workspace = temp_workspace("doc-content-project")?;
    let root = workspace.join("docs");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    fs::write(
        root.join("README.md"),
        readme("Docs", &["a.md", "b.md", "catalog.toml"]),
    )?;
    fs::write(root.join("a.md"), old_body("A"))?;
    fs::write(root.join("b.md"), old_body("B"))?;

    let audit = audit(&workspace, "docs")?;

    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("structure_only_content") || audit.contains("scaffold_only_content"));
    Ok(())
}

fn seed_story(workspace: &Path, root: &str, strong: bool) -> TestResult<()> {
    let root = workspace.join(root);
    fs::create_dir_all(root.join("setting"))?;
    fs::create_dir_all(root.join("manuscript"))?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(
        root.join("README.md"),
        readme(
            "Story",
            &["setting/README.md", "manuscript/README.md", "catalog.toml"],
        ),
    )?;
    fs::write(
        root.join("setting/README.md"),
        readme("Setting", &["cosmology.md", "stations.md"]),
    )?;
    fs::write(
        root.join("manuscript/README.md"),
        readme("Manuscript", &["draft-boundary.md", "opening.md"]),
    )?;
    for path in [
        "setting/cosmology.md",
        "setting/stations.md",
        "manuscript/draft-boundary.md",
        "manuscript/opening.md",
    ] {
        fs::write(
            root.join(path),
            if strong {
                story_text(path)
            } else {
                weak_text(path)
            },
        )?;
    }
    Ok(())
}

fn readme(title: &str, links: &[&str]) -> String {
    let items = links
        .iter()
        .map(|link| format!("- [{link}]({link})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("# {title}\n\n## Purpose\n\nNavigate {title}.\n\n## Contents\n\n{items}\n")
}

fn weak_text(title: &str) -> String {
    format!("# {title}\n\n## Purpose\n\ncontent_state=structure-only\n")
}

fn story_text(title: &str) -> String {
    format!("# {title}\n\n## Reference Detail\n\nChronos Fracture records concrete story facts with temporal pressure, continuity consequences, sensory stakes, faction impact, character intent, and verification notes for audit evidence.\n")
}

fn old_body(title: &str) -> String {
    format!("# {title}\n\n## Purpose\n\nThis file records the {title} role for the generated documentation tree.\n\n## Status\n\nscaffolded\n")
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
