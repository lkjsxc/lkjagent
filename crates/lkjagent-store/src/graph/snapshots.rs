use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_compaction_snapshot(
    conn: &Connection,
    case_id: i64,
    phase: &str,
    active_node: &str,
    objective: &str,
    preserved_fields: &[String],
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO graph_compaction_snapshots
         (case_id, phase, active_node, objective, preserved_fields, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            case_id,
            phase,
            active_node,
            objective,
            preserved_fields.join("\n"),
            now
        ],
    )?;
    Ok(conn.last_insert_rowid())
}
