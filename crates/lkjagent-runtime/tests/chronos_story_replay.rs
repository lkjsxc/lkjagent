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
const AUDIT: &str = "<action>
<tool>artifact.audit</tool>
<root>stories/chronos-fracture</root>
<kind>story</kind>
</action>";
const EVIDENCE: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>artifact.audit passed semantic readiness for Chronos Fracture</summary>
<path>stories/chronos-fracture</path>
</action>";
const DONE: &str = "<action>
<tool>agent.done</tool>
<summary>Chronos Fracture story bible passed artifact readiness</summary>
</action>";

#[test]
fn chronos_story_replay_closes_with_artifact_readiness() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "create Chronos Fracture story bible",
        "owner",
        "101",
    )?;
    let workspace = temp_workspace("chronos-story-replay")?;
    seed_story(&workspace)?;
    let server = serve_responses(vec![
        completion(PLAN),
        completion(AUDIT),
        completion(EVIDENCE),
        completion(DONE),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.runtime.model_log_path = Some(workspace.join("current-model-run.md"));

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    assert!(workspace
        .join("stories/chronos-fracture/catalog.toml")
        .exists());
    assert!(!has_part_file(&workspace.join("stories/chronos-fracture"))?);
    let log = fs::read_to_string(workspace.join("current-model-run.md"))?;
    assert!(log.contains("Model Run Log"));
    Ok(())
}

fn seed_story(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"NarrativeManuscript\"\n",
    )?;
    fs::write(
        root.join("README.md"),
        "# Chronos Fracture\n\n## Purpose\n\nNavigate the story bible.\n\n- [content](content.md)\n",
    )?;
    fs::write(root.join("content.md"), semantic_story_text())?;
    Ok(())
}

fn semantic_story_text() -> &'static str {
    "# Chronos Readiness\n\n## Purpose\n\nPremise, timeline, cosmology, technology rules, locations, society, factions, protagonist, antagonist, supporting cast, relationship matrix, logline, themes, conflict lattice, act structure, chapter spine, continuity rules, and completion evidence are each described with concrete story details, motives, constraints, causality, verification notes, and cross references for the Chronos Fracture story bible.\n"
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
