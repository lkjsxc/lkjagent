use rusqlite::{params, OptionalExtension, Transaction};

use crate::error::{StoreError, StoreResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRow {
    pub id: i64,
    pub kind: String,
    pub title: String,
    pub tags: String,
    pub content: String,
    pub tokens: i64,
    pub updated_at: String,
}

pub(super) fn get_required(tx: &Transaction<'_>, id: i64) -> StoreResult<MemoryRow> {
    let row = tx
        .query_row(
            "SELECT id, kind, title, tags, content, tokens, updated_at FROM memory WHERE id = ?1",
            params![id],
            row_from_sql,
        )
        .optional()?;
    row.ok_or_else(|| StoreError::NotFound(format!("memory row {id}")))
}

pub(super) fn rows_from_statement<P>(
    statement: &mut rusqlite::Statement<'_>,
    params: P,
) -> StoreResult<Vec<MemoryRow>>
where
    P: rusqlite::Params,
{
    let rows = statement.query_map(params, row_from_sql)?;
    let mut memory = Vec::new();
    for row in rows {
        memory.push(row?);
    }
    Ok(memory)
}

fn row_from_sql(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryRow> {
    Ok(MemoryRow {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        tags: row.get(3)?,
        content: row.get(4)?,
        tokens: row.get(5)?,
        updated_at: row.get(6)?,
    })
}
