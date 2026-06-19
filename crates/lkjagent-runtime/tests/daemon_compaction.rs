mod support;

use std::path::Path;

use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn daemon_compacts_before_endpoint_when_prediction_crosses_hard_trigger() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("daemon-pre-endpoint-compact")?;
    let mut daemon = daemon("http://127.0.0.1:9", &workspace)?;
    daemon.state.task = lkjagent_runtime::task::TaskState::Open { turns_remaining: 7 };
    daemon.state.context.log.push(Frame::new(
        FrameKind::Observation,
        "large prior output",
        20_000,
    ));

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.endpoint_attempt, 0);
    assert!(daemon.state.context.used_tokens() <= 8_192);
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "compaction" && event.content.contains("context_window=24576")
    }));
    Ok(())
}

#[test]
fn daemon_compacts_before_owner_delivery_can_cross_hard_trigger() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "continue with more detail", "owner-send", "101")?;
    let workspace = temp_workspace("daemon-pre-owner-compact")?;
    let mut daemon = daemon("http://127.0.0.1:9", &workspace)?;
    daemon.state.context.log.push(Frame::new(
        FrameKind::Observation,
        "large prior output",
        19_500,
    ));

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(queue::list(&conn)?
        .first()
        .is_some_and(|row| row.status == "pending"));
    assert!(daemon.state.context.used_tokens() <= 8_192);
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
