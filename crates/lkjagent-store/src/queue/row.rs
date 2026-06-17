use rusqlite::{params, OptionalExtension, Transaction};

use crate::error::{StoreError, StoreResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueRow {
    pub id: i64,
    pub source_queue_id: Option<i64>,
    pub content: String,
    pub status: String,
    pub delivered_turn: Option<i64>,
}

pub(super) fn next_pending(tx: &Transaction<'_>) -> StoreResult<Option<QueueRow>> {
    let row = tx
        .query_row(
            "SELECT id, source_queue_id, content, status, delivered_turn
             FROM queue WHERE status = 'pending' ORDER BY id LIMIT 1",
            [],
            row_from_sql,
        )
        .optional()?;
    Ok(row)
}

pub(super) fn pending_content(tx: &Transaction<'_>, id: i64) -> StoreResult<String> {
    let content = tx
        .query_row(
            "SELECT content FROM queue WHERE id = ?1 AND status = 'pending'",
            params![id],
            |row| row.get(0),
        )
        .optional()?;
    content.ok_or_else(|| StoreError::InvalidState(format!("queue row {id} is not pending")))
}

pub(super) fn queue_content(tx: &Transaction<'_>, id: i64) -> StoreResult<String> {
    let content = tx
        .query_row(
            "SELECT content FROM queue WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .optional()?;
    content.ok_or_else(|| StoreError::NotFound(format!("queue row {id}")))
}

pub(super) fn rows_from_statement<P>(
    statement: &mut rusqlite::Statement<'_>,
    params: P,
) -> StoreResult<Vec<QueueRow>>
where
    P: rusqlite::Params,
{
    let rows = statement.query_map(params, row_from_sql)?;
    let mut queue = Vec::new();
    for row in rows {
        queue.push(row?);
    }
    Ok(queue)
}

fn row_from_sql(row: &rusqlite::Row<'_>) -> rusqlite::Result<QueueRow> {
    Ok(QueueRow {
        id: row.get(0)?,
        source_queue_id: row.get(1)?,
        content: row.get(2)?,
        status: row.get(3)?,
        delivered_turn: row.get(4)?,
    })
}
