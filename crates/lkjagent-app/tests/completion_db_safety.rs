use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepState, TaskSnapshot, TaskState};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn db_blocked_bridge_shape_becomes_blocked_not_closed() -> TestResult<()> {
    let data = fixture_root("blocked-bridge")?;
    let mut snapshot = instantiate(1, "Create something to read with structured details");
    snapshot.steps[0].state = StepState::Blocked;
    for step in snapshot.steps.iter_mut().skip(1) {
        step.state = StepState::Done;
    }
    snapshot.task.checks.clear();
    persist(&data, &snapshot)?;

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let next = run_until_idle(&data, &mut endpoint, 3)?;

    assert_eq!(next.task.state, TaskState::Blocked);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(task_state(&conn)?, "blocked");
    assert_eq!(count(&conn, "check_results")?, 0);
    assert_eq!(count(&conn, "workspace_records")?, 0);
    assert_eq!(count(&conn, "artifacts")?, 0);
    assert_eq!(count(&conn, "tool_admissions")?, 0);
    assert_eq!(count(&conn, "observations")?, 0);
    assert_eq!(task_closed_events(&conn)?, 0);
    Ok(())
}

fn persist(data: &Path, snapshot: &TaskSnapshot) -> TestResult<()> {
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    Ok(())
}

fn task_state(conn: &Connection) -> TestResult<String> {
    Ok(conn.query_row("select state from tasks where id = 1", [], |row| row.get(0))?)
}

fn count(conn: &Connection, table: &str) -> TestResult<i64> {
    let sql = format!("select count(*) from {table}");
    Ok(conn.query_row(&sql, [], |row| row.get(0))?)
}

fn task_closed_events(conn: &Connection) -> TestResult<i64> {
    Ok(conn.query_row(
        "select count(*) from events where kind = 'taskclosed'",
        [],
        |row| row.get(0),
    )?)
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-completion-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
