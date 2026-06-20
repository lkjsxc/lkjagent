use rusqlite::{params, Transaction};

use crate::error::StoreResult;

use super::MemoryKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryIdentity {
    pub kind: MemoryKind,
    pub title_slug: String,
    pub tags_key: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryWriteDecision {
    Insert { memory_id: i64 },
    SkipDuplicate { existing_id: i64 },
    UpdateExisting { existing_id: i64 },
    MergeWith { ids: Vec<i64> },
}

impl MemoryWriteDecision {
    pub fn id(&self) -> i64 {
        match self {
            MemoryWriteDecision::Insert { memory_id } => *memory_id,
            MemoryWriteDecision::SkipDuplicate { existing_id }
            | MemoryWriteDecision::UpdateExisting { existing_id } => *existing_id,
            MemoryWriteDecision::MergeWith { ids } => ids.first().copied().unwrap_or_default(),
        }
    }
}

pub(super) fn memory_identity(
    kind: MemoryKind,
    title: &str,
    tags: &str,
    content: &str,
) -> MemoryIdentity {
    MemoryIdentity {
        kind,
        title_slug: slug(title),
        tags_key: tags_key(tags),
        content_hash: stable_hash(&normalize_space(content)),
    }
}

pub(super) fn find_duplicate(
    tx: &Transaction<'_>,
    candidate: &MemoryIdentity,
) -> StoreResult<Option<i64>> {
    let mut statement =
        tx.prepare("SELECT id, title, tags, content FROM memory WHERE kind = ?1 ORDER BY id")?;
    let mut rows = statement.query(params![candidate.kind.as_str()])?;
    while let Some(row) = rows.next()? {
        let id = row.get(0)?;
        let title: String = row.get(1)?;
        let tags: String = row.get(2)?;
        let content: String = row.get(3)?;
        if memory_identity(candidate.kind, &title, &tags, &content) == *candidate {
            return Ok(Some(id));
        }
    }
    Ok(None)
}

fn tags_key(tags: &str) -> String {
    let mut tags = tags
        .split(',')
        .map(slug)
        .filter(|tag| !tag.is_empty())
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    tags.join(",")
}

fn slug(value: &str) -> String {
    value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("-")
}

fn normalize_space(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}
