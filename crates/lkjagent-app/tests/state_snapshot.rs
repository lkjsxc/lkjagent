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
    assert!(status.contains("matter: 2 Open"));

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>State cell wins.</message>".to_string()],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    assert_eq!(snapshot.task.id, 2);
    assert_eq!(snapshot.task.summary, "State cell wins.");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(matter_snapshot_rows(&conn)?, 1);
    Ok(())
}

fn matter_snapshot_rows(conn: &Connection) -> TestResult<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE key_label LIKE 'matter:snapshot/%'",
        [],
        |row| row.get(0),
    )?)
}

#[test]
fn matter_snapshot_close_candidate_ignores_stale_plan_steps() -> TestResult<()> {
    let data = fixture_root("state-close-candidate")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut stale = instantiate(3, "What is an agent?");
    assign_step_ids(&mut stale);
    persist(&mut conn, &stale)?;
    let mut complete = stale.clone();
    for step in &mut complete.steps {
        step.state = lkjagent_core::model::StepState::Done;
    }
    insert_case(&conn, "3", &complete.task.objective, "later")?;
    upsert_state_cell(&conn, "3", &snapshot_cell(&complete)?)?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };

    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    assert_eq!(snapshot.task.state, lkjagent_core::model::TaskState::Closed);
    Ok(())
}

#[test]
fn plan_row_fallback_seeds_matter_snapshot_before_turn() -> TestResult<()> {
    let data = fixture_root("state-seed")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(4, "What is an agent?");
    assign_step_ids(&mut snapshot);
    persist(&mut conn, &snapshot)?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };

    let loaded = run_until_idle(&data, &mut endpoint, 0)?;

    assert_eq!(loaded.task.id, 4);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(matter_snapshot_rows(&conn)?, 1);
    Ok(())
}

fn snapshot_cell(snapshot: &TaskSnapshot) -> Result<StateCell, serde_json::Error> {
    let key =
        StateKey::new("matter", format!("snapshot/{}", snapshot.task.id)).unwrap_or_else(|_| {
            StateKey {
                namespace: "matter".to_string(),
                name: "snapshot/fallback".to_string(),
            }
        });
    let mut cell = StateCell::active(key, "event-snapshot");
    cell.payload_schema = "matter-snapshot".to_string();
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
