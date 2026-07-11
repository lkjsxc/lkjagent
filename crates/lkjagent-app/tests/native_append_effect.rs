use std::{fs, path::PathBuf};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{insert_case, upsert_state_cell};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn payload_workspace_append_effect_appends_file_and_artifact() -> TestResult<()> {
    let data = fixture_root("workspace-append")?;
    fs::create_dir_all(data.join("workspace/native"))?;
    fs::write(data.join("workspace/native/effect.md"), "First")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Append from native state.");
    for step in &mut snapshot.steps {
        step.id = snapshot.task.id.saturating_mul(1_000) + step.ordinal as u64;
    }
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_case(&conn, "1", &snapshot.task.objective, "now")?;
    upsert_state_cell(&conn, "1", &snapshot_cell(&snapshot)?)?;
    upsert_state_cell(&conn, "1", &payload_append_cell()?)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let _ = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;

    assert_eq!(
        fs::read_to_string(data.join("workspace/native/effect.md"))?,
        "First Second"
    );
    assert_eq!(cell_status(&conn)?, "Suppressed");
    assert_eq!(count_rows(&conn, "artifacts")?, 2);
    let journal: (String, String) = conn.query_row(
        "SELECT effect_journal.state, observations.content FROM effect_journal
         JOIN observations ON observations.id = effect_journal.observation_id",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(journal.0, "committed");
    assert!(journal.1.contains("path=native/effect.md"));
    assert!(journal.1.contains("fingerprint=fnv1a64:"));
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

fn payload_append_cell() -> TestResult<StateCell> {
    let mut cell = StateCell::active(key("custom", "settle-me")?, "custom-event");
    cell.payload_schema = "custom.operation".to_string();
    cell.payload_json = serde_json::json!({
        "operation_key": "effect.workspace.append",
        "effect_command": {"name":"workspace.append_text", "path":"native/effect.md", "content":" Second"}
    }).to_string();
    cell.created_at = "now".to_string();
    cell.updated_at = "now".to_string();
    Ok(cell)
}

fn key(namespace: &str, name: &str) -> Result<StateKey, String> {
    StateKey::new(namespace, name).map_err(|error| error.message)
}

fn cell_status(conn: &Connection) -> TestResult<String> {
    Ok(conn.query_row(
        "SELECT status FROM state_cells WHERE key_label = 'custom:settle-me'",
        [],
        |row| row.get(0),
    )?)
}

fn count_rows(conn: &Connection, table: &str) -> TestResult<i64> {
    Ok(
        conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
            row.get(0)
        })?,
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
