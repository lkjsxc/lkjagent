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

pub fn upsert_fault_retry(
    conn: &Connection,
    key: &FaultRetryKey<'_>,
    count: u8,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_fault_retries
         (case_id, node, tool, parameter_shape, fault_class, count, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(case_id, node, tool, parameter_shape, fault_class)
         DO UPDATE SET count = excluded.count, updated_at = excluded.updated_at",
        params![
            key.case_id,
            key.node,
            key.tool,
            key.parameter_shape,
            key.fault_class,
            i64::from(count),
            now
        ],
    )?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultRetryKey<'a> {
    pub case_id: i64,
    pub node: &'a str,
    pub tool: &'a str,
    pub parameter_shape: &'a str,
    pub fault_class: &'a str,
}

pub fn retry_count(conn: &Connection, key: &FaultRetryKey<'_>) -> StoreResult<Option<u8>> {
    let mut statement = conn.prepare(
        "SELECT count FROM graph_fault_retries
         WHERE case_id = ?1 AND node = ?2 AND tool = ?3
         AND parameter_shape = ?4 AND fault_class = ?5",
    )?;
    let mut rows = statement.query(params![
        key.case_id,
        key.node,
        key.tool,
        key.parameter_shape,
        key.fault_class
    ])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    let value: i64 = row.get(0)?;
    Ok(u8::try_from(value).ok())
}

pub fn upsert_recovery_state(
    conn: &Connection,
    case_id: i64,
    ladder_position: u8,
    strategy: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_recovery_state
         (case_id, ladder_position, strategy, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(case_id) DO UPDATE SET
         ladder_position = excluded.ladder_position,
         strategy = excluded.strategy,
         updated_at = excluded.updated_at",
        params![case_id, i64::from(ladder_position), strategy, now],
    )?;
    Ok(())
}
