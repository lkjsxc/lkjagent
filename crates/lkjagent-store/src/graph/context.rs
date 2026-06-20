use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_context_binding(
    conn: &Connection,
    case_id: i64,
    package: &str,
    reason: &str,
    priority: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_context_bindings
         (case_id, package, reason, priority, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![case_id, package, reason, priority, now],
    )?;
    Ok(())
}
