use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRow {
    pub id: i64,
    pub topic: String,
    pub content: String,
    pub task_id: Option<i64>,
}

pub fn insert_memory_tx(
    tx: &Connection,
    topic: &str,
    content: &str,
    task_id: i64,
    now: &str,
) -> StoreResult<bool> {
    let changed = tx.execute(
        "INSERT INTO memory (topic, content, task_id, created_at)
         SELECT ?1, ?2, ?3, ?4
         WHERE NOT EXISTS (
             SELECT 1 FROM memory WHERE topic = ?1 AND content = ?2
         )",
        params![topic, content, task_id, now],
    )?;
    Ok(changed > 0)
}

pub fn search_memory(conn: &Connection, query: &str, limit: usize) -> StoreResult<Vec<MemoryRow>> {
    if query.trim().is_empty() || limit == 0 {
        return Ok(Vec::new());
    }
    let fts = fts_query(query);
    if !fts.is_empty() {
        match search_fts(conn, &fts, limit) {
            Ok(rows) if !rows.is_empty() => return Ok(rows),
            Ok(_) => {}
            Err(_) => {}
        }
    }
    search_like(conn, query, limit)
}

fn search_fts(conn: &Connection, query: &str, limit: usize) -> StoreResult<Vec<MemoryRow>> {
    let mut statement = conn.prepare(
        "SELECT m.id, m.topic, m.content, m.task_id
         FROM memory_fts JOIN memory m ON m.id = memory_fts.rowid
         WHERE memory_fts MATCH ?1
         ORDER BY bm25(memory_fts), m.id
         LIMIT ?2",
    )?;
    let rows = statement.query_map(params![query, limit as i64], memory_row)?;
    collect(rows)
}

fn search_like(conn: &Connection, query: &str, limit: usize) -> StoreResult<Vec<MemoryRow>> {
    let pattern = format!("%{}%", escape_like(query.trim()));
    let mut statement = conn.prepare(
        "SELECT id, topic, content, task_id FROM memory
         WHERE topic LIKE ?1 ESCAPE '\\' OR content LIKE ?1 ESCAPE '\\'
         ORDER BY id LIMIT ?2",
    )?;
    let rows = statement.query_map(params![pattern, limit as i64], memory_row)?;
    collect(rows)
}

fn memory_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryRow> {
    Ok(MemoryRow {
        id: row.get(0)?,
        topic: row.get(1)?,
        content: row.get(2)?,
        task_id: row.get(3)?,
    })
}

fn collect(rows: impl Iterator<Item = rusqlite::Result<MemoryRow>>) -> StoreResult<Vec<MemoryRow>> {
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

fn fts_query(query: &str) -> String {
    query
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .take(8)
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" OR ")
}

fn escape_like(query: &str) -> String {
    let mut escaped = String::new();
    for ch in query.chars() {
        if matches!(ch, '%' | '_' | '\\') {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}
