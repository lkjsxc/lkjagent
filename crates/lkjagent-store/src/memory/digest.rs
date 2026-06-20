use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::row::rows_from_statement;
use super::MemoryRow;

pub fn digest(
    conn: &Connection,
    task_summary_id: Option<i64>,
    budget: i64,
) -> StoreResult<Vec<MemoryRow>> {
    let mut selected = Vec::new();
    let mut remaining = budget;
    if let Some(row) = task_summary_id
        .and_then(|id| get(conn, id).transpose())
        .transpose()?
    {
        if row.tokens <= remaining {
            remaining -= row.tokens;
            selected.push(row);
        }
    }
    let mut statement = conn.prepare(
        "SELECT id, kind, title, tags, content, tokens, updated_at
         FROM memory
         ORDER BY
           CASE kind
             WHEN 'task-summary' THEN 4
             WHEN 'incident' THEN 3
             WHEN 'lesson' THEN 2
             ELSE 1
           END DESC,
           updated_at DESC",
    )?;
    for row in rows_from_statement(&mut statement, [])? {
        if Some(row.id) != task_summary_id && row.tokens <= remaining {
            remaining -= row.tokens;
            selected.push(row);
        }
    }
    Ok(selected)
}

fn get(conn: &Connection, id: i64) -> StoreResult<Option<MemoryRow>> {
    let mut statement = conn.prepare(
        "SELECT id, kind, title, tags, content, tokens, updated_at FROM memory WHERE id = ?1",
    )?;
    let mut rows = rows_from_statement(&mut statement, params![id])?;
    Ok(rows.pop())
}
