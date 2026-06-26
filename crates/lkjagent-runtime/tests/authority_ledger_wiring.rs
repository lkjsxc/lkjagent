mod support;

use std::{fs, path::Path};

use lkjagent_protocol::render_action;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::{PendingAction, TaskState};
use lkjagent_store::{queue, runtime_authority, state};
use support::http::{completion, serve_responses};
use support::{action, runtime_state, store, temp_workspace, TestResult};

const GRAPH_STATE: &str = "<action>
<tool>graph.state</tool>
</action>";

#[test]
fn daemon_records_prompt_frame_and_effect_observation() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "inspect the graph", "owner-send", "101")?;
    let workspace = temp_workspace("authority-ledger-wiring")?;
    let server = serve_responses(vec![completion(GRAPH_STATE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    let authority_log = find_authority_json(&workspace)?;
    assert!(authority_log.contains("kernel_mission"));
    assert!(authority_log.contains("kernel_staleness_fingerprint"));

    let prompt_frame_id = state::get(&conn, "authority prompt frame id")?
        .ok_or("missing authority prompt frame id")?
        .parse::<i64>()?;
    let prompt_kind: String = conn.query_row(
        "SELECT frame_kind FROM runtime_prompt_frames WHERE id = ?1",
        [prompt_frame_id],
        |row| row.get(0),
    )?;
    assert_eq!(prompt_kind, "authority");
    assert_eq!(
        state::get(&conn, "kernel mission")?,
        Some("owner_execution".to_string())
    );
    assert!(state::get(&conn, "kernel event id")?.is_some());

    let decision = runtime_authority::latest_decision(&conn, 1)?.ok_or("missing decision")?;
    assert_eq!(
        decision.authority_fingerprint,
        state::get(&conn, "kernel authority fingerprint")?.ok_or("missing kernel authority")?
    );
    assert_eq!(
        decision.staleness_fingerprint,
        state::get(&conn, "kernel staleness fingerprint")?.ok_or("missing kernel stale")?
    );
    let observation = runtime_authority::latest_observation_for_decision(&conn, decision.id)?
        .ok_or("missing runtime observation")?;
    assert!(observation.admission_id.is_some());
    assert!(observation.effect_id.is_some());
    assert!(!observation.summary.is_empty());
    Ok(())
}

#[test]
fn changed_staleness_fingerprint_refuses_persisted_pending_action() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    state::set(&conn, "authority decision id", "1")?;
    state::set(&conn, "authority prompt frame id", "frame-same")?;
    state::set(&conn, "kernel staleness fingerprint", "stale-current")?;
    let workspace = temp_workspace("authority-stale-fingerprint")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let action = action("graph.state", &[]);
    daemon.state.task = TaskState::Open { turns_remaining: 3 };
    daemon.state.pending_action = Some(PendingAction {
        action: action.clone(),
        action_text: render_action(&action),
        authority_decision_id: Some("1".to_string()),
        prompt_frame_id: Some("frame-same".to_string()),
        staleness_fingerprint: Some("stale-old".to_string()),
    });
    daemon.turn_authority = None;

    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("reason=stale_decision")
            && frame.content.contains("staleness_fingerprint")
    }));
    Ok(())
}

#[test]
fn changed_prompt_frame_refuses_persisted_pending_action() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    state::set(&conn, "authority decision id", "1")?;
    state::set(&conn, "authority prompt frame id", "frame-new")?;
    let workspace = temp_workspace("authority-stale-prompt-frame")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let action = action("graph.state", &[]);
    daemon.state.task = TaskState::Open { turns_remaining: 3 };
    daemon.state.pending_action = Some(PendingAction {
        action: action.clone(),
        action_text: render_action(&action),
        authority_decision_id: Some("decision-old".to_string()),
        prompt_frame_id: Some("frame-old".to_string()),
        staleness_fingerprint: None,
    });
    daemon.turn_authority = None;

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("reason=stale_decision")
            && frame.content.contains("failed_gate=stale-persisted-action")
            && frame.content.contains("prompt_frame_id")
    }));
    Ok(())
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    )
    .with_model_log_path(workspace.join("logs/current-model-run.md"));
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}

fn find_authority_json(workspace: &Path) -> TestResult<String> {
    let model_dir = workspace.join("logs/model");
    for epoch in fs::read_dir(model_dir)? {
        let path = epoch?.path().join("case-1/turn-000000/authority.json");
        if path.is_file() {
            return Ok(fs::read_to_string(path)?);
        }
    }
    Err("missing authority.json".into())
}
