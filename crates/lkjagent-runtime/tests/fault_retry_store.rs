mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::queue;
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn daemon_persists_retry_count_for_repeated_parse_fault() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "recover parse loop", "owner-send", "101")?;
    let workspace = temp_workspace("fault-retry-store")?;
    let server = serve_responses(vec![
        completion("not an act"),
        completion("still not an act"),
        completion("again not an act"),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Working);

    let active = lkjagent_store::graph::active_case(&conn)?.ok_or("missing graph case")?;
    assert_eq!(active.phase, "recovery");
    assert!(active.active_node.starts_with("recover-"));
    assert!(retry_count(&conn, active.id)?.unwrap_or_default() <= 3);
    Ok(())
}

fn retry_count(conn: &rusqlite::Connection, case_id: i64) -> TestResult<Option<u8>> {
    Ok(lkjagent_store::graph::faults::retry_count(
        conn,
        &lkjagent_store::graph::faults::FaultRetryKey {
            case_id,
            node: "plan",
            tool: "none",
            parameter_shape: "none",
            fault_class: "parse",
        },
    )?)
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
