mod support;

use std::path::Path;

use lkjagent_protocol::render_action;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::maintenance::{MaintenanceCycle, MaintenanceDirective};
use lkjagent_runtime::mode::{decide_turn_authority, TurnAuthorityInput};
use lkjagent_runtime::task::{PendingAction, TaskState};
use lkjagent_store::{queue, runtime_authority, state};
use support::http::{completion, serve_responses};
use support::{action, runtime_state, store, temp_workspace, TestResult};

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
    assert_eq!(
        state::get(&conn, "authority active mode")?,
        Some("OwnerTask".to_string())
    );
    assert!(state::get(&conn, "authority node")?.is_some());
    assert!(state::get(&conn, "authority next action")?.is_some());
    let decision = runtime_authority::latest_decision(&conn, 1)?.ok_or("missing decision")?;
    assert_eq!(decision.mission, "owner_execution");
    assert_eq!(decision.active_mode, "OwnerTask");
    let event_kind: String = conn.query_row(
        "SELECT event_kind FROM runtime_authority_events ORDER BY id DESC LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(event_kind, "owner_message_received");
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert_eq!(active_mode_cards(&daemon), 1);
    assert!(active_mode_card(&daemon).contains("policy_layers=graph"));
    Ok(())
}

#[test]
fn stale_maintenance_action_is_refused_when_owner_queue_arrives() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "write owner file", "owner-send", "101")?;
    let workspace = temp_workspace("stale-maintenance-action")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let action = action("agent.done", &[("summary", "maintenance complete")]);
    let action_text = render_action(&action);
    daemon.state.task = TaskState::Idle;
    daemon.state.maintenance = Some(MaintenanceCycle {
        directive: MaintenanceDirective::AuditSelf,
        turns_remaining: 3,
    });
    daemon.state.pending_action = Some(PendingAction {
        action,
        action_text: action_text.clone(),
    });
    daemon.turn_authority = Some(decide_turn_authority(TurnAuthorityInput {
        maintenance_active: true,
        ..TurnAuthorityInput::default()
    }));

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.pending_action.is_none());
    assert!(daemon.state.maintenance.is_none());
    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("stale model action refused")
            && frame.content.contains("active_mode=OwnerTask")
            && frame.content.contains("failed_gate=stale-turn-authority")
    }));
    assert_eq!(queue::pending_count(&conn)?, 1);
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
