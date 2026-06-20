use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_artifact(
    conn: &Connection,
    case_id: i64,
    path: &str,
    kind: &str,
    status: &str,
    summary: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_artifacts
         (case_id, path, kind, status, summary, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![case_id, path, kind, status, summary, now],
    )?;
    Ok(())
}
