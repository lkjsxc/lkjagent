use lkjagent_core::model::CheckResult;
use lkjagent_core::runtime_event::{reduce_event, RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::row_json::json_string;
use crate::state_rows::{hydrate_snapshot, persist_state_patch};

pub fn next_event_id(conn: &Connection, case_id: &str, prefix: &str) -> StoreResult<String> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM runtime_events WHERE case_id = ?1",
        [case_id],
        |row| row.get(0),
    )?;
    Ok(format!("{prefix}-{case_id}-{:04}", count + 1))
}

pub fn insert_event(conn: &Connection, event: &RuntimeEvent) -> StoreResult<bool> {
    let payload_json = json_string(&event.payload)?;
    let event_json = json_string(event)?;
    let inserted = conn.execute(
        "INSERT OR IGNORE INTO runtime_events
         (id, case_id, kind, payload_json, source, decision_id, event_json,
          created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            event.id,
            event.case_id,
            event.kind,
            payload_json,
            event.source,
            event.decision_id,
            event_json,
            event.created_at,
        ],
    )?;
    Ok(inserted == 1)
}

pub fn append_and_apply_event(conn: &Connection, event: &RuntimeEvent) -> StoreResult<()> {
    conn.execute_batch("BEGIN IMMEDIATE")?;
    let result = append_and_apply_event_tx(conn, event);
    if result.is_ok() {
        conn.execute_batch("COMMIT")?;
    } else {
        let _ = conn.execute_batch("ROLLBACK");
    }
    result
}

pub fn append_check_result_cell_tx(
    conn: &Connection,
    case_id: &str,
    step_id: u64,
    row_id: i64,
    result: &CheckResult,
    now: &str,
) -> StoreResult<()> {
    ensure_case(conn, case_id, now)?;
    let state = if result.passed {
        "check-passed"
    } else {
        "check-failed"
    };
    let event_id = next_event_id(conn, case_id, "state-check")?;
    let mut cell = StateCell::active(check_key(state, step_id, row_id)?, event_id.clone());
    cell.payload_schema = "state.completion-check-outcome".to_string();
    cell.payload_json = json_string(&serde_json::json!({
        "step_id": step_id,
        "check_result_id": row_id,
        "name": result.name,
        "passed": result.passed,
        "measured": result.measured,
        "artifact_refs": result.artifact_refs,
    }))?;
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "check_result".to_string(),
        source_id: row_id.to_string(),
        fingerprint: result
            .evidence_fingerprint
            .clone()
            .unwrap_or_else(|| result.measured.clone()),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    let event = RuntimeEvent {
        id: event_id,
        case_id: case_id.to_string(),
        kind: "state.cell.upsert".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "check-result".to_string(),
        created_at: now.to_string(),
        decision_id: result.decision_id.clone(),
    };
    append_and_apply_event_tx(conn, &event)
}

fn ensure_case(conn: &Connection, case_id: &str, now: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT OR IGNORE INTO cases
         (id, objective, lifecycle, summary, created_at, updated_at)
         VALUES (?1, COALESCE((SELECT objective FROM tasks WHERE id = CAST(?1 AS INTEGER)), ?2),
                 'open', '', ?3, ?3)",
        params![case_id, format!("case {case_id}"), now],
    )?;
    Ok(())
}

fn check_key(state: &str, step_id: u64, row_id: i64) -> StoreResult<StateKey> {
    StateKey::new("completion", format!("{state}/{step_id}/{row_id}"))
        .map_err(|error| crate::error::StoreError::InvalidState(error.message))
}

fn append_and_apply_event_tx(conn: &Connection, event: &RuntimeEvent) -> StoreResult<()> {
    if !insert_event(conn, event)? {
        return Ok(());
    }
    let snapshot = hydrate_snapshot(conn, &event.case_id)?;
    let patch = reduce_event(&snapshot, event);
    persist_state_patch(conn, &event.case_id, &patch)
}
