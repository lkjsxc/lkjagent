use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn record_fault(
    conn: &Connection,
    case_id: i64,
    kind: &str,
    action_fingerprint: Option<&str>,
    summary: &str,
    count: u8,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_faults
         (case_id, kind, action_fingerprint, summary, count, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            case_id,
            kind,
            action_fingerprint,
            summary,
            i64::from(count),
            now
        ],
    )?;
    Ok(())
}
