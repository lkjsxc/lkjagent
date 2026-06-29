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
    assert!(output.contains("story_scale_missing"));
    assert!(output.contains("profile-scale-content-groups"));
    assert!(output.contains("premise"));
    assert!(output.contains("candidate_action=artifact.next"));
    Ok(())
}

#[test]
fn story_audit_resolves_kind_from_root_before_ledger_readiness() -> TestResult<()> {
    let workspace = temp_workspace("story-kind-resolution")?;
    seed_story_root(&workspace, generic_story_text())?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action("artifact.audit", &[("root", "stories/chronos-fracture")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    let ledger = lkjagent_store::artifact_ledger::latest_for_case(&conn, 0)?
        .ok_or("missing artifact ledger")?;

    assert!(output.contains("artifact audit failed"), "{output}");
    assert!(output.contains("story_semantic_missing"), "{output}");
    assert_eq!(ledger.kind, "story");
    assert_eq!(ledger.readiness_status, "failed");
    Ok(())
}

#[test]
fn story_audit_passes_when_semantic_requirements_exist() -> TestResult<()> {
    let workspace = temp_workspace("story-readiness-pass")?;
    seed_semantic_story_root(&workspace)?;

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
    fs::write(root.join("README.md"), root_readme(&["content.md"]))?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"NarrativeManuscript\"\n",
    )?;
    fs::write(root.join("content.md"), content)?;
    Ok(())
}

fn seed_semantic_story_root(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    let files = role_files();
    let names = files.iter().map(|(path, _, _)| *path).collect::<Vec<_>>();
    fs::write(root.join("README.md"), root_readme(&names))?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"NarrativeManuscript\"\n",
    )?;
    for (path, title, signals) in files {
        fs::write(root.join(path), role_text(title, signals))?;
    }
    Ok(())
}

fn root_readme(files: &[&str]) -> String {
    let links = files
        .iter()
        .map(|file| format!("- [{file}]({file})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("# Chronos Fracture\n\n## Purpose\n\nNavigate the story bible.\n\n- [catalog](catalog.toml)\n{links}\n")
}

fn generic_story_text() -> &'static str {
    "# Chronos Notes\n\n## Purpose\n\nThis page gives concrete narrative reference material with named characters, stakes, locations, motives, consequences, decisions, discoveries, constraints, dangers, emotional turns, operational limits, historical pressure, cultural texture, and verification notes for a developing science fiction project without using the required readiness labels.\n"
}

fn role_text(title: &str, signals: &str) -> String {
    format!(
        "# {title}\n\n## {title}\n\n{signals}. This role records concrete Chronos Fracture story facts, named constraints, causal consequences, cross references, verification notes, and continuity checks that guide later scenes without acting as a placeholder.\n"
    )
}

fn role_files() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("premise.md", "Premise", "premise stakes inciting story"),
        ("timeline.md", "Timeline", "timeline sequence past future"),
        (
            "cosmology.md",
            "Cosmology",
            "cosmology universe physics rule",
        ),
        (
            "technology-rules.md",
            "Technology Rules",
            "technology rule limit cost",
        ),
        ("locations.md", "Locations", "location place district route"),
        ("society.md", "Society", "society culture law class"),
        ("factions.md", "Factions", "faction agenda rival alliance"),
        (
            "protagonist.md",
            "Protagonist",
            "protagonist goal flaw choice",
        ),
        (
            "antagonist.md",
            "Antagonist",
            "antagonist pressure motive threat",
        ),
        (
            "supporting-cast.md",
            "Supporting Cast",
            "supporting ally mentor rival",
        ),
        (
            "relationships.md",
            "Relationships",
            "relationship trust conflict bond",
        ),
        ("logline.md", "Logline", "logline must before stakes"),
        ("themes.md", "Themes", "theme cost memory identity"),
        (
            "conflict-lattice.md",
            "Conflict Lattice",
            "conflict escalation pressure choice",
        ),
        (
            "act-structure.md",
            "Act Structure",
            "act turning point reversal climax",
        ),
        (
            "chapter-spine.md",
            "Chapter Spine",
            "chapter scene reveal consequence",
        ),
        (
            "rules.md",
            "Continuity Rules",
            "continuity rule contradiction check",
        ),
        (
            "readiness-audit.md",
            "Completion Evidence",
            "completion evidence verified audit",
        ),
    ]
}
