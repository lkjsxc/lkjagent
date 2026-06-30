mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::queue;
use support::http::serve_responses;
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn direct_manuscript_request_does_not_auto_scaffold_structured_output() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "Write one 700 to 900 word chapter at stories/the-bell-rings-twice/manuscript/chapter-01.md. Do not create structured-output.",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("story-manuscript-no-scaffold")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    let tick = daemon.poll_once(&mut conn, "101")?;
    server.join()?;

    assert_ne!(tick, DaemonTick::Idle);
    assert!(!workspace.join("structured-output").exists());
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
