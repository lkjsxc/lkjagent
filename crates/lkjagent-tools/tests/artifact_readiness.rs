mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn story_audit_requires_named_semantic_content() -> TestResult<()> {
    let workspace = temp_workspace("story-readiness-missing")?;
    seed_story_root(&workspace, generic_story_text())?;

    let output = run_audit(&workspace)?;

    assert!(output.contains("artifact audit failed"));
    assert!(output.contains("readiness=missing-semantic-content"));
    assert!(output.contains("story_semantic_missing"));
    assert!(output.contains("premise"));
    assert!(output.contains("candidate_action=artifact.next"));
    Ok(())
}

#[test]
fn story_audit_passes_when_semantic_requirements_exist() -> TestResult<()> {
    let workspace = temp_workspace("story-readiness-pass")?;
    seed_story_root(&workspace, semantic_story_text())?;

    let output = run_audit(&workspace)?;

    assert!(output.contains("artifact audit passed"));
    assert!(output.contains("readiness=story-semantic-content"));
    assert!(!output.contains("story_semantic_missing"));
    Ok(())
}

fn run_audit(workspace: &Path) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/chronos-fracture"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}

fn seed_story_root(workspace: &Path, content: &str) -> TestResult<()> {
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("README.md"),
        "# Chronos Fracture\n\n## Purpose\n\nNavigate the story bible.\n\n- [catalog](catalog.toml)\n- [content](content.md)\n",
    )?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"NarrativeManuscript\"\n",
    )?;
    fs::write(root.join("content.md"), content)?;
    Ok(())
}

fn generic_story_text() -> &'static str {
    "# Chronos Notes\n\n## Purpose\n\nThis page gives concrete narrative reference material with named characters, stakes, locations, motives, consequences, decisions, discoveries, constraints, dangers, emotional turns, operational limits, historical pressure, cultural texture, and verification notes for a developing science fiction project without using the required readiness labels.\n"
}

fn semantic_story_text() -> &'static str {
    "# Chronos Readiness\n\n## Purpose\n\nPremise, timeline, cosmology, technology rules, locations, society, factions, protagonist, antagonist, supporting cast, relationship matrix, logline, themes, conflict lattice, act structure, chapter spine, continuity rules, and completion evidence are each described with concrete story details, motives, constraints, causality, verification notes, and cross references for the Chronos Fracture story bible.\n"
}
