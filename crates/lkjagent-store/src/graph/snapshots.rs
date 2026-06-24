use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionSnapshotRow {
    pub id: i64,
    pub case_id: i64,
    pub phase: String,
    pub active_node: String,
    pub objective: String,
    pub preserved_fields: String,
    pub created_at: String,
}

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

pub fn latest_compaction_snapshot(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<CompactionSnapshotRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, phase, active_node, objective, preserved_fields, created_at
         FROM graph_compaction_snapshots WHERE case_id = ?1 ORDER BY id DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![case_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(CompactionSnapshotRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        phase: row.get(2)?,
        active_node: row.get(3)?,
        objective: row.get(4)?,
        preserved_fields: row.get(5)?,
        created_at: row.get(6)?,
    }))
}
