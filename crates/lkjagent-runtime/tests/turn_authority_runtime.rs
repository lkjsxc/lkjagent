mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::queue;
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const GRAPH_STATE: &str = "<act>
<tool>graph.state</tool>
</act>";

#[test]
fn endpoint_turn_refreshes_one_active_mode_card() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "inspect the graph", "owner-send", "101")?;
    let workspace = temp_workspace("authority-card")?;
    let server = serve_responses(vec![completion(GRAPH_STATE), completion(GRAPH_STATE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(active_mode_cards(&daemon), 1);
    assert!(active_mode_card(&daemon).contains("mode=OwnerTask"));
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert_eq!(active_mode_cards(&daemon), 1);
    assert!(active_mode_card(&daemon).contains("policy_layers=graph"));
    Ok(())
}

#[test]
fn closed_idle_does_not_call_endpoint() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    lkjagent_runtime::maintenance::defer_all_directives(&conn, "101")?;
    let workspace = temp_workspace("closed-idle-authority")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Idle);
    server.join()?;

    assert_eq!(active_mode_cards(&daemon), 0);
    assert_eq!(daemon.endpoint_attempt, 0);
    Ok(())
}

fn active_mode_cards(daemon: &ResidentDaemon) -> usize {
    daemon
        .state
        .context
        .log
        .iter()
        .filter(|frame| frame.content.starts_with("Active Mode:\n"))
        .count()
}

fn active_mode_card(daemon: &ResidentDaemon) -> String {
    daemon
        .state
        .context
        .log
        .iter()
        .find(|frame| frame.content.starts_with("Active Mode:\n"))
        .map_or_else(String::new, |frame| frame.content.clone())
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
