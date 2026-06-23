use std::collections::BTreeSet;

use rusqlite::{params, Transaction};

use crate::error::StoreResult;

use super::prune::MemoryPruneReport;
use super::row::{rows_from_statement, MemoryRow};
use super::{delete_fts, insert_fts};

pub(super) fn rewrite_low_signal(
    tx: &Transaction<'_>,
    report: &mut MemoryPruneReport,
) -> StoreResult<()> {
    let rows = low_signal_rows(tx)?;
    if rows.len() < 3 {
        return Ok(());
    }
    let Some((keeper, sources)) = rows.split_first() else {
        return Ok(());
    };
    let content = rewrite_content(keeper, sources);
    let tags = rewrite_tags(&rows);
    let tokens = rows.iter().map(|row| row.tokens).sum::<i64>().max(1);
    update_keeper(tx, keeper, &tags, &content, tokens)?;
    for source in sources {
        delete_fts(tx, source)?;
        tx.execute("DELETE FROM memory WHERE id = ?1", params![source.id])?;
        report.deleted = report.deleted.saturating_add(1);
        report.source_rows.push(source.id);
    }
    report.rewritten = report.rewritten.saturating_add(1);
    Ok(())
}

fn low_signal_rows(tx: &Transaction<'_>) -> StoreResult<Vec<MemoryRow>> {
    let mut statement = tx.prepare(
        "SELECT id, kind, title, tags, content, tokens, updated_at
         FROM memory WHERE kind = 'lesson' ORDER BY id",
    )?;
    Ok(rows_from_statement(&mut statement, [])?
        .into_iter()
        .filter(low_signal)
        .collect())
}

fn low_signal(row: &MemoryRow) -> bool {
    tagged(&row.tags, "maintenance") && low_signal_text(&format!("{} {}", row.title, row.content))
}

fn low_signal_text(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("reviewed")
        || lower.contains("checked")
        || lower.contains("scanned")
        || lower.contains("minor")
}

fn update_keeper(
    tx: &Transaction<'_>,
    keeper: &MemoryRow,
    tags: &str,
    content: &str,
    tokens: i64,
) -> StoreResult<()> {
    delete_fts(tx, keeper)?;
    tx.execute(
        "UPDATE memory SET title = ?1, tags = ?2, content = ?3, tokens = ?4 WHERE id = ?5",
        params![
            "Maintenance Rewrite Summary",
            tags,
            content,
            tokens,
            keeper.id
        ],
    )?;
    insert_fts(tx, keeper.id, "Maintenance Rewrite Summary", tags, content)
}

fn rewrite_content(keeper: &MemoryRow, sources: &[MemoryRow]) -> String {
    let mut lines = vec![
        "Maintenance rewrite summary preserving low-signal source rows.".to_string(),
        format!("- source_row={}: {}", keeper.id, one_line(&keeper.content)),
    ];
    for source in sources {
        lines.push(format!(
            "- source_row={}: {}",
            source.id,
            one_line(&source.content)
        ));
    }
    lines.join("\n")
}

fn rewrite_tags(rows: &[MemoryRow]) -> String {
    let mut tags = BTreeSet::from(["maintenance".to_string(), "rewrite".to_string()]);
    for row in rows {
        for tag in row.tags.split(',') {
            let trimmed = tag.trim();
            if !trimmed.is_empty() {
                tags.insert(trimmed.to_string());
            }
        }
    }
    tags.into_iter().collect::<Vec<_>>().join(",")
}

fn tagged(tags: &str, needle: &str) -> bool {
    tags.split(',')
        .any(|tag| tag.trim().eq_ignore_ascii_case(needle))
}

fn one_line(text: &str) -> String {
    text.lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.chars().take(120).collect())
        .unwrap_or_else(|| "empty".to_string())
}
