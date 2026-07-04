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
             WHERE key_label = 'case:snapshot' AND status = 'Active'
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
    let mut cell = StateCell::active(key()?, event_id.clone());
    cell.payload_schema = "task-snapshot.v1".to_string();
    cell.payload_json = serde_json::to_string(snapshot).map_err(|error| error.to_string())?;
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "task".to_string(),
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

fn key() -> Result<StateKey, String> {
    StateKey::new("case", "snapshot").map_err(|error| error.message)
}
