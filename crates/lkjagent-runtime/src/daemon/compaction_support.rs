use lkjagent_protocol::Action;
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub(super) fn compaction_prompt(task_summary_required: bool) -> String {
    format!(
        "distill before compaction\nmax_turns=4\nuse memory.save only\n\
         task_summary_required={task_summary_required}\nfinal_save=task-summary when required"
    )
}

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

pub(super) fn memory_id(content: &str) -> Option<i64> {
    content
        .strip_prefix("memory_id=")
        .and_then(|value| value.trim().parse::<i64>().ok())
}

pub(super) fn param(action: &Action, name: &str) -> Option<String> {
    action
        .params
        .iter()
        .find(|param| param.name == name)
        .map(|param| param.value.clone())
}
