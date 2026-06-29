use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::dense_model::{DenseRuntimePacket, DenseRuntimeRow};

pub fn dense_packet_for_decision(
    conn: &Connection,
    decision_id: i64,
) -> StoreResult<DenseRuntimePacket> {
    let rows = dense_rows_for_decision(conn, decision_id)?;
    Ok(DenseRuntimePacket {
        decision_id,
        facts: filter(&rows, "fact"),
        obligations: filter(&rows, "obligation"),
        resolver_plans: filter(&rows, "resolver_plan"),
        progress: filter(&rows, "progress"),
        completion_inputs: filter(&rows, "completion_input"),
    })
}

pub fn dense_rows_for_decision(
    conn: &Connection,
    decision_id: i64,
) -> StoreResult<Vec<DenseRuntimeRow>> {
    let mut statement = conn.prepare(
        "SELECT id, decision_id, row_kind, subject, predicate, object
         FROM runtime_dense_rows WHERE decision_id = ?1 ORDER BY id",
    )?;
    let rows = statement.query_map(params![decision_id], dense_row)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

fn dense_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DenseRuntimeRow> {
    Ok(DenseRuntimeRow {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        row_kind: row.get(2)?,
        subject: row.get(3)?,
        predicate: row.get(4)?,
        object: row.get(5)?,
    })
}

fn filter(rows: &[DenseRuntimeRow], kind: &str) -> Vec<DenseRuntimeRow> {
    rows.iter()
        .filter(|row| row.row_kind == kind)
        .cloned()
        .collect()
}
