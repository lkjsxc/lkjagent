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
fn journal_endpoint_closes_with_file_check_and_artifacts() -> TestResult<()> {
    let objective = "Append a journal note for today.";
    let data = fixture_root("journal-artifact")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let outputs = vec![
        "<content># Today\n\nA bounded daily note.</content>".to_string(),
        "<message>Journal updated.</message>".to_string(),
    ];
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };

    let snapshot = run_until_idle(&data, &mut endpoint, 10)?;

    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(data.join("workspace/journal/today.md").exists());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let units: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artifacts WHERE kind = 'unit'",
        [],
        |row| row.get(0),
    )?;
    assert!(units >= 1);
    let refs: String = conn.query_row(
        "SELECT artifact_refs_json FROM check_results WHERE passed = 1 LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert!(refs.contains("task-1-artifact-"));
    let edges: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_edges WHERE relation = 'verifies'",
        [],
        |row| row.get(0),
    )?;
    assert!(edges >= 1);
    let show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "matter",
        "show",
        "1",
    ])?;
    assert!(show.contains("artifacts:"));
    assert!(show.contains("checks: total="));
    Ok(())
}

#[test]
fn journal_parse_fault_recovers_before_verified_close() -> TestResult<()> {
    let objective = "Append a journal note for today.";
    let data = fixture_root("journal-fault")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let outputs = vec![
        "<message>wrong envelope</message>".to_string(),
        "<content># Today\n\nRecovered note.</content>".to_string(),
        "<message>Journal updated after recovery.</message>".to_string(),
    ];
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };

    let snapshot = run_until_idle(&data, &mut endpoint, 10)?;

    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(snapshot
        .attempts
        .iter()
        .any(|attempt| attempt.diagnosis.contains("expected envelope")));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
