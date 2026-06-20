use std::collections::BTreeMap;

use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::identity::{memory_identity, MemoryIdentity};
use super::row::{get_required, rows_from_statement};
use super::{delete_fts, MemoryKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPruneReport {
    pub kept: usize,
    pub deleted: usize,
}

pub fn prune_exact_duplicates(conn: &mut Connection) -> StoreResult<MemoryPruneReport> {
    let tx = conn.transaction()?;
    let groups = duplicate_groups(&tx)?;
    let mut kept = 0usize;
    let mut deleted = 0usize;
    for ids in groups.values().filter(|ids| ids.len() > 1) {
        kept = kept.saturating_add(1);
        for id in ids.iter().skip(1) {
            let row = get_required(&tx, *id)?;
            delete_fts(&tx, &row)?;
            tx.execute("DELETE FROM memory WHERE id = ?1", params![id])?;
            deleted = deleted.saturating_add(1);
        }
    }
    tx.commit()?;
    Ok(MemoryPruneReport { kept, deleted })
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
