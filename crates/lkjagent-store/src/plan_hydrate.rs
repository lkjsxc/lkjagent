use lkjagent_core::classify::instantiate;
use lkjagent_core::model::*;
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
use crate::plan_names::{attempt_outcome, event_kind, step_kind, step_state, task_state, template};

pub fn active_snapshot(conn: &Connection) -> StoreResult<Option<TaskSnapshot>> {
    first_snapshot_with_state(conn, "open")?.map_or_else(
        || first_snapshot_with_state(conn, "waiting"),
        |snapshot| Ok(Some(snapshot)),
    )
}

pub fn first_snapshot_with_state(
    conn: &Connection,
    state: &str,
) -> StoreResult<Option<TaskSnapshot>> {
    let mut statement = conn.prepare(
        "SELECT id FROM tasks WHERE state = ?1
         ORDER BY COALESCE(queue_id, id), id LIMIT 1",
    )?;
    let mut rows = statement.query(params![state])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    snapshot_by_id(conn, row.get(0)?)
}

pub fn snapshot_by_id(conn: &Connection, id: i64) -> StoreResult<Option<TaskSnapshot>> {
    let task = match task(conn, id)? {
        Some(task) => task,
        None => return Ok(None),
    };
    Ok(Some(TaskSnapshot {
        steps: steps(conn, id)?,
        attempts: attempts(conn, id)?,
        check_results: check_results(conn, id)?,
        events: events(conn, id)?,
        task,
    }))
}

pub fn pending_count(conn: &Connection) -> StoreResult<usize> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM queue WHERE state = 'pending'",
        [],
        |row| row.get(0),
    )?;
    Ok(count as usize)
}

fn task(conn: &Connection, id: i64) -> StoreResult<Option<Task>> {
    let mut statement = conn.prepare(
        "SELECT objective, template, state, brief, budget_used, budget, summary
         FROM tasks WHERE id = ?1",
    )?;
    let mut rows = statement.query(params![id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    let objective: String = row.get(0)?;
    let mut task = instantiate(id as u64, &objective).task;
    task.template = template(&row.get::<_, String>(1)?)?;
    task.state = task_state(&row.get::<_, String>(2)?)?;
    task.brief = row.get(3)?;
    task.budget_used = row.get::<_, i64>(4)? as u32;
    task.budget = row.get::<_, i64>(5)? as u32;
    task.summary = row.get(6)?;
    Ok(Some(task))
}

fn steps(conn: &Connection, task_id: i64) -> StoreResult<Vec<Step>> {
    let mut statement = conn.prepare(
        "SELECT id, ordinal, kind, title, instruction, inputs_json, output_path,
         checks_json, state, attempts_used, actions_used, action_budget, split_used
         FROM steps WHERE task_id = ?1 ORDER BY ordinal, id",
    )?;
    let rows = statement.query_map(params![task_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, Option<String>>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, i64>(10)?,
            row.get::<_, i64>(11)?,
            row.get::<_, i64>(12)?,
        ))
    })?;
    let mut output = Vec::new();
    for row in rows {
        let row = row?;
        output.push(Step {
            id: row.0 as u64,
            task_id: task_id as u64,
            ordinal: row.1 as u32,
            kind: step_kind(&row.2)?,
            title: row.3,
            instruction: row.4,
            inputs: row.5,
            output_path: row.6,
            checks: checks_json(&row.7)?,
            state: step_state(&row.8)?,
            attempts_used: row.9 as u32,
            actions_used: row.10 as u32,
            action_budget: row.11 as u32,
            split_used: row.12 != 0,
        });
    }
    Ok(output)
}

fn attempts(conn: &Connection, task_id: i64) -> StoreResult<Vec<Attempt>> {
    let mut statement = conn.prepare(
        "SELECT step_id, ordinal, prompt_fingerprint, outcome, diagnosis, tokens_in, tokens_out
         FROM attempts WHERE step_id IN (SELECT id FROM steps WHERE task_id = ?1) ORDER BY id",
    )?;
    let rows = statement.query_map(params![task_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, i64>(6)?,
        ))
    })?;
    let mut output = Vec::new();
    for row in rows {
        let row = row?;
        output.push(Attempt {
            step_id: row.0 as u64,
            ordinal: row.1 as u32,
            prompt_fingerprint: row.2,
            outcome: attempt_outcome(&row.3)?,
            diagnosis: row.4,
            tokens_in: row.5 as u32,
            tokens_out: row.6 as u32,
            cached_tokens: 0,
            cache_status: "unknown".to_string(),
        });
    }
    Ok(output)
}

fn check_results(conn: &Connection, task_id: i64) -> StoreResult<Vec<CheckResult>> {
    let mut statement = conn.prepare(
        "SELECT name, params_json, decision_id, evidence_fingerprint, passed, measured
         FROM check_results WHERE step_id IN (SELECT id FROM steps WHERE task_id = ?1)
         ORDER BY id",
    )?;
    let mapped = statement.query_map(params![task_id], |row| {
        Ok(CheckResult {
            name: row.get(0)?,
            params: serde_json::from_str(&row.get::<_, String>(1)?).unwrap_or(None),
            decision_id: row.get(2)?,
            evidence_fingerprint: row.get(3)?,
            passed: row.get::<_, i64>(4)? != 0,
            measured: row.get(5)?,
        })
    })?;
    rows(mapped)
}

fn events(conn: &Connection, task_id: i64) -> StoreResult<Vec<Event>> {
    let mut statement =
        conn.prepare("SELECT kind, content FROM events WHERE task_id = ?1 ORDER BY id")?;
    let rows = statement.query_map(params![task_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut output = Vec::new();
    for row in rows {
        let (kind, content) = row?;
        output.push(Event {
            kind: event_kind(&kind)?,
            content,
        });
    }
    Ok(output)
}

fn rows<T>(rows: impl Iterator<Item = rusqlite::Result<T>>) -> StoreResult<Vec<T>> {
    let mut output = Vec::new();
    for row in rows {
        output.push(row?);
    }
    Ok(output)
}

fn checks_json(text: &str) -> StoreResult<Vec<CheckSpec>> {
    serde_json::from_str(text).map_err(|error| StoreError::Sql(error.to_string()))
}
