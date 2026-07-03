use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, StepState, TaskState};
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::plan_turn::set_config;
use rusqlite::Connection;

const SNAPSHOT_KEY: &str = "app.active-snapshot";

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
        outputs: vec!["<message>Rows are authority.</message>".to_string()],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 3)?;
    assert_eq!(snapshot.task.id, 1);
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert_eq!(snapshot.task.summary, "Rows are authority.");
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
        outputs: vec!["<message>Which file?</message>".to_string()],
        index: 0,
    };
    let waiting = run_until_idle(&data, &mut first, 1)?;
    assert_eq!(waiting.task.state, TaskState::Waiting);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "README.md", "now")?;
    drop(conn);
    let mut second = ScriptedEndpoint {
        outputs: vec!["<message>README.md was selected.</message>".to_string()],
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

fn persist(conn: &mut Connection, snapshot: &lkjagent_core::model::TaskSnapshot) -> TestResult<()> {
    insert_task(conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-resume-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
