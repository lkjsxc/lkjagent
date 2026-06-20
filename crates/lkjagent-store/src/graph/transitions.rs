use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_transition(
    conn: &Connection,
    case_id: i64,
    from_node: &str,
    to_node: &str,
    decision: &str,
    reason: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_transitions
         (case_id, from_node, to_node, decision, reason, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![case_id, from_node, to_node, decision, reason, now],
    )?;
    Ok(())
}
