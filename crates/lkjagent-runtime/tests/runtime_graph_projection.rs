mod support;

use std::path::Path;

use lkjagent_protocol::render_action;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::{PendingAction, TaskState};
use support::http::serve_responses;
use support::{action, runtime_state, store, temp_workspace, TestResult};

#[test]
fn graph_state_reads_store_backed_active_case_without_memory_graph() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    lkjagent_runtime::graph_state::open_owner_case(&conn, "store backed story", "101")?;
    let workspace = temp_workspace("store-backed-graph-state")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let action = action("graph.state", &[]);
    let action_text = render_action(&action);
    daemon.state.task = TaskState::Open { turns_remaining: 3 };
    daemon.state.pending_action = Some(PendingAction {
        action,
        action_text,
        authority_decision_id: None,
        prompt_frame_id: None,
        staleness_fingerprint: None,
    });

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    let joined = daemon
        .state
        .context
        .log
        .iter()
        .map(|frame| frame.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(joined.contains("store backed story"), "{joined}");
    assert!(!joined.contains("no active graph case"), "{joined}");
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
