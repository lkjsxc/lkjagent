mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const WRITE_ACTION: &str = "<action>
<tool>fs.write</tool>
<path>notes.md</path>
<content># Notes</content>
</action>";
const PLAN_ACTION: &str = "<action>
<tool>graph.plan</tool>
<objective>start long work</objective>
<steps>write notes; read notes; record verification</steps>
<checks>fs.read notes.md confirms content</checks>
<paths>notes.md</paths>
<reason>mutation requires a graph plan</reason>
</action>";
const DONE_ACTION: &str = "<action>
<tool>agent.done</tool>
<summary>three markdown files complete</summary>
</action>";
const READ_ACTION: &str = "<action>
<tool>fs.read</tool>
<path>notes.md</path>
</action>";
const EVIDENCE_ACTION: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed notes.md content</summary>
<path>notes.md</path>
</action>";

#[test]
fn owner_guidance_during_open_task_persists_count_guard() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "start long work", "owner-send", "101")?;
    let workspace = temp_workspace("owner-guidance")?;
    let server = serve_responses(vec![
        completion(PLAN_ACTION),
        completion(WRITE_ACTION),
        completion(READ_ACTION),
        completion(EVIDENCE_ACTION),
        completion(DONE_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    queue::enqueue(
        &mut conn,
        "Finish with exactly 3 markdown files in docs.",
        "owner-send",
        "102",
    )?;
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(fs::read_to_string(workspace.join("notes.md"))?, "# Notes");
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    server.join()?;

    assert_eq!(
        state::get(&conn, "completion guard")?,
        Some("markdown-count:3".to_string())
    );
    assert!(events::read_events(&conn)?
        .iter()
        .any(|event| event.content.contains("need exactly 3 markdown files")));
    Ok(())
}

#[test]
fn benchmark_docs_task_auto_scaffolds_exact_markdown_count() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "Create a benchmark documentation corpus with exactly 12 markdown files.",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("benchmark-guidance")?;
    let server = serve_responses(vec![completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    server.join()?;

    let root = workspace.join("docs/benchmark-corpus");
    assert_eq!(markdown_count(&root)?, 12);
    assert_eq!(other_count(&root)?, 0);
    assert_eq!(state::get(&conn, "completion guard")?, None);
    Ok(())
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

fn markdown_count(path: &Path) -> TestResult<usize> {
    count_with(path, true)
}

fn other_count(path: &Path) -> TestResult<usize> {
    count_with(path, false)
}

fn count_with(path: &Path, markdown: bool) -> TestResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(count_with(&child, markdown)?);
        } else if child.extension().is_some_and(|extension| extension == "md") == markdown {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
