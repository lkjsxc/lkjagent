mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::{events, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const WRITE_ACTION: &str = "<act>
<tool>fs.write</tool>
<path>out.txt</path>
<content>hello</content>
</act>";
const DONE_ACTION: &str = "<act>
<tool>agent.done</tool>
<summary>wrote file</summary>
</act>";

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
    let server = serve_responses(vec![completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 0 };

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(state::get(&conn, "daemon question")?, None);
    assert!(queue::list(&conn)?
        .first()
        .is_some_and(|row| row.status == "delivered"));
    Ok(())
}

#[test]
fn exhausted_task_waits_visibly_and_resumes_from_next_send() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    state::set(&conn, "open task", "long task")?;
    let workspace = temp_workspace("daemon-exhausted-wait")?;
    let server = serve_responses(vec![completion(WRITE_ACTION), completion(DONE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 0 };

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Waiting);
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("waiting".to_string())
    );
    assert!(state::get(&conn, "daemon question")?
        .is_some_and(|question| question.contains("Turn budget exhausted")));

    queue::enqueue(&mut conn, "continue", "owner-send", "102")?;
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;

    let log = events::read_events(&conn)?;
    assert!(log.iter().any(|event| {
        event.kind == "notice" && event.content.contains("turn budget exhausted")
    }));
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
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
