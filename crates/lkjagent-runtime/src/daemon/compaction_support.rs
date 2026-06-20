use lkjagent_store::state as store_state;
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub(super) fn compaction_summary(
    conn: &Connection,
    reason: &str,
    before: usize,
) -> RuntimeResult<String> {
    let task = match store_state::get(conn, "open task")? {
        Some(value) => value,
        None => "active".to_string(),
    };
    Ok(format!(
        "compaction resume\nreason={reason}\nopen_task={task}\nbefore_tokens={before}"
    ))
}
