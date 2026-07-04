use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::Connection;

pub fn resolve_conflict(
    conn: &Connection,
    case_id: &str,
    semantic_key: &str,
    winning_item_id: &str,
    now: &str,
) -> Result<String, String> {
    let event_id =
        next_event_id(conn, case_id, "context-resolve").map_err(|error| error.to_string())?;
    let mut cell = StateCell::active(resolve_key(semantic_key)?, event_id.clone());
    cell.payload_schema = "context-resolution.v1".to_string();
    cell.payload_json = serde_json::json!({
        "semantic_key": semantic_key,
        "winning_item_id": winning_item_id,
    })
    .to_string();
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    let event = RuntimeEvent {
        id: event_id,
        case_id: case_id.to_string(),
        kind: "context.resolve".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "owner-cli".to_string(),
        created_at: now.to_string(),
        decision_id: None,
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())?;
    Ok(format!(
        "context resolved: {semantic_key} -> {winning_item_id}"
    ))
}

fn resolve_key(semantic_key: &str) -> Result<StateKey, String> {
    StateKey::new("context", format!("resolve/{semantic_key}")).map_err(|error| error.message)
}
