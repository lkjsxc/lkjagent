use rusqlite::Connection;

use crate::artifact_next_response::{audit_response, batch_response, cursor_key};
use crate::error::ToolResult;

const WEAK_PATH_BATCH_SIZE: usize = 1;

pub(crate) fn record_story_batch(
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
    selected: &[String],
    valid_example: &str,
) -> ToolResult<()> {
    record_batch(BatchRecord {
        conn,
        now,
        root,
        kind,
        weak_count: selected.len(),
        selected,
        valid_example,
        current_index: selected.len(),
    })
}

pub(crate) fn cursor_batch(
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
    weak: Vec<String>,
) -> ToolResult<String> {
    let start = next_start(conn, root, &weak)?;
    if start >= weak.len() {
        return audit_response(root, kind, &format!("missing={}", weak.len()));
    }
    let weak_count = weak.len();
    let selected = weak
        .into_iter()
        .skip(start)
        .take(WEAK_PATH_BATCH_SIZE)
        .collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write_contract(root, kind, &selected);
    record_batch(BatchRecord {
        conn,
        now,
        root,
        kind,
        weak_count,
        selected: &selected,
        valid_example: &valid_example,
        current_index: start.saturating_add(selected.len()),
    })?;
    if let Some(last) = selected.last() {
        lkjagent_store::state::set(conn, &cursor_key(root), last)?;
    }
    Ok(batch_response(root, kind, &selected, &valid_example))
}

struct BatchRecord<'a> {
    conn: &'a Connection,
    now: &'a str,
    root: &'a str,
    kind: &'a str,
    weak_count: usize,
    selected: &'a [String],
    valid_example: &'a str,
    current_index: usize,
}

fn record_batch(record: BatchRecord<'_>) -> ToolResult<()> {
    crate::artifact_cursor_support::record_next_batch(
        crate::artifact_cursor_support::NextBatchRecord {
            conn: record.conn,
            root: record.root,
            kind: record.kind,
            weak_count: record.weak_count,
            selected: record.selected,
            valid_example: record.valid_example,
            current_index: record.current_index,
            now: record.now,
        },
    )
}

fn next_start(conn: &Connection, root: &str, weak: &[String]) -> ToolResult<usize> {
    let Some(cursor) = lkjagent_store::state::get(conn, &cursor_key(root))? else {
        return Ok(0);
    };
    Ok(weak
        .iter()
        .position(|path| path > &cursor)
        .unwrap_or(weak.len()))
}
