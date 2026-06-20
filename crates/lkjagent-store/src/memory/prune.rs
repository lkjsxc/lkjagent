use std::collections::{BTreeMap, BTreeSet};

use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::identity::{memory_identity, MemoryIdentity};
use super::row::{get_required, rows_from_statement, MemoryRow};
use super::{delete_fts, insert_fts, MemoryKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPruneReport {
    pub kept: usize,
    pub deleted: usize,
    pub merged: usize,
    pub source_rows: Vec<i64>,
}

pub fn prune_exact_duplicates(conn: &mut Connection) -> StoreResult<MemoryPruneReport> {
    let tx = conn.transaction()?;
    let mut report = MemoryPruneReport {
        kept: 0,
        deleted: 0,
        merged: 0,
        source_rows: Vec::new(),
    };
    let groups = duplicate_groups(&tx)?;
    for ids in groups.values().filter(|ids| ids.len() > 1) {
        report.kept = report.kept.saturating_add(1);
        for id in ids.iter().skip(1) {
            let row = get_required(&tx, *id)?;
            delete_fts(&tx, &row)?;
            tx.execute("DELETE FROM memory WHERE id = ?1", params![id])?;
            report.deleted = report.deleted.saturating_add(1);
        }
    }
    semantic_merge(&tx, &mut report)?;
    tx.commit()?;
    Ok(report)
}

fn duplicate_groups(
    tx: &rusqlite::Transaction<'_>,
) -> StoreResult<BTreeMap<MemoryIdentity, Vec<i64>>> {
    let mut statement = tx.prepare(
        "SELECT id, kind, title, tags, content, tokens, updated_at
         FROM memory
         ORDER BY id",
    )?;
    let mut groups: BTreeMap<MemoryIdentity, Vec<i64>> = BTreeMap::new();
    for row in rows_from_statement(&mut statement, [])? {
        let Some(kind) = MemoryKind::parse(&row.kind) else {
            continue;
        };
        groups
            .entry(memory_identity(kind, &row.title, &row.tags, &row.content))
            .or_default()
            .push(row.id);
    }
    Ok(groups)
}

fn semantic_merge(
    tx: &rusqlite::Transaction<'_>,
    report: &mut MemoryPruneReport,
) -> StoreResult<()> {
    for rows in title_groups(tx)?.values() {
        let Some((keeper, rest)) = rows.split_first() else {
            continue;
        };
        let mut merged = keeper.clone();
        let mut source_rows = Vec::new();
        for row in rest {
            if !high_overlap(&merged.content, &row.content) {
                continue;
            }
            merged.content = merged_content(&merged, row);
            merged.tags = merge_tags(&merged.tags, &row.tags);
            merged.tokens = merged.tokens.saturating_add(row.tokens);
            source_rows.push(row.id);
            delete_fts(tx, row)?;
            tx.execute("DELETE FROM memory WHERE id = ?1", params![row.id])?;
            report.deleted = report.deleted.saturating_add(1);
        }
        if !source_rows.is_empty() {
            update_merged(tx, &merged)?;
            report.merged = report.merged.saturating_add(1);
            report.source_rows.extend(source_rows);
        }
    }
    Ok(())
}

fn title_groups(tx: &rusqlite::Transaction<'_>) -> StoreResult<BTreeMap<String, Vec<MemoryRow>>> {
    let mut statement = tx.prepare(
        "SELECT id, kind, title, tags, content, tokens, updated_at
         FROM memory
         ORDER BY id",
    )?;
    let mut groups: BTreeMap<String, Vec<MemoryRow>> = BTreeMap::new();
    for row in rows_from_statement(&mut statement, [])? {
        let Some(kind) = MemoryKind::parse(&row.kind) else {
            continue;
        };
        let identity = memory_identity(kind, &row.title, &row.tags, &row.content);
        groups
            .entry(format!("{}:{}", row.kind, identity.title_slug))
            .or_default()
            .push(row);
    }
    Ok(groups)
}

fn update_merged(tx: &rusqlite::Transaction<'_>, row: &MemoryRow) -> StoreResult<()> {
    let before = get_required(tx, row.id)?;
    delete_fts(tx, &before)?;
    tx.execute(
        "UPDATE memory
         SET tags = ?1, content = ?2, tokens = ?3
         WHERE id = ?4",
        params![row.tags, row.content, row.tokens, row.id],
    )?;
    insert_fts(tx, row.id, &row.title, &row.tags, &row.content)
}

fn merged_content(keeper: &MemoryRow, source: &MemoryRow) -> String {
    if keeper.content.contains(&source.content) {
        return keeper.content.clone();
    }
    format!(
        "{}\n\n## Merged Memory Sources\n\n- source_row={}: {}",
        keeper.content, source.id, source.content
    )
}

fn merge_tags(left: &str, right: &str) -> String {
    let mut tags = BTreeSet::new();
    for tag in left.split(',').chain(right.split(',')) {
        let trimmed = tag.trim();
        if !trimmed.is_empty() {
            tags.insert(trimmed.to_string());
        }
    }
    tags.into_iter().collect::<Vec<_>>().join(",")
}

fn high_overlap(existing_content: &str, candidate_content: &str) -> bool {
    let existing = words(existing_content);
    let candidate = words(candidate_content);
    if existing.len() < 6 || candidate.len() < 6 {
        return false;
    }
    let shared = candidate
        .iter()
        .filter(|word| existing.iter().any(|existing| existing == *word))
        .count();
    shared.saturating_mul(100) / candidate.len().min(existing.len()) >= 85
}

fn words(value: &str) -> Vec<String> {
    value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}
