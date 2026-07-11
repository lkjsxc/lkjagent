use std::collections::BTreeSet;

use lkjagent_core::model::{Step, Task, TaskState};
use rusqlite::{params, Connection, Transaction};

use crate::error::StoreResult;

pub use crate::queue_access::{
    deliver_answer, deliver_forced_new, deliver_matter_update, deliver_next, enqueue,
    enqueue_with_force, mark_recorded, next_pending,
};

pub fn attach_answer(
    conn: &Connection,
    task_id: i64,
    content: &str,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO queue (content, state, created_at, task_id) VALUES (?1, 'answered', ?2, ?3)",
        params![content, now, task_id],
    )?;
    conn.execute(
        "UPDATE tasks SET state = 'open', updated_at = ?1 WHERE id = ?2",
        params![now, task_id],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_task(
    conn: &Connection,
    task: &Task,
    queue_id: Option<i64>,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO tasks (id, queue_id, objective, template, state, brief, budget_used, budget,
         summary, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)",
        params![
            task.id as i64,
            queue_id,
            task.objective,
            format!("{:?}", task.template).to_ascii_lowercase(),
            state_name(task.state),
            task.brief,
            task.budget_used as i64,
            task.budget as i64,
            task.summary,
            now,
        ],
    )?;
    Ok(())
}

pub fn insert_step_tx(tx: &Transaction<'_>, step: &Step, now: &str) -> StoreResult<()> {
    let checks = serde_json::to_string(&step.checks)
        .map_err(|error| crate::error::StoreError::Sql(error.to_string()))?;
    tx.execute(
        "INSERT INTO steps (id, task_id, ordinal, kind, title, instruction, inputs_json,
         output_path, checks_json, state, attempts_used, actions_used, action_budget, split_used,
         created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?15)",
        params![
            step.id as i64,
            step.task_id as i64,
            step.ordinal as i64,
            format!("{:?}", step.kind).to_ascii_lowercase(),
            step.title,
            step.instruction,
            step.inputs,
            step.output_path,
            checks,
            format!("{:?}", step.state).to_ascii_lowercase(),
            step.attempts_used as i64,
            step.actions_used as i64,
            step.action_budget as i64,
            i64::from(step.split_used),
            now,
        ],
    )?;
    Ok(())
}

pub fn set_task_state(
    conn: &Connection,
    task_id: i64,
    state: TaskState,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE tasks SET state = ?1, updated_at = ?2 WHERE id = ?3",
        params![state_name(state), now, task_id],
    )?;
    Ok(())
}

pub fn application_tables(conn: &Connection) -> StoreResult<BTreeSet<String>> {
    let mut statement = conn.prepare(
        "SELECT name FROM sqlite_master
         WHERE type IN ('table', 'virtual table') AND name NOT LIKE 'sqlite_%'
         ORDER BY name",
    )?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    let mut output = BTreeSet::new();
    for row in rows {
        let name = row?;
        if !fts_shadow(&name) {
            output.insert(name);
        }
    }
    Ok(output)
}

fn fts_shadow(name: &str) -> bool {
    [
        "memory_fts_",
        "workspace_search_lexical_",
        "workspace_search_trigram_",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
}

fn state_name(state: TaskState) -> &'static str {
    match state {
        TaskState::Open => "open",
        TaskState::Waiting => "waiting",
        TaskState::Blocked => "blocked",
        TaskState::Closed => "closed",
    }
}
