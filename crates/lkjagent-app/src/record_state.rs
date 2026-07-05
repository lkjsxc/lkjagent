use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_core::workspace_record::{state_keys_for_record, WorkspaceRecord};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use lkjagent_store::record_rows::RecordRow;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

const CASE_ID: &str = "workspace";

pub fn upsert_record_cells(
    conn: &Connection,
    record: &WorkspaceRecord,
    path: &str,
    fingerprint: &str,
) -> Result<(), String> {
    insert_case(conn, CASE_ID, "workspace records", &record.updated_at)
        .map_err(|error| error.to_string())?;
    for label in &record.state_keys {
        let mut cell = record_cell(record, label, path, fingerprint)?;
        let event_id = next_event_id(conn, CASE_ID, "record-state").map_err(|e| e.to_string())?;
        cell.source_event_id = event_id.clone();
        let event = RuntimeEvent {
            id: event_id,
            case_id: CASE_ID.to_string(),
            kind: "state.cell.upsert".to_string(),
            payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
            source: "workspace-record".to_string(),
            created_at: record.updated_at.clone(),
            decision_id: None,
        };
        append_and_apply_event(conn, &event).map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn suppress_record_cells(conn: &Connection, row: &RecordRow, now: &str) -> Result<(), String> {
    insert_case(conn, CASE_ID, "workspace records", now).map_err(|error| error.to_string())?;
    for label in state_keys_for_record(&row.kind, &row.id, &row.state) {
        let key = StateKey::from_label(&label).map_err(|error| error.message)?;
        let event_id = next_event_id(conn, CASE_ID, "record-state").map_err(|e| e.to_string())?;
        let event = RuntimeEvent {
            id: event_id,
            case_id: CASE_ID.to_string(),
            kind: "state.cell.suppress".to_string(),
            payload: RuntimeEventPayload::SuppressCell {
                key,
                reason: "record archived".to_string(),
            },
            source: "workspace-record".to_string(),
            created_at: now.to_string(),
            decision_id: None,
        };
        append_and_apply_event(conn, &event).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn record_cell(
    record: &WorkspaceRecord,
    label: &str,
    path: &str,
    fingerprint: &str,
) -> Result<StateCell, String> {
    let mut cell = StateCell::active(
        StateKey::from_label(label).map_err(|error| error.message)?,
        "pending-record-event",
    );
    cell.payload_schema = "workspace-record".to_string();
    cell.payload_json = serde_json::json!({
        "record_id": record.id,
        "kind": record.kind,
        "title": record.title,
        "state": record.state,
        "path": path,
        "selector_tier": selector_tier(label),
    })
    .to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "record".to_string(),
        source_id: record.id.clone(),
        fingerprint: fingerprint.to_string(),
    }];
    cell.created_at = record.created_at.clone();
    cell.updated_at = record.updated_at.clone();
    Ok(cell)
}

fn selector_tier(label: &str) -> u8 {
    match label.split_once(':').map(|(namespace, _)| namespace) {
        Some("todo") => 35,
        Some("calendar") => 36,
        Some("routine") => 37,
        Some("index") => 38,
        Some("proof") => 39,
        Some("dev") => 40,
        Some("project") => 41,
        _ => 80,
    }
}
