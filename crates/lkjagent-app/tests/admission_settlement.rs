use std::{fs, path::PathBuf};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

use support::action_pairs;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn rejected_admission_rolls_back_with_late_failure() -> TestResult<()> {
    let data = fixture_root()?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Investigate workspace files", "now")?;
    conn.execute_batch(
        "CREATE TRIGGER fail_admission_settlement BEFORE UPDATE OF status ON runtime_decisions
         WHEN NEW.status <> OLD.status BEGIN
         SELECT RAISE(FAIL, 'forced admission settlement failure'); END;",
    )?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![action_pairs("fs.read", &[("path", "../secret")])],
        index: 0,
    };
    let error = run_until_idle(&data, &mut endpoint, 1)
        .err()
        .ok_or("rejected admission unexpectedly settled")?;
    assert!(error.contains("forced admission settlement failure"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let decision: String =
        conn.query_row("SELECT status FROM runtime_decisions", [], |row| row.get(0))?;
    let count = |table: &str| -> rusqlite::Result<i64> {
        conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
            row.get(0)
        })
    };
    let recovery: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'recovery.failure'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(decision, "pending");
    assert_eq!(
        (count("tool_admissions")?, count("attempts")?, recovery),
        (0, 0, 0)
    );
    Ok(())
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-admission-settlement-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
