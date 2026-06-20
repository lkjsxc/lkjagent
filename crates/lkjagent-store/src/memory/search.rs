use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::row::rows_from_statement;
use super::MemoryRow;

pub fn find(conn: &Connection, query: &str, limit: i64) -> StoreResult<Vec<MemoryRow>> {
    let Some(query) = normalize_fts_query(query) else {
        return Ok(Vec::new());
    };
    let mut statement = conn.prepare(
        "SELECT m.id, m.kind, m.title, m.tags, m.content, m.tokens, m.updated_at
         FROM memory_fts
         JOIN memory m ON m.id = memory_fts.rowid
         LEFT JOIN (
           SELECT links.memory_id,
             MAX(CASE WHEN cases.status = 'active' THEN 1 ELSE 0 END) AS active_link,
             MAX(CASE WHEN cases.status = 'active'
               AND lower(links.node) = lower(cases.active_node) THEN 1 ELSE 0 END)
               AS active_node_link
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

pub fn normalize_fts_query(raw: &str) -> Option<String> {
    let mut seen = Vec::new();
    let tokens = raw
        .split(|ch: char| !ch.is_alphanumeric())
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_ascii_lowercase)
        .filter(|token| {
            if seen.iter().any(|seen_token| seen_token == token) {
                false
            } else {
                seen.push(token.clone());
                true
            }
        })
        .collect::<Vec<_>>();
    (!tokens.is_empty()).then(|| tokens.join(" "))
}
