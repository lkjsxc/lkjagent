use lkjagent_core::runtime_event::{reduce_event, RuntimeEvent};
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

pub fn insert_event(conn: &Connection, event: &RuntimeEvent) -> StoreResult<()> {
    let payload_json = json_string(&event.payload)?;
    let event_json = json_string(event)?;
    conn.execute(
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
    Ok(())
}

pub fn append_and_apply_event(conn: &Connection, event: &RuntimeEvent) -> StoreResult<()> {
    insert_event(conn, event)?;
    let snapshot = hydrate_snapshot(conn, &event.case_id)?;
    let patch = reduce_event(&snapshot, event);
    persist_state_patch(conn, &event.case_id, &patch)
}
