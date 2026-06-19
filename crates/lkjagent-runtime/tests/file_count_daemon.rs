mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{memory, queue, state};
use support::http::{completion, serve_responses};
use support::{
    contains_all, store, temp_workspace, TestResult, COUNTED_EVIDENCE_TERMS, COUNTED_SUMMARY_TERMS,
};

const BATCH_WRITE: &str = "<act>
<tool>shell.run</tool>
<command>
set -eu
rm -rf deliverable
mkdir -p deliverable/docs deliverable/main
printf '# Deliverable\n' > deliverable/README.md
printf '# Doc\n' > deliverable/docs/plan.md
printf '# Main 1\n' > deliverable/main/part-001.md
printf '# Main 2\n' > deliverable/main/part-002.md
printf '# Main 3\n' > deliverable/main/part-003.md
printf 'files='
find deliverable -type f | wc -l
</command>
</act>";

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>created a README-indexed five-file deliverable</summary>
</act>";

#[test]
fn counted_documentation_task_closes_after_batch_shell_write() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "Create a code package with exactly 5 files total.",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("file-count-daemon")?;
    let server = serve_responses(vec![completion(BATCH_WRITE), completion(DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(file_count(&workspace.join("deliverable"))?, 5);
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn counted_documentation_task_auto_scaffolds_before_endpoint() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "200", "0")?;
    queue::enqueue(
        &mut conn,
        "Create about 20 files total for docs and main content.",
        "owner-send",
        "201",
    )?;
    let workspace = temp_workspace("file-count-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "201")?, DaemonTick::Idle);
    assert_eq!(file_count(&workspace.join("structured-output"))?, 20);
    assert_eq!(daemon.poll_once(&mut conn, "202")?, DaemonTick::Idle);
    server.join()?;

    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    assert_closed_graph_case(&conn)?;
    assert_counted_graph_evidence(&conn, 20)?;
    assert_counted_task_summary(&conn)?;
    Ok(())
}

#[test]
fn counted_documentation_task_auto_scaffolds_full_width_japanese_count() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "300", "0")?;
    queue::enqueue(
        &mut conn,
        "合計２０ファイル程度の大きな物語を、設計メモと本文に分けて作ってください。",
        "owner-send",
        "301",
    )?;
    let workspace = temp_workspace("file-count-auto-full-width")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "301")?, DaemonTick::Idle);
    assert_eq!(file_count(&workspace.join("structured-output"))?, 20);
    assert_eq!(daemon.poll_once(&mut conn, "302")?, DaemonTick::Idle);
    server.join()?;

    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
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

fn assert_counted_graph_evidence(conn: &rusqlite::Connection, target: usize) -> TestResult<()> {
    let mut statement = conn.prepare(
        "SELECT requirement, summary, path
         FROM graph_evidence
         WHERE summary LIKE 'counted document scaffold%'
         ORDER BY id",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
        ))
    })?;
    let evidence = rows.collect::<Result<Vec<_>, _>>()?;
    assert!(evidence.len() >= 2, "{evidence:?}");
    assert!(
        evidence.iter().any(|(requirement, _, path)| {
            requirement == "document-structure" && path.as_deref() == Some("structured-output")
        }),
        "{evidence:?}"
    );
    assert!(
        evidence.iter().any(|(requirement, summary, path)| {
            requirement == "verification"
                && summary.contains(&format!("files={target}"))
                && contains_all(summary, COUNTED_EVIDENCE_TERMS)
                && path.as_deref() == Some("structured-output")
        }),
        "{evidence:?}"
    );
    Ok(())
}

fn assert_counted_task_summary(conn: &rusqlite::Connection) -> TestResult<()> {
    assert!(
        memory::find(conn, "file_budget", 5)?.iter().any(|row| {
            row.kind == "task-summary" && contains_all(&row.content, COUNTED_SUMMARY_TERMS)
        }),
        "missing counted task-summary evidence"
    );
    Ok(())
}

fn assert_closed_graph_case(conn: &rusqlite::Connection) -> TestResult<()> {
    let (phase, active_node, status): (String, String, String) = conn.query_row(
        "SELECT phase, active_node, status FROM graph_cases ORDER BY id DESC LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!(phase, "closed");
    assert_eq!(active_node, "complete");
    assert_eq!(status, "closed");
    Ok(())
}
