mod support;

use std::net::TcpListener;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn endpoint_error_waits_until_retry_deadline_before_next_attempt() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "fail endpoint", "owner-send", "101")?;
    let workspace = temp_workspace("endpoint-retry")?;
    let base_url = closed_local_url()?;
    let mut daemon = daemon(&base_url, &workspace)?;

    assert_eq!(
        daemon.poll_once(&mut conn, "101")?,
        DaemonTick::EndpointError
    );
    assert_eq!(daemon.endpoint_attempt, 1);
    assert_eq!(daemon.endpoint_retry_at.as_deref(), Some("102"));
    assert_eq!(error_events(&conn)?, 1);

    assert_eq!(
        daemon.poll_once(&mut conn, "101")?,
        DaemonTick::EndpointError
    );
    assert_eq!(daemon.endpoint_attempt, 1);
    assert_eq!(daemon.endpoint_retry_at.as_deref(), Some("102"));
    assert_eq!(error_events(&conn)?, 1);

    assert_eq!(
        daemon.poll_once(&mut conn, "102")?,
        DaemonTick::EndpointError
    );
    assert_eq!(daemon.endpoint_attempt, 2);
    assert_eq!(daemon.endpoint_retry_at.as_deref(), Some("104"));
    assert_eq!(error_events(&conn)?, 2);
    Ok(())
}

fn closed_local_url() -> TestResult<String> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}", listener.local_addr()?);
    drop(listener);
    Ok(url)
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

fn error_events(conn: &rusqlite::Connection) -> TestResult<usize> {
    Ok(events::read_events(conn)?
        .iter()
        .filter(|event| event.kind == "error")
        .count())
}
