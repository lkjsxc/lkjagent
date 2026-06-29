use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::dense_model::DenseRuntimeRowInput;

pub fn record_dense_runtime_row(
    conn: &Connection,
    row: &DenseRuntimeRowInput<'_>,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO runtime_dense_rows
         (decision_id, row_kind, subject, predicate, object, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            row.decision_id,
            row.row_kind,
            row.subject,
            row.predicate,
            row.object,
            row.created_at,
        ],
    )?;
    Ok(())
}

pub fn record_dense_runtime_rows(
    conn: &Connection,
    rows: &[DenseRuntimeRowInput<'_>],
) -> StoreResult<()> {
    for row in rows {
        record_dense_runtime_row(conn, row)?;
    }
    Ok(())
}
