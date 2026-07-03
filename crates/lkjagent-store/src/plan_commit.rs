use lkjagent_core::engine::Command;
use lkjagent_core::model::{EventKind, Step, Task, TaskSnapshot};
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
use crate::memory::insert_memory_tx;
use crate::plan_access::insert_step_tx;

pub fn commit_turn(
    conn: &mut Connection,
    snapshot: &TaskSnapshot,
    commands: &[Command],
    now: &str,
) -> StoreResult<()> {
    let tx = conn.transaction()?;
    for command in commands {
        persist_command(&tx, snapshot, command, now)?;
    }
    update_task(&tx, &snapshot.task, now)?;
    for step in &snapshot.steps {
        update_step(&tx, step, now)?;
    }
    tx.commit()?;
    Ok(())
}

fn persist_command(
    tx: &rusqlite::Transaction<'_>,
    snapshot: &TaskSnapshot,
    command: &Command,
    now: &str,
) -> StoreResult<()> {
    let task_id = snapshot.task.id as i64;
    match command {
        Command::RecordAttempt(attempt) => {
            tx.execute(
                "INSERT INTO attempts (step_id, ordinal, prompt_fingerprint, exchange_ref,
                 outcome, diagnosis, tokens_in, tokens_out, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    attempt.step_id as i64,
                    attempt.ordinal as i64,
                    attempt.prompt_fingerprint,
                    exchange_ref(
                        task_id,
                        step_ordinal(snapshot, attempt.step_id),
                        attempt.ordinal
                    ),
                    lower(&format!("{:?}", attempt.outcome)),
                    attempt.diagnosis,
                    attempt.tokens_in as i64,
                    attempt.tokens_out as i64,
                    now,
                ],
            )?;
            insert_usage(tx, task_id, attempt, now)?;
        }
        Command::RecordEvent(event) => insert_event(tx, task_id, event.kind, &event.content, now)?,
        Command::RecordMemory { topic, content } => {
            insert_memory_tx(tx, topic, content, task_id, now)?;
        }
        Command::RecordChecks { step_id, results } => {
            for (index, result) in results.iter().enumerate() {
                tx.execute(
                    "INSERT INTO check_results (step_id, name, params_json, passed, measured,
                     created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        *step_id as i64,
                        result.name,
                        check_params(snapshot, *step_id, index)?,
                        i64::from(result.passed),
                        result.measured,
                        now
                    ],
                )?;
            }
        }
        Command::AddSteps(steps) => {
            for step in steps {
                insert_step_tx(tx, step, now)?;
            }
        }
        Command::WriteFile { .. } | Command::RunExplore(_) => {}
    }
    Ok(())
}

fn update_task(tx: &rusqlite::Transaction<'_>, task: &Task, now: &str) -> StoreResult<()> {
    tx.execute(
        "UPDATE tasks SET state = ?1, brief = ?2, budget_used = ?3, budget = ?4,
         summary = ?5, updated_at = ?6 WHERE id = ?7",
        params![
            lower(&format!("{:?}", task.state)),
            task.brief,
            task.budget_used as i64,
            task.budget as i64,
            task.summary,
            now,
            task.id as i64,
        ],
    )?;
    Ok(())
}

fn update_step(tx: &rusqlite::Transaction<'_>, step: &Step, now: &str) -> StoreResult<()> {
    let checks =
        serde_json::to_string(&step.checks).map_err(|error| StoreError::Sql(error.to_string()))?;
    tx.execute(
        "UPDATE steps SET title = ?1, instruction = ?2, inputs_json = ?3,
         output_path = ?4, checks_json = ?5, state = ?6, attempts_used = ?7,
         actions_used = ?8, action_budget = ?9, split_used = ?10, updated_at = ?11
         WHERE id = ?12",
        params![
            step.title,
            step.instruction,
            step.inputs,
            step.output_path,
            checks,
            lower(&format!("{:?}", step.state)),
            step.attempts_used as i64,
            step.actions_used as i64,
            step.action_budget as i64,
            i64::from(step.split_used),
            now,
            step.id as i64,
        ],
    )?;
    Ok(())
}

fn insert_usage(
    tx: &rusqlite::Transaction<'_>,
    task_id: i64,
    attempt: &lkjagent_core::model::Attempt,
    now: &str,
) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO token_usage (task_id, attempt_id, prompt_tokens, completion_tokens,
         cached_tokens, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            task_id,
            tx.last_insert_rowid(),
            token(attempt.tokens_in),
            token(attempt.tokens_out),
            token(attempt.cached_tokens),
            now
        ],
    )?;
    Ok(())
}

fn check_params(snapshot: &TaskSnapshot, step_id: u64, index: usize) -> StoreResult<String> {
    let params = snapshot
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .and_then(|step| step.checks.get(index));
    serde_json::to_string(&params).map_err(|error| StoreError::Sql(error.to_string()))
}

fn token(value: u32) -> Option<i64> {
    (value > 0).then_some(value as i64)
}

fn insert_event(
    tx: &rusqlite::Transaction<'_>,
    task_id: i64,
    kind: EventKind,
    content: &str,
    now: &str,
) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO events (task_id, kind, content, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![task_id, lower(&format!("{:?}", kind)), content, now],
    )?;
    Ok(())
}

fn exchange_ref(task_id: i64, step_ordinal: u32, ordinal: u32) -> String {
    format!("logs/task-{task_id}/step-{step_ordinal}/attempt-{ordinal}")
}

fn step_ordinal(snapshot: &TaskSnapshot, step_id: u64) -> u32 {
    snapshot
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .map_or(step_id as u32, |step| step.ordinal)
}

fn lower(value: &str) -> String {
    value.to_ascii_lowercase()
}
