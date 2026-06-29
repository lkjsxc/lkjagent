mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::{events, queue, state};
use support::http::serve_responses;
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn owner_send_refreshes_exhausted_open_task_budget() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    state::set(&conn, "open task", "long task")?;
    queue::enqueue(
        &mut conn,
        "continue with owner guidance",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("daemon-exhausted-owner")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 0 };

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    assert!(
        matches!(daemon.state.task, TaskState::Open { turns_remaining } if turns_remaining > 0)
    );
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("working".to_string())
    );
    assert_eq!(state::get(&conn, "daemon question")?, None);
    assert!(queue::list(&conn)?
        .first()
        .is_some_and(|row| row.status == "delivered"));
    Ok(())
}

#[test]
fn exhausted_task_checkpoints_and_continues_without_owner_guidance() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    state::set(&conn, "open task", "long task")?;
    let workspace = temp_workspace("daemon-exhausted-continue")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 0 };

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    assert_eq!(state::get(&conn, "daemon question")?, None);
    assert!(
        matches!(daemon.state.task, TaskState::Open { turns_remaining } if turns_remaining > 0)
    );
    let log = events::read_events(&conn)?;
    assert!(log
        .iter()
        .any(|event| { event.kind == "notice" && event.content.contains("TurnBudgetCheckpoint") }));
    assert!(!log.iter().any(|event| event
        .content
        .contains("Turn budget exhausted. Send guidance to continue.")));
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("working".to_string())
    );
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

fn take_lock(conn: &rusqlite::Connection) -> TestResult<()> {
    take_daemon_lock(conn, "test", "100", "0")?;
    Ok(())
}
