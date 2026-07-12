use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, StepState, TaskState};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_store::decision_rows::insert_runtime_decision;
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

const SNAPSHOT_KEY: &str = "app.active-snapshot";

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn config_snapshot_is_ignored_when_rows_are_absent() -> TestResult<()> {
    let data = fixture_root("no-rows")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let stale = instantiate(42, "stale task from config");
    set_config(&conn, SNAPSHOT_KEY, &serde_json::to_string(&stale)?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 0)?;
    assert_eq!(snapshot.task.id, 0);
    assert_eq!(snapshot.task.state, TaskState::Closed);
    Ok(())
}

#[test]
fn rows_win_over_stale_config_snapshot() -> TestResult<()> {
    let data = fixture_root("rows-win")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    persist(&mut conn, &instantiate(1, "What is an agent?"))?;
    let stale = instantiate(99, "stale task from config");
    set_config(&conn, SNAPSHOT_KEY, &serde_json::to_string(&stale)?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>Rows are authority.</message></final>".to_string()],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 3)?;
    assert_eq!(snapshot.task.id, 1);
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert_eq!(snapshot.task.summary, "Rows are authority.");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let active: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE namespace = 'model' AND status = 'Active'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(active, 0);
    Ok(())
}

#[test]
fn pending_runtime_decision_is_reused_on_resume() -> TestResult<()> {
    let data = fixture_root("decision-reuse")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What is an agent?");
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let decision = pending_decision(&snapshot);
    let expected_fp = decision.tool_view_fingerprint().unwrap_or_default();
    insert_runtime_decision(&conn, &decision, "pending", "before")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>Rows are authority.</message></final>".to_string()],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (i64, String, String, String) = conn.query_row(
        "SELECT COUNT(*), id, status, tool_view_fingerprint FROM runtime_decisions",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    assert_eq!(row.0, 1);
    assert_eq!(row.1, "decision-reused");
    assert_eq!(row.2, "settled");
    assert_eq!(row.3, expected_fp);
    let events: i64 =
        conn.query_row("SELECT COUNT(*) FROM runtime_events", [], |row| row.get(0))?;
    assert!(events >= 1);
    Ok(())
}

#[test]
fn ask_step_records_answer_and_resumes_next_step() -> TestResult<()> {
    let data = fixture_root("ask")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "What is in the workspace?");
    snapshot.steps[0].kind = StepKind::Ask;
    snapshot.steps[0].instruction = "ask which file to inspect".to_string();
    persist(&mut conn, &snapshot)?;
    drop(conn);

    let mut first = ScriptedEndpoint {
        outputs: vec!["<final><message>Which file?</message></final>".to_string()],
        index: 0,
    };
    let waiting = run_until_idle(&data, &mut first, 1)?;
    assert_eq!(waiting.task.state, TaskState::Waiting);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "README.md", "now")?;
    drop(conn);
    let mut second = ScriptedEndpoint {
        outputs: vec!["<final><message>README.md was selected.</message></final>".to_string()],
        index: 0,
    };
    let closed = run_until_idle(&data, &mut second, 3)?;
    assert_eq!(closed.task.state, TaskState::Closed);
    assert!(closed
        .events
        .iter()
        .any(|event| event.content == "README.md"));
    assert_eq!(closed.steps[0].state, StepState::Done);
    Ok(())
}

fn pending_decision(snapshot: &lkjagent_core::model::TaskSnapshot) -> RuntimeDecision {
    let step_id = snapshot.steps.first().map_or(0, |step| step.id);
    RuntimeDecision::new(
        "decision-reused",
        "1",
        OperationKey(format!("model.call/{step_id}")),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    )
}

fn persist(conn: &mut Connection, snapshot: &lkjagent_core::model::TaskSnapshot) -> TestResult<()> {
    insert_task(conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    Ok(())
}

fn set_config(conn: &Connection, key: &str, value: &str) -> TestResult<()> {
    conn.execute(
        "INSERT INTO config(key,value) VALUES(?1,?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        [key, value],
    )?;
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-resume-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
