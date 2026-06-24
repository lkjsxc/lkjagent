mod support;

use std::path::Path;

use lkjagent_protocol::render_action;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::mode::{decide_turn_authority, TurnAuthorityInput};
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

    let prompt_frame_id = state::get(&conn, "authority prompt frame id")?
        .ok_or("missing authority prompt frame id")?
        .parse::<i64>()?;
    let prompt_kind: String = conn.query_row(
        "SELECT frame_kind FROM runtime_prompt_frames WHERE id = ?1",
        [prompt_frame_id],
        |row| row.get(0),
    )?;
    assert_eq!(prompt_kind, "authority");

    let decision = runtime_authority::latest_decision(&conn, 1)?.ok_or("missing decision")?;
    let observation = runtime_authority::latest_observation_for_decision(&conn, decision.id)?
        .ok_or("missing runtime observation")?;
    assert!(observation.admission_id.is_some());
    assert!(observation.effect_id.is_some());
    assert!(!observation.summary.is_empty());
    Ok(())
}

#[test]
fn changed_prompt_frame_refuses_cached_action() -> TestResult<()> {
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
    });
    daemon.turn_authority = Some(decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        latest_decision_id: Some("decision-old".to_string()),
        prompt_frame_id: Some("frame-old".to_string()),
        ..TurnAuthorityInput::default()
    }));

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("reason=stale_decision") && frame.content.contains("prompt_frame_id")
    }));
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
