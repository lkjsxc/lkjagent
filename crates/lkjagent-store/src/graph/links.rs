use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphMemoryLinkRow {
    pub case_id: i64,
    pub memory_id: i64,
    pub node: String,
    pub reason: String,
}

pub fn link_memory(
    conn: &Connection,
    case_id: i64,
    memory_id: i64,
    node: &str,
    reason: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO graph_memory_links
         (case_id, memory_id, node, reason, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![case_id, memory_id, node, reason, now],
    )?;
    Ok(())
}

pub fn memory_links_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Vec<GraphMemoryLinkRow>> {
    let mut statement = conn.prepare(
        "SELECT case_id, memory_id, node, reason
         FROM graph_memory_links
         WHERE case_id = ?1
         ORDER BY memory_id, node",
    )?;
    let rows = statement.query_map(params![case_id], |row| {
        Ok(GraphMemoryLinkRow {
            case_id: row.get(0)?,
            memory_id: row.get(1)?,
            node: row.get(2)?,
            reason: row.get(3)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}
