use std::path::Path;

use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{apply_turn, next_work, TurnOutcome, Work};
use lkjagent_core::model::{Event, EventKind, StepState, TaskSnapshot, TaskState};
use lkjagent_core::parse::parse_expected;
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::{deliver_next, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::plan_turn::commit_commands;
use rusqlite::Connection;

use crate::endpoint::LlmEndpoint;
use crate::state::{load_snapshot, save_snapshot};
use crate::turn_effects::{dispatch_effects, gather_checks};

pub trait Endpoint {
    fn complete(&mut self, prompt: &Prompt, attempt: u32) -> Result<String, String>;
}

pub fn run_daemon(data_dir: &Path) -> Result<(), String> {
    let mut endpoint = LlmEndpoint::new(data_dir);
    loop {
        let snapshot = run_until_idle(data_dir, &mut endpoint, 1)?;
        if !matches!(snapshot.task.state, TaskState::Open) {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

pub fn run_until_idle<E: Endpoint>(
    data_dir: &Path,
    endpoint: &mut E,
    max_turns: usize,
) -> Result<TaskSnapshot, String> {
    let db = data_dir.join("lkjagent.sqlite3");
    let workspace = data_dir.join("workspace");
    std::fs::create_dir_all(&workspace).map_err(|error| error.to_string())?;
    let mut conn = Connection::open(db).map_err(|error| error.to_string())?;
    setup(&conn).map_err(|error| error.to_string())?;
    let mut snapshot = match load_snapshot(&conn).map_err(|error| error.to_string())? {
        Some(snapshot) if runnable(&snapshot) => resume_waiting(&mut conn, snapshot)?,
        _ => match intake(&mut conn)? {
            Some(snapshot) => snapshot,
            None => return Ok(idle_snapshot()),
        },
    };
    for _ in 0..max_turns {
        if !matches!(snapshot.task.state, TaskState::Open) {
            break;
        }
        snapshot = run_turn(&mut conn, &workspace, snapshot, endpoint)?;
    }
    save_snapshot(&conn, &snapshot).map_err(|error| error.to_string())?;
    Ok(snapshot)
}

fn intake(conn: &mut Connection) -> Result<Option<TaskSnapshot>, String> {
    let task_id = next_task_id(conn)?;
    let queue = deliver_next(conn, task_id as i64, "now").map_err(|error| error.to_string())?;
    let Some(row) = queue else { return Ok(None) };
    let mut snapshot = instantiate(task_id, &row.content);
    assign_step_ids(&mut snapshot);
    insert_task(conn, &snapshot.task, Some(row.id), "now").map_err(|error| error.to_string())?;
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now").map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(Some(snapshot))
}

fn runnable(snapshot: &TaskSnapshot) -> bool {
    matches!(snapshot.task.state, TaskState::Open | TaskState::Waiting)
}

fn idle_snapshot() -> TaskSnapshot {
    let mut snapshot = instantiate(0, "idle");
    snapshot.task.state = TaskState::Closed;
    snapshot
}

fn next_task_id(conn: &Connection) -> Result<u64, String> {
    let id: i64 = conn
        .query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM tasks", [], |row| {
            row.get(0)
        })
        .map_err(|error| error.to_string())?;
    Ok(id as u64)
}

fn assign_step_ids(snapshot: &mut TaskSnapshot) {
    let base = snapshot.task.id.saturating_mul(1_000);
    for step in &mut snapshot.steps {
        step.id = base.saturating_add(step.ordinal as u64);
    }
}

fn resume_waiting(
    conn: &mut Connection,
    mut snapshot: TaskSnapshot,
) -> Result<TaskSnapshot, String> {
    if snapshot.task.state != TaskState::Waiting {
        return Ok(snapshot);
    }
    let Some(answer) = deliver_next(conn, 1, "now").map_err(|error| error.to_string())? else {
        return Ok(snapshot);
    };
    snapshot.task.state = TaskState::Open;
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .find(|step| step.state == StepState::Active)
    {
        step.inputs
            .push_str(&format!("\nowner_answer={}", answer.content));
    }
    snapshot.events.push(Event {
        kind: EventKind::Answer,
        content: answer.content,
    });
    Ok(snapshot)
}

fn run_turn<E: Endpoint>(
    conn: &mut Connection,
    workspace: &Path,
    snapshot: TaskSnapshot,
    endpoint: &mut E,
) -> Result<TaskSnapshot, String> {
    let work = next_work(&snapshot);
    let outcome = match &work {
        Work::CallModel { step_id, prompt } => {
            let step = snapshot
                .steps
                .iter()
                .find(|step| step.id == *step_id)
                .ok_or_else(|| "active step missing".to_string())?;
            match endpoint.complete(prompt, step.attempts_used) {
                Ok(raw) => match parse_expected(step.kind, &raw) {
                    Ok(parsed) => TurnOutcome::Model(parsed),
                    Err(fault) => TurnOutcome::ParseFault(fault),
                },
                Err(error) => TurnOutcome::EndpointError(error),
            }
        }
        Work::RunChecks { step_id } => gather_checks(workspace, &snapshot, *step_id)?,
        Work::CloseTask | Work::BlockTask(_) | Work::Wait => TurnOutcome::Noop,
    };
    let (mut next, commands) = apply_turn(&snapshot, &work, outcome);
    dispatch_effects(workspace, &mut next, &commands)?;
    commit_commands(conn, next.task.id as i64, &commands, "now")
        .map_err(|error| error.to_string())?;
    save_snapshot(conn, &next).map_err(|error| error.to_string())?;
    Ok(next)
}

#[derive(Debug, Clone)]
pub struct ScriptedEndpoint {
    pub outputs: Vec<String>,
    pub index: usize,
}

impl Endpoint for ScriptedEndpoint {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<String, String> {
        let Some(output) = self.outputs.get(self.index).cloned() else {
            return Err("scripted endpoint exhausted".to_string());
        };
        self.index = self.index.saturating_add(1);
        Ok(output)
    }
}
