use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn cli_send_writes_workspace_transcript_trace() -> TestResult<()> {
    let data = fixture_root("workspace-send-trace")?;
    let output = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "send",
        "are you ok?",
    ])?;
    assert!(output.contains("queue: 1"));
    assert!(data.join("workspace/README.md").exists());
    assert_contains(
        &data,
        "workspace/artifacts/transcripts/queue-000001.md",
        "are you ok?",
    )?;
    Ok(())
}

#[test]
fn ambiguous_save_like_turn_writes_inbox_without_endpoint() -> TestResult<()> {
    let data = fixture_root("workspace-inbox")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "remember this", "queued")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(queue_state_count(&conn, "recorded")?, 1);
    assert_eq!(count(&conn, "tasks")?, 0);
    assert_eq!(count(&conn, "workspace_records")?, 0);
    assert_contains(&data, "workspace/inbox/queue-000001.md", "remember this")?;
    Ok(())
}

#[test]
fn empty_workspace_daily_turns_create_record_trace_and_indexes() -> TestResult<()> {
    let data = fixture_root("workspace-empty-regression")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "todo renew passport", "queued")?;
    enqueue(&conn, "are you ok?", "queued")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<final><message>Workspace evidence is linked.</message></final>".to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 2)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert!(data.join("workspace/README.md").exists());
    assert_contains(
        &data,
        "workspace/artifacts/transcripts/queue-000001.md",
        "todo renew",
    )?;
    assert_contains(
        &data,
        "workspace/artifacts/transcripts/queue-000002.md",
        "are you ok?",
    )?;
    assert_eq!(count(&conn, "workspace_records")?, 1);
    assert_eq!(count(&conn, "workspace_record_history")?, 1);
    assert!(data.join("workspace/records/life/todo").exists());
    assert!(index_artifacts(&conn)? > 0);
    let validation = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "workspace",
        "validate",
    ])?;
    assert!(validation.contains("workspace validate: ok"));
    Ok(())
}

fn assert_contains(data: &std::path::Path, rel: &str, needle: &str) -> TestResult<()> {
    let text = fs::read_to_string(data.join(rel))?;
    assert!(text.contains(needle), "{rel}: {needle}");
    Ok(())
}

fn index_artifacts(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM artifacts WHERE kind = 'workspace-index'",
        [],
        |row| row.get(0),
    )
}

fn count(conn: &Connection, table: &str) -> rusqlite::Result<i64> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
}

fn queue_state_count(conn: &Connection, state: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM queue WHERE state = ?1",
        [state],
        |row| row.get(0),
    )
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
