use rusqlite::{params, Transaction};

use crate::error::StoreResult;

use super::row::get_required;
use super::{delete_fts, insert_fts, MemoryUpdate};

pub(super) fn update_similar(
    tx: &Transaction<'_>,
    id: i64,
    row: MemoryUpdate<'_>,
    now: &str,
) -> StoreResult<()> {
    let before = get_required(tx, id)?;
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
    delete_fts(tx, &before)?;
    insert_fts(tx, id, row.title, row.tags, row.content)
}
