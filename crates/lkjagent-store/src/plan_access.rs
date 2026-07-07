use lkjagent_core::model::{Step, Task, TaskState};
use rusqlite::{params, Connection, Transaction};

use crate::error::StoreResult;
use crate::plan_rows::QueueRow;

pub fn enqueue(conn: &Connection, content: &str, now: &str) -> StoreResult<i64> {
    enqueue_with_force(conn, content, false, now)
}

pub fn enqueue_with_force(
    conn: &Connection,
    content: &str,
    force_new: bool,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO queue (content, state, force_new, created_at)
         VALUES (?1, 'pending', ?2, ?3)",
        params![content, i64::from(force_new), now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn deliver_next(conn: &Connection, task_id: i64, now: &str) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, None, now)
}

pub fn mark_recorded(conn: &Connection, queue_id: i64, now: &str) -> StoreResult<()> {
    conn.execute(
        "UPDATE queue SET state = 'recorded', delivered_at = ?1 WHERE id = ?2",
        params![now, queue_id],
    )?;
    Ok(())
}

pub fn deliver_answer(conn: &Connection, task_id: i64, now: &str) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, Some(false), now)
}

pub fn deliver_forced_new(
    conn: &Connection,
    task_id: i64,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, Some(true), now)
}

fn deliver_matching(
    conn: &Connection,
    task_id: i64,
    force_new: Option<bool>,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    let row = next_pending_matching(conn, force_new)?;
    let Some(row) = row else { return Ok(None) };
    conn.execute(
        "UPDATE queue SET state = 'delivered', delivered_at = ?1, task_id = ?2 WHERE id = ?3",
        params![now, task_id, row.id],
    )?;
    Ok(Some(QueueRow {
        state: "delivered".to_string(),
        task_id: Some(task_id),
        ..row
    }))
}

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

pub fn next_pending(conn: &Connection) -> StoreResult<Option<QueueRow>> {
    next_pending_matching(conn, None)
}

pub fn next_answer(conn: &Connection) -> StoreResult<Option<QueueRow>> {
    next_pending_matching(conn, Some(false))
}

pub fn next_forced_new(conn: &Connection) -> StoreResult<Option<QueueRow>> {
    next_pending_matching(conn, Some(true))
}

fn next_pending_matching(
    conn: &Connection,
    force_new: Option<bool>,
) -> StoreResult<Option<QueueRow>> {
    let clause = match force_new {
        Some(true) => " AND force_new = 1",
        Some(false) => " AND force_new = 0",
        None => "",
    };
    let sql = format!(
        "SELECT id, content, state, task_id, force_new FROM queue
         WHERE state = 'pending'{clause} ORDER BY id LIMIT 1",
    );
    let mut statement = conn.prepare(&sql)?;
    let mut rows = statement.query([])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(QueueRow {
        id: row.get(0)?,
        content: row.get(1)?,
        state: row.get(2)?,
        task_id: row.get(3)?,
        force_new: row.get::<_, i64>(4)? != 0,
    }))
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

fn state_name(state: TaskState) -> &'static str {
    match state {
        TaskState::Open => "open",
        TaskState::Waiting => "waiting",
        TaskState::Blocked => "blocked",
        TaskState::Closed => "closed",
    }
}
