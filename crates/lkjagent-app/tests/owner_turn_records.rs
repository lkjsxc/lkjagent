use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn record_like_owner_turns_write_workspace_files_without_tasks() -> TestResult<()> {
    let data = fixture_root("owner-turn-records")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    for text in [
        "今日はcodexの枠がリセットされる日だったので急いでたくさんaiを使ったと記録してほしい",
        "todo buy milk",
        "record meeting with Emi tomorrow",
        "record that I paid 1200 yen",
        "note that local endpoint is offline",
        "project note for lkjagent",
        "artifact record for the report",
    ] {
        enqueue(&conn, text, "queued")?;
    }
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(count(&conn, "tasks")?, 0);
    assert_eq!(queue_state_count(&conn, "recorded")?, 7);
    assert_eq!(count(&conn, "workspace_records")?, 7);
    assert_eq!(count(&conn, "workspace_record_history")?, 7);
    assert_eq!(index_artifacts(&conn)?, 6);
    for (kind, path_part) in [
        ("journal", "records/life/journal"),
        ("todo", "records/life/todo"),
        ("calendar", "records/life/calendar"),
        ("finance", "records/life/finance"),
        ("note", "records/life/notes"),
        ("project", "records/work/projects"),
        ("artifact", "artifacts/documents"),
    ] {
        assert_path(&conn, &data, kind, path_part)?;
    }
    assert!(data.join("workspace/records/life/README.md").exists());
    assert_contains(&data, "workspace/indexes/README.md", "open-todos.md")?;
    assert_contains(&data, "workspace/indexes/open-todos.md", "todo buy milk")?;
    Ok(())
}

#[test]
fn cli_run_once_processes_record_like_turn() -> TestResult<()> {
    let data = fixture_root("owner-turn-run-once")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "todo buy tea", "queued")?;
    drop(conn);

    let output = cli::run(["--data", data.to_string_lossy().as_ref(), "run", "--once"])?;

    assert!(output.contains("run-once:"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(queue_state_count(&conn, "recorded")?, 1);
    assert_eq!(count(&conn, "workspace_records")?, 1);
    assert!(data.join("workspace/indexes/open-todos.md").exists());
    Ok(())
}

fn assert_path(
    conn: &Connection,
    data: &std::path::Path,
    kind: &str,
    part: &str,
) -> TestResult<()> {
    let path: String = conn.query_row(
        "SELECT path FROM workspace_records WHERE kind = ?1",
        [kind],
        |row| row.get(0),
    )?;
    assert!(path.contains(part), "{kind}: {path}");
    let full = data.join("workspace").join(&path);
    assert!(full.exists());
    let text = fs::read_to_string(full)?;
    let fp: String = conn.query_row(
        "SELECT fingerprint FROM workspace_records WHERE kind = ?1",
        [kind],
        |row| row.get(0),
    )?;
    assert_eq!(fp, record_fingerprint(&text).map_err(|e| e.message)?);
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
