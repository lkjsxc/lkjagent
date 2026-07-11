mod query;

pub use query::search;

use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchChunk {
    pub id: String,
    pub document_id: String,
    pub revision_fingerprint: String,
    pub path: String,
    pub field: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub kind: String,
    pub state: String,
    pub project: String,
    pub effective_date: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchFilter {
    pub kind: Option<String>,
    pub state: Option<String>,
    pub project: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    Lexical,
    Trigram,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub chunk: SearchChunk,
    pub score: f64,
}

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS workspace_search_chunks (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            revision_fingerprint TEXT NOT NULL,
            path TEXT NOT NULL,
            field TEXT NOT NULL,
            start_byte INTEGER NOT NULL,
            end_byte INTEGER NOT NULL,
            kind TEXT NOT NULL,
            state TEXT NOT NULL,
            project TEXT NOT NULL,
            effective_date TEXT NOT NULL,
            content TEXT NOT NULL,
            UNIQUE(document_id, revision_fingerprint, field, start_byte, end_byte)
        );
        CREATE INDEX IF NOT EXISTS idx_workspace_search_predicates
            ON workspace_search_chunks(kind, state, project, effective_date, document_id);
        CREATE VIRTUAL TABLE IF NOT EXISTS workspace_search_lexical
            USING fts5(chunk_id UNINDEXED, content, tokenize = 'unicode61');
        CREATE VIRTUAL TABLE IF NOT EXISTS workspace_search_trigram
            USING fts5(chunk_id UNINDEXED, content, tokenize = 'trigram');
        ",
    )?;
    Ok(())
}

pub fn replace_chunks(conn: &Connection, chunks: &[SearchChunk]) -> StoreResult<()> {
    let tx = conn.unchecked_transaction()?;
    tx.execute("DELETE FROM workspace_search_lexical", [])?;
    tx.execute("DELETE FROM workspace_search_trigram", [])?;
    tx.execute("DELETE FROM workspace_search_chunks", [])?;
    for chunk in chunks {
        insert_chunk(&tx, chunk)?;
        insert_fts(&tx, "workspace_search_lexical", chunk)?;
        insert_fts(&tx, "workspace_search_trigram", chunk)?;
    }
    tx.commit()?;
    Ok(())
}

pub fn canonical_rows(conn: &Connection) -> StoreResult<Vec<SearchChunk>> {
    let mut statement = conn.prepare(
        "SELECT id, document_id, revision_fingerprint, path, field, start_byte,
         end_byte, kind, state, project, effective_date, content
         FROM workspace_search_chunks
         ORDER BY document_id, field, start_byte, end_byte, id",
    )?;
    let rows = statement.query_map([], row)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

fn insert_chunk(tx: &rusqlite::Transaction<'_>, chunk: &SearchChunk) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO workspace_search_chunks (
         id, document_id, revision_fingerprint, path, field, start_byte, end_byte,
         kind, state, project, effective_date, content
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            chunk.id,
            chunk.document_id,
            chunk.revision_fingerprint,
            chunk.path,
            chunk.field,
            chunk.start_byte as i64,
            chunk.end_byte as i64,
            chunk.kind,
            chunk.state,
            chunk.project,
            chunk.effective_date,
            chunk.content,
        ],
    )?;
    Ok(())
}

fn insert_fts(tx: &rusqlite::Transaction<'_>, table: &str, chunk: &SearchChunk) -> StoreResult<()> {
    let statement = format!("INSERT INTO {table} (chunk_id, content) VALUES (?1, ?2)");
    tx.execute(&statement, params![chunk.id, chunk.content])?;
    Ok(())
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SearchChunk> {
    Ok(SearchChunk {
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
    })
}
