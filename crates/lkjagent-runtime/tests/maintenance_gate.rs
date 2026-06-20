mod support;

use std::path::Path;

use lkjagent_graph::{initial_state, GraphNodeId};
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, memory, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const WRITE_ACTION: &str = "<act>
<tool>fs.write</tool>
<path>owner.txt</path>
<content>owner wins</content>
</act>";
const MEMORY_SAVE: &str = "<act>
<tool>memory.save</tool>
<kind>lesson</kind>
<title>maintenance note</title>
<tags>maintenance</tags>
<content>idle cycle recorded a durable note</content>
</act>";
const MEMORY_PRUNE: &str = "<act>
<tool>memory.prune</tool>
</act>";

#[test]
fn maintenance_blocks_workspace_write_tools() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    let workspace = temp_workspace("maintenance-gate")?;
    let server = serve_responses(vec![completion(WRITE_ACTION), completion(MEMORY_SAVE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert!(!workspace.join("owner.txt").exists());
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "notice" && event.content.contains("maintenance only allows")
    }));

    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    server.join()?;
    assert!(!workspace.join("owner.txt").exists());
    assert!(daemon.state.maintenance.is_some());
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("working".to_string())
    );
    Ok(())
}

#[test]
fn maintenance_memory_save_ignores_stale_graph_policy() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    let workspace = temp_workspace("maintenance-graph-clear")?;
    let server = serve_responses(vec![completion(MEMORY_SAVE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let mut graph = initial_state("closed stale owner graph", None);
    graph.active_node = GraphNodeId("complete");
    daemon.state.graph = Some(graph);

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    let rows = memory::find(&conn, "maintenance note", 10)?;
    assert_eq!(rows.len(), 1);
    assert!(events::read_events(&conn)?.iter().all(|event| {
        !event
            .content
            .contains("graph policy refused tool=memory.save")
    }));
    Ok(())
}

#[test]
fn maintenance_memory_prune_is_allowed() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    let workspace = temp_workspace("maintenance-prune")?;
    let server = serve_responses(vec![completion(MEMORY_PRUNE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "observation" && event.content.contains("memory prune completed")
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
