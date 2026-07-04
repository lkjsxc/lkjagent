use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::Connection;

use crate::runtime_cell::projected_cell;

pub fn ensure_runtime_cell(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    state: &RuntimeSnapshot,
    now: &str,
) -> Result<(), String> {
    if has_operation_cell(state) {
        return Ok(());
    }
    let case_id = snapshot.task.id.to_string();
    let cell = projected_cell(snapshot, now)?;
    append_cell_event(conn, &case_id, cell, now, None, "plan-bridge")
}

pub fn suppress_decision_cell(
    conn: &Connection,
    decision: &RuntimeDecision,
    now: &str,
) -> Result<(), String> {
    let Some(key) = decision_cell_key(decision)? else {
        return Ok(());
    };
    let event_id = next_event_id(conn, &decision.case_id, "state-suppress")
        .map_err(|error| error.to_string())?;
    let event = RuntimeEvent {
        id: event_id,
        case_id: decision.case_id.clone(),
        kind: "state.cell.suppress".to_string(),
        payload: RuntimeEventPayload::SuppressCell {
            key,
            reason: "decision settled".to_string(),
        },
        source: "runtime-decision".to_string(),
        created_at: now.to_string(),
        decision_id: Some(decision.id.clone()),
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn append_cell_event(
    conn: &Connection,
    case_id: &str,
    mut cell: StateCell,
    now: &str,
    decision_id: Option<String>,
    source: &str,
) -> Result<(), String> {
    let event_id = next_event_id(conn, case_id, "state-project").map_err(|e| e.to_string())?;
    cell.source_event_id = event_id.clone();
    let event = RuntimeEvent {
        id: event_id,
        case_id: case_id.to_string(),
        kind: "state.cell.upsert".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: source.to_string(),
        created_at: now.to_string(),
        decision_id,
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn has_operation_cell(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.active_cells().iter().any(|cell| {
        matches!(
            cell.key.namespace.as_str(),
            "case" | "recovery" | "effect" | "model" | "check" | "completion" | "runtime"
        )
    })
}

fn decision_cell_key(decision: &RuntimeDecision) -> Result<Option<StateKey>, String> {
    let operation = decision.operation.0.as_str();
    if let Some(step) = operation.strip_prefix("model.call/") {
        return StateKey::new("model", step)
            .map(Some)
            .map_err(|e| e.message);
    }
    if let Some(step) = operation.strip_prefix("check.run/") {
        return StateKey::new("check", step)
            .map(Some)
            .map_err(|e| e.message);
    }
    match operation {
        "completion.close" => StateKey::new("completion", "close-candidate"),
        "owner.answer" => StateKey::new("case", "waiting-answer"),
        "runtime.idle" => StateKey::new("runtime", "idle"),
        _ => return Ok(None),
    }
    .map(Some)
    .map_err(|error| error.message)
}
