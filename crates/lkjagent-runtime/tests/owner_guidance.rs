mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, seed_skill_path, store, temp_workspace, TestResult};

const WRITE_ACTION: &str = "<act>
<tool>fs.write</tool>
<path>notes.md</path>
<content># Notes</content>
</act>";
const DONE_ACTION: &str = "<act>
<tool>agent.done</tool>
<summary>three markdown files complete</summary>
</act>";

#[test]
fn owner_guidance_during_open_task_persists_count_guard() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "start long work", "owner-send", "101")?;
    let workspace = temp_workspace("owner-guidance")?;
    let server = serve_responses(vec![completion(WRITE_ACTION), completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(fs::read_to_string(workspace.join("notes.md"))?, "# Notes");
    queue::enqueue(
        &mut conn,
        "Finish with exactly 3 markdown files in docs.",
        "owner-send",
        "102",
    )?;
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
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

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180),
        workspace.to_path_buf(),
        seed_skill_path(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
