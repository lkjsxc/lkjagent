mod digest;
mod duplicate;
mod identity;
mod prune;
mod row;
mod search;

use rusqlite::{params, Connection, Transaction};

pub use digest::digest;
pub use identity::{MemoryIdentity, MemoryWriteDecision};
pub use prune::{prune_exact_duplicates, MemoryPruneReport};
pub use row::MemoryRow;
pub use search::{find, normalize_fts_query};

use crate::error::StoreResult;
use row::get_required;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "lesson" => Some(Self::Lesson),
            "fact" => Some(Self::Fact),
            "task-summary" => Some(Self::TaskSummary),
            "incident" => Some(Self::Incident),
            _ => None,
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
    Ok(save_decision(conn, kind, title, tags, content, tokens, now)?.id())
}

pub fn save_decision(
    conn: &mut Connection,
    kind: MemoryKind,
    title: &str,
    tags: &str,
    content: &str,
    tokens: i64,
    now: &str,
) -> StoreResult<MemoryWriteDecision> {
    let identity = identity::memory_identity(kind, title, tags, content);
    let tx = conn.transaction()?;
    if let Some(duplicate) = identity::find_duplicate(&tx, &identity, content)? {
        match duplicate {
            identity::DuplicateMatch::Exact { existing_id } => {
                tx.commit()?;
                return Ok(MemoryWriteDecision::SkipDuplicate { existing_id });
            }
            identity::DuplicateMatch::Similar { existing_id } => {
                duplicate::update_similar(
                    &tx,
                    existing_id,
                    MemoryUpdate {
                        kind,
                        title,
                        tags,
                        content,
                        tokens,
                    },
                    now,
                )?;
                tx.commit()?;
                return Ok(MemoryWriteDecision::UpdateExisting { existing_id });
            }
        }
    }
    tx.execute(
        "INSERT INTO memory (kind, title, tags, content, tokens, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
        params![kind.as_str(), title, tags, content, tokens, now],
    )?;
    let id = tx.last_insert_rowid();
    insert_fts(&tx, id, title, tags, content)?;
    tx.commit()?;
    Ok(MemoryWriteDecision::Insert { memory_id: id })
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

pub(super) fn insert_fts(
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

pub(super) fn delete_fts(tx: &Transaction<'_>, row: &MemoryRow) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO memory_fts (memory_fts, rowid, title, tags, content)
         VALUES ('delete', ?1, ?2, ?3, ?4)",
        params![row.id, row.title, row.tags, row.content],
    )?;
    Ok(())
}
