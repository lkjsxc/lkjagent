mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const PLAN: &str = "<action>
<tool>graph.plan</tool>
<objective>Finish Chronos Fracture story bible</objective>
<steps>audit story content; record readiness; close with evidence</steps>
<checks>artifact audit passes semantic readiness before completion</checks>
<paths>stories/chronos-fracture</paths>
<reason>story completion requires artifact readiness evidence</reason>
</action>";
#[test]
fn chronos_story_replay_records_deterministic_artifact_repair() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "create story bible named \"Chronos Fracture\"",
        "owner",
        "101",
    )?;
    let workspace = temp_workspace("chronos-story-replay")?;
    seed_story(&workspace)?;
    let server = serve_responses(vec![completion(PLAN)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.runtime.model_log_path = Some(workspace.join("current-model-run.md"));

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    server.join()?;

    assert_eq!(
        state::get(&conn, "open task")?,
        Some("create story bible named \"Chronos Fracture\"".to_string())
    );
    assert!(workspace
        .join("stories/chronos-fracture/catalog.toml")
        .exists());
    assert!(!has_part_file(&workspace.join("stories/chronos-fracture"))?);
    let log = fs::read_to_string(workspace.join("current-model-run.md"))?;
    assert!(log.contains("Model Run Log"));
    assert!(log.contains("<tool>artifact.audit</tool>"));
    assert!(log.contains("<tool>artifact.next</tool>"));
    assert!(log.contains("missing_root"));
    Ok(())
}

fn seed_story(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"NarrativeManuscript\"\n",
    )?;
    let files = role_files();
    let links = files
        .iter()
        .map(|(path, _, _)| format!("- [{path}]({path})"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        root.join("README.md"),
        format!("# Chronos Fracture\n\n## Purpose\n\nNavigate the story bible.\n\n{links}\n"),
    )?;
    for (path, title, signals) in files {
        fs::write(root.join(path), role_text(title, signals))?;
    }
    Ok(())
}

fn role_text(title: &str, signals: &str) -> String {
    format!(
        "# {title}\n\n## {title}\n\n{signals}. This Chronos Fracture page records concrete story facts, named constraints, causal consequences, cross references, verification notes, continuity checks, character pressure, setting texture, and scene-ready decisions without placeholder prose.\n"
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

fn has_part_file(path: &Path) -> TestResult<bool> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().contains("part") {
            return Ok(true);
        }
    }
    Ok(false)
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
