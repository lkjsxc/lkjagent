use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{insert_case, upsert_state_cell};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn settled_payload_decision_suppresses_selected_state_key() -> TestResult<()> {
    let data = fixture_root("selected-key")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Settle payload state key.");
    assign_step_ids(&mut snapshot);
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_case(&conn, "1", &snapshot.task.objective, "now")?;
    upsert_state_cell(&conn, "1", &snapshot_cell(&snapshot)?)?;
    upsert_state_cell(&conn, "1", &payload_cell("custom.done")?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let _ = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;

    assert_eq!(cell_status(&conn, "custom:settle-me")?, "Suppressed");
    assert_eq!(decision_selected_key(&conn)?, "custom:settle-me");
    Ok(())
}

#[test]
fn state_resolve_payload_settles_without_blocking_matter() -> TestResult<()> {
    let data = fixture_root("state-resolve")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Resolve payload state key.");
    assign_step_ids(&mut snapshot);
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_case(&conn, "1", &snapshot.task.objective, "now")?;
    upsert_state_cell(&conn, "1", &snapshot_cell(&snapshot)?)?;
    upsert_state_cell(&conn, "1", &payload_cell("state.resolve")?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let next = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;

    assert_eq!(format!("{:?}", next.task.state), "Open");
    assert_eq!(cell_status(&conn, "custom:settle-me")?, "Suppressed");
    assert_eq!(blocked_events(&conn)?, 0);
    Ok(())
}

#[test]
fn payload_workspace_write_effect_creates_file_and_artifact() -> TestResult<()> {
    let data = fixture_root("workspace-write")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Write from native state.");
    assign_step_ids(&mut snapshot);
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_case(&conn, "1", &snapshot.task.objective, "now")?;
    upsert_state_cell(&conn, "1", &snapshot_cell(&snapshot)?)?;
    upsert_state_cell(&conn, "1", &payload_write_cell()?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let _ = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let file = data.join("workspace/native/effect.md");

    assert_eq!(fs::read_to_string(file)?, "Native body");
    assert_eq!(cell_status(&conn, "custom:settle-me")?, "Suppressed");
    assert_eq!(count_rows(&conn, "artifacts")?, 2);
    Ok(())
}

fn snapshot_cell(snapshot: &lkjagent_core::model::TaskSnapshot) -> TestResult<StateCell> {
    let mut cell = StateCell::active(key("case", "snapshot")?, "snapshot-event");
    cell.payload_schema = "matter-snapshot".to_string();
    cell.payload_json = serde_json::to_string(snapshot)?;
    cell.created_at = "now".to_string();
    cell.updated_at = "now".to_string();
    Ok(cell)
}

fn payload_cell(operation: &str) -> TestResult<StateCell> {
    let mut cell = StateCell::active(key("custom", "settle-me")?, "custom-event");
    cell.payload_schema = "custom.operation".to_string();
    cell.payload_json = serde_json::json!({"operation_key": operation}).to_string();
    cell.created_at = "now".to_string();
    cell.updated_at = "now".to_string();
    Ok(cell)
}

fn payload_write_cell() -> TestResult<StateCell> {
    let mut cell = StateCell::active(key("custom", "settle-me")?, "custom-event");
    cell.payload_schema = "custom.operation".to_string();
    cell.payload_json = serde_json::json!({
        "operation_key": "effect.workspace.write",
        "effect_command": {
            "name":"workspace.write_text",
            "path":"native/effect.md",
            "content":"Native body"
        }
    })
    .to_string();
    cell.created_at = "now".to_string();
    cell.updated_at = "now".to_string();
    Ok(cell)
}

fn assign_step_ids(snapshot: &mut lkjagent_core::model::TaskSnapshot) {
    for step in &mut snapshot.steps {
        step.id = snapshot.task.id.saturating_mul(1_000) + step.ordinal as u64;
    }
}

fn key(namespace: &str, name: &str) -> Result<StateKey, String> {
    StateKey::new(namespace, name).map_err(|error| error.message)
}

fn cell_status(conn: &Connection, key_label: &str) -> TestResult<String> {
    Ok(conn.query_row(
        "SELECT status FROM state_cells WHERE key_label = ?1",
        [key_label],
        |row| row.get(0),
    )?)
}

fn count_rows(conn: &Connection, table: &str) -> TestResult<i64> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    Ok(conn.query_row(&sql, [], |row| row.get(0))?)
}

fn blocked_events(conn: &Connection) -> TestResult<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM events WHERE kind = 'taskblocked'",
        [],
        |row| row.get(0),
    )?)
}

fn decision_selected_key(conn: &Connection) -> TestResult<String> {
    let body: String =
        conn.query_row("SELECT decision_json FROM runtime_decisions", [], |row| {
            row.get(0)
        })?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json["selected_state_key"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-state-key-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
