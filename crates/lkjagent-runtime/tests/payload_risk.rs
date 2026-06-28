mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue};
use support::http::{length_completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn max_token_inside_write_routes_to_payload_recovery() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "Create long SF story.", "owner-send", "101")?;
    let workspace = temp_workspace("payload-risk")?;
    let server = serve_responses(vec![length_completion(
        "<action>\n<tool>fs.write</tool>\n<path>story.md</path>\n<content>",
    )])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.graph.as_ref().is_some_and(|graph| {
        graph.active_node.0 == "recover-by-artifact-plan"
            && graph.next_action_class == "artifact-plan-or-bounded-write"
    }));
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.content.contains("same-shape retry is blocked")
            && event.content.contains("artifact.next")
            && event.content.contains("one-file fs.batch_write")
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
