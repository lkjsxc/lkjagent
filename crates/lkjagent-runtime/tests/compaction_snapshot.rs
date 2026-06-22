mod support;

use std::path::Path;

use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::{initial_state, GraphNodeId, TaskPhase};
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::state;
use support::http::serve_responses;
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn compaction_snapshot_renders_recovery_resume_fields() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    state::set(&conn, "open task", "create detailed bread dictionary")?;
    let workspace = temp_workspace("daemon-compaction-snapshot")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 7 };
    daemon.state.graph = Some(recovery_graph());
    if let Some(root) = daemon
        .state
        .graph
        .as_ref()
        .and_then(|graph| graph.document.as_ref().map(|doc| doc.root.clone()))
    {
        state::set(
            &conn,
            &format!("artifact.next cursor {root}"),
            "docs/next.md",
        )?;
    }
    push_orange_pressure(&mut daemon);

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    let summary = compaction_notice(&daemon)?;
    for field in [
        "active_mission=Recovery",
        "required_evidence=",
        "missing_evidence=",
        "active_artifact_id=",
        "write_batch_cursor=docs/next.md",
        "recovery_ladder_step=3",
        "last_failed_action=fs.write:large",
        "last_successful_observation=large prior output",
        "admitted_next_tools=",
        "exact_next_valid_action=",
        "completion_blocked_reasons=",
    ] {
        assert!(summary.contains(field), "missing {field}");
    }
    assert!(summary.contains("<tool>artifact.next</tool>"));
    Ok(())
}

fn recovery_graph() -> lkjagent_graph::TaskGraphState {
    let mut graph = initial_state("Create a big bread cookbook with recipes.", Some(8));
    graph.phase = TaskPhase::Recovery;
    graph.active_node = GraphNodeId("recover-by-artifact-plan");
    graph.recovery.ladder_position = 3;
    graph.recovery.last_failed_action_fingerprint = Some("fs.write:large".to_string());
    graph
}

fn push_orange_pressure(daemon: &mut ResidentDaemon) {
    let used = daemon.state.context.used_tokens();
    let tokens = daemon
        .runtime
        .budget
        .soft_trigger
        .saturating_sub(used)
        .saturating_add(32);
    daemon.state.context.log.push(Frame::new(
        FrameKind::Observation,
        "large prior output",
        tokens,
    ));
}

fn compaction_notice(daemon: &ResidentDaemon) -> TestResult<&str> {
    daemon
        .state
        .context
        .log
        .iter()
        .find(|frame| frame.content.contains("compaction resume"))
        .map(|frame| frame.content.as_str())
        .ok_or_else(|| "missing compaction notice".into())
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
