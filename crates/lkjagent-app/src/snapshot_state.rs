use lkjagent_core::model::{TaskSnapshot, TaskState};
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

pub fn load_snapshot_cell(conn: &Connection) -> Result<Option<TaskSnapshot>, String> {
    let mut statement = conn
        .prepare(
            "SELECT payload_json FROM state_cells
             WHERE status = 'Active'
             AND (key_label = 'case:snapshot' OR key_label LIKE 'matter:snapshot/%')
             ORDER BY updated_at DESC, case_id DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?;
    for row in rows {
        let snapshot: TaskSnapshot = serde_json::from_str(&row.map_err(|e| e.to_string())?)
            .map_err(|error| error.to_string())?;
        if matches!(snapshot.task.state, TaskState::Open | TaskState::Waiting) {
            return Ok(Some(snapshot));
        }
    }
    Ok(None)
}

pub fn persist_snapshot_cell(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    now: &str,
) -> Result<(), String> {
    let case_id = snapshot.task.id.to_string();
    insert_case(conn, &case_id, &snapshot.task.objective, now).map_err(|e| e.to_string())?;
    let event_id = next_event_id(conn, &case_id, "snapshot").map_err(|e| e.to_string())?;
    let mut cell = StateCell::active(key(&case_id)?, event_id.clone());
    cell.payload_schema = "matter-snapshot".to_string();
    cell.payload_json = serde_json::to_string(snapshot).map_err(|error| error.to_string())?;
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "matter".to_string(),
        source_id: case_id.clone(),
        fingerprint: format!("budget-{}", snapshot.task.budget_used),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    let event = RuntimeEvent {
        id: event_id,
        case_id,
        kind: "case.snapshot".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "daemon".to_string(),
        created_at: now.to_string(),
        decision_id: None,
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn key(case_id: &str) -> Result<StateKey, String> {
    StateKey::new("matter", format!("snapshot/{case_id}")).map_err(|error| error.message)
}

pub fn watch(conn: &Connection) -> Result<String, String> {
    let status = crate::status::status(conn)?;
    let events = crate::log_view::log(conn, 8)?;
    Ok([
        "watch: rerun to refresh; use log --follow to stream".to_string(),
        "== status ==".to_string(),
        status,
        "== transcript ==".to_string(),
        crate::tui_snapshot::transcript(conn, 40)?,
        "== recent events ==".to_string(),
        events,
        "== matter trace ==".to_string(),
        matter_trace(conn)?,
        "== proof rows ==".to_string(),
        proof_line(conn)?,
    ]
    .join("\n"))
}

fn matter_trace(conn: &Connection) -> Result<String, String> {
    if let Some(snapshot) = crate::state::load_snapshot(conn).map_err(|error| error.to_string())? {
        return Ok(crate::status::task_show(&snapshot));
    }
    let Some(id) = latest_task_id(conn)? else {
        return Ok("matter: none".to_string());
    };
    lkjagent_store::plan_hydrate::snapshot_by_id(conn, id)
        .map_err(|error| error.to_string())?
        .map_or_else(
            || Ok("matter: none".to_string()),
            |snapshot| Ok(crate::status::task_show(&snapshot)),
        )
}

fn latest_task_id(conn: &Connection) -> Result<Option<i64>, String> {
    match conn.query_row("SELECT id FROM tasks ORDER BY id DESC LIMIT 1", [], |row| {
        row.get(0)
    }) {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

fn proof_line(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "proof: prompt_frames={} checks={} artifacts={} exchanges={}",
        count(conn, "prompt_frames")?,
        count(conn, "check_results")?,
        count(conn, "artifacts")?,
        count(conn, "provider_exchanges")?
    ))
}

fn count(conn: &Connection, table: &str) -> Result<i64, String> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .map_err(|error| error.to_string())
}
