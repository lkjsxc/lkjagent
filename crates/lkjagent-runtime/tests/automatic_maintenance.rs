mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, seed_skill_path, store, temp_workspace, TestResult};

const MAINT_DONE: &str = "<act>
<tool>agent.done</tool>
<summary>maintenance cycle checked current state</summary>
</act>";
const WRITE_ACTION: &str = "<act>
<tool>fs.write</tool>
<path>owner.txt</path>
<content>owner wins</content>
</act>";
const DONE_ACTION: &str = "<act>
<tool>agent.done</tool>
<summary>owner task complete</summary>
</act>";

#[test]
fn idle_daemon_runs_maintenance_and_restarts_after_empty_cycle() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("auto-maintenance")?;
    let server = serve_responses(vec![completion(MAINT_DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("working".to_string())
    );
    assert!(state::get(&conn, "open task")?.is_some_and(|task| task.starts_with("maintenance:")));

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));

    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "notice" && event.content.contains("maintenance cycle opened")
    }));
    Ok(())
}

#[test]
fn owner_queue_preempts_idle_maintenance_at_turn_boundary() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("auto-maintenance-preempt")?;
    let server = serve_responses(vec![completion(WRITE_ACTION), completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    queue::enqueue(&mut conn, "write owner file", "owner-send", "102")?;
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(
        fs::read_to_string(workspace.join("owner.txt"))?,
        "owner wins"
    );

    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Done);
    server.join()?;
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn closed_owner_task_returns_to_maintenance_on_next_empty_boundary() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "write owner file", "owner-send", "101")?;
    let workspace = temp_workspace("auto-maintenance-after-task")?;
    let server = serve_responses(vec![completion(WRITE_ACTION), completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));

    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert!(state::get(&conn, "open task")?.is_some_and(|task| task.starts_with("maintenance:")));
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

fn take_lock(conn: &rusqlite::Connection) -> TestResult<()> {
    take_daemon_lock(conn, "test", "100", "0")?;
    Ok(())
}
