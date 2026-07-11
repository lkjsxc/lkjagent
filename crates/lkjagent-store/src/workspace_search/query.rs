use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection};

use crate::error::{StoreError, StoreResult};

use super::{SearchChunk, SearchFilter, SearchHit, SearchMode};

pub fn search(
    conn: &Connection,
    text: &str,
    filter: &SearchFilter,
    mode: SearchMode,
    limit: usize,
    offset: usize,
) -> StoreResult<Vec<SearchHit>> {
    let table = table(mode);
    let mut values = vec![Value::Text(fts_query(text)?)];
    let mut predicates = Vec::new();
    predicate(&mut predicates, &mut values, "kind", &filter.kind);
    predicate(&mut predicates, &mut values, "state", &filter.state);
    predicate(&mut predicates, &mut values, "project", &filter.project);
    predicate(&mut predicates, &mut values, "effective_date", &filter.date);
    values.push(Value::Integer(limit.min(50) as i64));
    values.push(Value::Integer(offset as i64));
    let where_clause = if predicates.is_empty() {
        String::new()
    } else {
        format!(" AND {}", predicates.join(" AND "))
    };
    let sql = format!(
        "SELECT c.id, c.document_id, c.revision_fingerprint, c.path, c.field,
         c.start_byte, c.end_byte, c.kind, c.state, c.project, c.effective_date,
         c.content, bm25({table})
         FROM {table} JOIN workspace_search_chunks c ON c.id = {table}.chunk_id
         WHERE {table} MATCH ? {where_clause}
         ORDER BY bm25({table}), c.document_id, c.field, c.start_byte, c.end_byte, c.id LIMIT ? OFFSET ?"
    );
    let mut statement = conn.prepare(&sql)?;
    let rows = statement.query_map(params_from_iter(values.iter()), row)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

fn table(mode: SearchMode) -> &'static str {
    match mode {
        SearchMode::Lexical => "workspace_search_lexical",
        SearchMode::Trigram => "workspace_search_trigram",
    }
}

fn fts_query(text: &str) -> StoreResult<String> {
    let terms = text
        .split_whitespace()
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>();
    if terms.is_empty() {
        Err(StoreError::InvalidState(
            "search query is empty".to_string(),
        ))
    } else {
        Ok(terms.join(" AND "))
    }
}

fn predicate(
    predicates: &mut Vec<String>,
    values: &mut Vec<Value>,
    column: &str,
    value: &Option<String>,
) {
    if let Some(value) = value {
        predicates.push(format!("c.{column} = ?"));
        values.push(Value::Text(value.clone()));
    }
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SearchHit> {
    Ok(SearchHit {
        chunk: SearchChunk {
            id: row.get(0)?,
            document_id: row.get(1)?,
            revision_fingerprint: row.get(2)?,
            path: row.get(3)?,
            field: row.get(4)?,
            start_byte: row.get::<_, i64>(5)? as usize,
            end_byte: row.get::<_, i64>(6)? as usize,
            kind: row.get(7)?,
            state: row.get(8)?,
            project: row.get(9)?,
            effective_date: row.get(10)?,
            content: row.get(11)?,
        },
        score: row.get(12)?,
    })
}
