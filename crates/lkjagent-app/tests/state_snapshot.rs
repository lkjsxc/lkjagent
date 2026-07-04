use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{insert_case, upsert_state_cell};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn state_snapshot_cell_wins_over_older_plan_rows() -> TestResult<()> {
    let data = fixture_root("state-snapshot")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut stale = instantiate(1, "stale plan row");
    assign_step_ids(&mut stale);
    persist(&mut conn, &stale)?;
    let mut state_snapshot = instantiate(2, "What is an agent?");
    assign_step_ids(&mut state_snapshot);
    persist(&mut conn, &state_snapshot)?;
    insert_case(&conn, "2", &state_snapshot.task.objective, "later")?;
    upsert_state_cell(&conn, "2", &snapshot_cell(&state_snapshot)?)?;
    drop(conn);
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("task: 2 Open"));

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>State cell wins.</message>".to_string()],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    assert_eq!(snapshot.task.id, 2);
    assert_eq!(snapshot.task.summary, "State cell wins.");
    Ok(())
}

fn snapshot_cell(snapshot: &TaskSnapshot) -> Result<StateCell, serde_json::Error> {
    let key = StateKey::new("case", "snapshot").unwrap_or_else(|_| StateKey {
        namespace: "case".to_string(),
        name: "snapshot".to_string(),
    });
    let mut cell = StateCell::active(key, "event-snapshot");
    cell.payload_schema = "task-snapshot".to_string();
    cell.payload_json = serde_json::to_string(snapshot)?;
    cell.created_at = "later".to_string();
    cell.updated_at = "later".to_string();
    Ok(cell)
}

fn assign_step_ids(snapshot: &mut TaskSnapshot) {
    let base = snapshot.task.id.saturating_mul(1_000);
    for step in &mut snapshot.steps {
        step.id = base.saturating_add(step.ordinal as u64);
    }
}

fn persist(conn: &mut Connection, snapshot: &TaskSnapshot) -> TestResult<()> {
    insert_task(conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-state-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
