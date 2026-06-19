mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::queue;
use support::http::serve_responses;
use support::{store, temp_workspace, TestResult};

#[test]
fn counted_scaffold_task_summary_links_to_closed_graph_case() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "900", "0")?;
    queue::enqueue(
        &mut conn,
        "Create about 20 files total for docs and main content.",
        "owner-send",
        "901",
    )?;
    let workspace = temp_workspace("graph-memory-link-counted")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "901")?, DaemonTick::Idle);
    server.join()?;

    assert_eq!(file_count(&workspace.join("structured-output"))?, 20);
    let (case_id, active_node): (i64, String) = conn.query_row(
        "SELECT id, active_node FROM graph_cases ORDER BY id DESC LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    let summary_id: i64 = lkjagent_store::state::get(&conn, "last task summary id")?
        .ok_or("missing task summary id")?
        .parse()?;
    let links = lkjagent_store::graph::memory_links_for_case(&conn, case_id)?;
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].memory_id, summary_id);
    assert_eq!(links[0].node, active_node);
    assert_eq!(links[0].reason, "task-summary");
    Ok(())
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(support::runtime_state()?, runtime))
}

fn file_count(path: &Path) -> TestResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(file_count(&child)?);
        } else {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
