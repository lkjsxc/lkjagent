use std::collections::BTreeSet;

use lkjagent_core::engine::Command;
use lkjagent_core::model::EventKind;
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::memory::insert_memory_tx;
use crate::plan_access::insert_step_tx;
use crate::plan_rows::{OrphanExchange, StoredEvent};

pub fn commit_commands(
    conn: &mut Connection,
    task_id: i64,
    commands: &[Command],
    now: &str,
) -> StoreResult<()> {
    let tx = conn.transaction()?;
    for command in commands {
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
                        exchange_ref(task_id, attempt.step_id, attempt.ordinal),
                        format!("{:?}", attempt.outcome).to_ascii_lowercase(),
                        attempt.diagnosis,
                        attempt.tokens_in as i64,
                        attempt.tokens_out as i64,
                        now,
                    ],
                )?;
            }
            Command::RecordEvent(event) => {
                update_task_for_event(&tx, task_id, event.kind, &event.content, now)?;
                insert_event(&tx, task_id, event.kind, &event.content, now)?
            }
            Command::RecordMemory { topic, content } => {
                insert_memory_tx(&tx, topic, content, task_id, now)?;
            }
            Command::RecordChecks { step_id, results } => {
                for result in results {
                    tx.execute(
                        "INSERT INTO check_results (step_id, name, params_json, passed, measured,
                         created_at) VALUES (?1, ?2, '{}', ?3, ?4, ?5)",
                        params![
                            *step_id as i64,
                            result.name,
                            i64::from(result.passed),
                            result.measured,
                            now
                        ],
                    )?;
                }
            }
            Command::AddSteps(steps) => {
                for step in steps {
                    insert_step_tx(&tx, step, now)?;
                }
            }
            Command::WriteFile { .. } | Command::RunExplore(_) => {}
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn events(conn: &Connection) -> StoreResult<Vec<StoredEvent>> {
    let mut statement =
        conn.prepare("SELECT id, task_id, kind, content FROM events ORDER BY id")?;
    let rows = statement.query_map([], |row| {
        Ok(StoredEvent {
            id: row.get(0)?,
            task_id: row.get(1)?,
            kind: row.get(2)?,
            content: row.get(3)?,
        })
    })?;
    let mut output = Vec::new();
    for row in rows {
        output.push(row?);
    }
    Ok(output)
}

pub fn orphan_exchanges(paths: &[String], committed_refs: &[String]) -> Vec<OrphanExchange> {
    let committed = committed_refs.iter().collect::<BTreeSet<_>>();
    paths
        .iter()
        .filter(|path| !committed.contains(path))
        .map(|path| OrphanExchange { path: path.clone() })
        .collect()
}

fn update_task_for_event(
    tx: &rusqlite::Transaction<'_>,
    task_id: i64,
    kind: EventKind,
    content: &str,
    now: &str,
) -> StoreResult<()> {
    match kind {
        EventKind::TaskClosed => {
            tx.execute(
                "UPDATE tasks SET state = 'closed', summary = ?1, updated_at = ?2 WHERE id = ?3",
                params![content, now, task_id],
            )?;
        }
        EventKind::TaskBlocked => {
            tx.execute(
                "UPDATE tasks SET state = 'blocked', summary = ?1, updated_at = ?2 WHERE id = ?3",
                params![content, now, task_id],
            )?;
        }
        _ => {}
    }
    Ok(())
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
        params![
            task_id,
            format!("{:?}", kind).to_ascii_lowercase(),
            content,
            now
        ],
    )?;
    Ok(())
}

pub fn set_config(conn: &Connection, key: &str, value: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

fn exchange_ref(task_id: i64, step_id: u64, ordinal: u32) -> String {
    format!("logs/task-{task_id}/step-{step_id}/attempt-{ordinal}")
}

pub fn config(conn: &Connection, key: &str) -> StoreResult<Option<String>> {
    let mut statement = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
    let mut rows = statement.query(params![key])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(row.get(0)?))
}
