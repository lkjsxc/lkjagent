mod row;

use rusqlite::{params, Connection, Transaction};

pub use row::MemoryRow;

use crate::error::StoreResult;
use row::{get_required, rows_from_statement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Lesson,
    Fact,
    TaskSummary,
    Incident,
}

impl MemoryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            MemoryKind::Lesson => "lesson",
            MemoryKind::Fact => "fact",
            MemoryKind::TaskSummary => "task-summary",
            MemoryKind::Incident => "incident",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryUpdate<'a> {
    pub kind: MemoryKind,
    pub title: &'a str,
    pub tags: &'a str,
    pub content: &'a str,
    pub tokens: i64,
}

pub fn save(
    conn: &mut Connection,
    kind: MemoryKind,
    title: &str,
    tags: &str,
    content: &str,
    tokens: i64,
    now: &str,
) -> StoreResult<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO memory (kind, title, tags, content, tokens, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
        params![kind.as_str(), title, tags, content, tokens, now],
    )?;
    let id = tx.last_insert_rowid();
    insert_fts(&tx, id, title, tags, content)?;
    tx.commit()?;
    Ok(id)
}

pub fn update(conn: &mut Connection, id: i64, row: MemoryUpdate<'_>, now: &str) -> StoreResult<()> {
    let tx = conn.transaction()?;
    let before = get_required(&tx, id)?;
    tx.execute(
        "UPDATE memory
         SET kind = ?1, title = ?2, tags = ?3, content = ?4, tokens = ?5, updated_at = ?6
         WHERE id = ?7",
        params![
            row.kind.as_str(),
            row.title,
            row.tags,
            row.content,
            row.tokens,
            now,
            id
        ],
    )?;
    delete_fts(&tx, &before)?;
    insert_fts(&tx, id, row.title, row.tags, row.content)?;
    tx.commit()?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64) -> StoreResult<()> {
    let tx = conn.transaction()?;
    let before = get_required(&tx, id)?;
    delete_fts(&tx, &before)?;
    tx.execute("DELETE FROM memory WHERE id = ?1", params![id])?;
    tx.commit()?;
    Ok(())
}

pub fn find(conn: &Connection, query: &str, limit: i64) -> StoreResult<Vec<MemoryRow>> {
    let mut statement = conn.prepare(
        "SELECT m.id, m.kind, m.title, m.tags, m.content, m.tokens, m.updated_at
         FROM memory_fts
         JOIN memory m ON m.id = memory_fts.rowid
         LEFT JOIN (
           SELECT
             links.memory_id,
             MAX(CASE WHEN cases.status = 'active' THEN 1 ELSE 0 END) AS active_link,
             MAX(
               CASE
                 WHEN cases.status = 'active'
                  AND lower(links.node) = lower(cases.active_node)
                 THEN 1
                 ELSE 0
               END
             ) AS active_node_link
           FROM graph_memory_links links
           JOIN graph_cases cases ON cases.id = links.case_id
           GROUP BY links.memory_id
         ) graph_rank ON graph_rank.memory_id = m.id
         WHERE memory_fts MATCH ?1
         ORDER BY
           CASE m.kind
             WHEN 'task-summary' THEN 4
             WHEN 'incident' THEN 3
             WHEN 'lesson' THEN 2
             ELSE 1
           END DESC,
           COALESCE(graph_rank.active_link, 0) DESC,
           COALESCE(graph_rank.active_node_link, 0) DESC,
           bm25(memory_fts) ASC,
           m.updated_at DESC
         LIMIT ?2",
    )?;
    rows_from_statement(&mut statement, params![query, limit])
}

pub fn digest(
    conn: &Connection,
    task_summary_id: Option<i64>,
    budget: i64,
) -> StoreResult<Vec<MemoryRow>> {
    let mut selected = Vec::new();
    let mut remaining = budget;
    if let Some(id) = task_summary_id {
        if let Some(row) = get(conn, id)? {
            if row.tokens <= remaining {
                remaining -= row.tokens;
                selected.push(row);
            }
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
    let rows = rows_from_statement(&mut statement, [])?;
    for row in rows {
        if Some(row.id) == task_summary_id {
            continue;
        }
        if row.tokens <= remaining {
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

fn insert_fts(
    tx: &Transaction<'_>,
    id: i64,
    title: &str,
    tags: &str,
    content: &str,
) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO memory_fts (rowid, title, tags, content) VALUES (?1, ?2, ?3, ?4)",
        params![id, title, tags, content],
    )?;
    Ok(())
}

fn delete_fts(tx: &Transaction<'_>, row: &MemoryRow) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO memory_fts (memory_fts, rowid, title, tags, content)
         VALUES ('delete', ?1, ?2, ?3, ?4)",
        params![row.id, row.title, row.tags, row.content],
    )?;
    Ok(())
}
