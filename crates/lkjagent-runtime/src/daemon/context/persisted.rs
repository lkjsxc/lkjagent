use lkjagent_store::state as store_state;
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::dispatch::DispatchState;
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::prompt::token_estimate;

pub fn restore_completion_guard(conn: &Connection, state: &mut DispatchState) -> RuntimeResult<()> {
    let value = store_state::get(conn, "completion guard")?.unwrap_or_else(|| "none".to_string());
    state
        .control
        .set_guard(CompletionGuard::from_state_value(&value));
    Ok(())
}

pub(super) fn next_owner_tokens(conn: &Connection) -> RuntimeResult<usize> {
    let rows = lkjagent_store::queue::list(conn)?;
    let tokens = rows
        .iter()
        .find(|row| row.status == "pending")
        .map_or(0, |row| {
            token_estimate(&lkjagent_protocol::render_owner(&row.content))
        });
    Ok(tokens)
}

pub(super) fn owner_preview(content: &str) -> String {
    let first = content
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or("active");
    first.chars().take(80).collect()
}
