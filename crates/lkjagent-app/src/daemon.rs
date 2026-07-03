use std::path::Path;

use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{apply_turn, next_work, Command, TurnOutcome, Work};
use lkjagent_core::model::{Event, EventKind, StepKind, StepState, TaskSnapshot, TaskState};
use lkjagent_store::plan_access::{
    deliver_answer, deliver_forced_new, deliver_next, insert_step_tx, insert_task,
};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_hydrate::first_snapshot_with_state;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

use crate::effect_error::settle as settle_effect_error;
use crate::endpoint::LlmEndpoint;
use crate::model_call::{apply_record, call};
use crate::turn_effects::{dispatch_effects, gather_checks};

pub use crate::model_io::{CompletionRecord, Endpoint, ScriptedEndpoint};

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
    let logs = data_dir.join("logs");
    std::fs::create_dir_all(&workspace).map_err(|error| error.to_string())?;
    let mut conn = Connection::open(db).map_err(|error| error.to_string())?;
    setup(&conn).map_err(|error| error.to_string())?;
    let mut snapshot = match load_runtime_snapshot(&mut conn)? {
        Some(snapshot) if runnable(&snapshot) => snapshot,
        Some(snapshot) => return Ok(snapshot),
        None => return Ok(idle_snapshot()),
    };
    for _ in 0..max_turns {
        if !matches!(snapshot.task.state, TaskState::Open) {
            break;
        }
        snapshot = run_turn(&mut conn, &workspace, &logs, snapshot, endpoint)?;
    }
    Ok(snapshot)
}

fn load_runtime_snapshot(conn: &mut Connection) -> Result<Option<TaskSnapshot>, String> {
    if let Some(snapshot) = first_snapshot_with_state(conn, "open").map_err(|e| e.to_string())? {
        return Ok(Some(snapshot));
    }
    if let Some(waiting) = first_snapshot_with_state(conn, "waiting").map_err(|e| e.to_string())? {
        let resumed = resume_waiting(conn, waiting)?;
        if resumed.task.state == TaskState::Open {
            return Ok(Some(resumed));
        }
        if let Some(snapshot) = intake(conn, true)? {
            return Ok(Some(snapshot));
        }
        return Ok(Some(resumed));
    }
    intake(conn, false)
}

fn intake(conn: &mut Connection, forced_only: bool) -> Result<Option<TaskSnapshot>, String> {
    let task_id = next_task_id(conn)?;
    let queue = if forced_only {
        deliver_forced_new(conn, task_id as i64, "now")
    } else {
        deliver_next(conn, task_id as i64, "now")
    }
    .map_err(|error| error.to_string())?;
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
    let Some(answer) =
        deliver_answer(conn, snapshot.task.id as i64, "now").map_err(|error| error.to_string())?
    else {
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
        if step.kind == StepKind::Ask {
            step.state = StepState::Done;
        }
    }
    let event = Event {
        kind: EventKind::Answer,
        content: answer.content,
    };
    snapshot.events.push(event.clone());
    commit_turn(conn, &snapshot, &[Command::RecordEvent(event)], "now")
        .map_err(|error| error.to_string())?;
    Ok(snapshot)
}

fn run_turn<E: Endpoint>(
    conn: &mut Connection,
    workspace: &Path,
    logs: &Path,
    snapshot: TaskSnapshot,
    endpoint: &mut E,
) -> Result<TaskSnapshot, String> {
    let work = next_work(&snapshot);
    let outcome = match &work {
        Work::CallModel { step_id, prompt } => {
            let (outcome, record) = call(logs, &snapshot, *step_id, prompt, endpoint)?;
            let (mut next, mut commands) = apply_turn(&snapshot, &work, outcome);
            if let Some(record) = record {
                apply_record(&mut next, &mut commands, &record);
            }
            if let Err(error) = dispatch_effects(workspace, &mut next, &commands) {
                return settle_effect_error(conn, &snapshot, &work, error);
            }
            commit_turn(conn, &next, &commands, "now").map_err(|error| error.to_string())?;
            return Ok(next);
        }
        Work::RunChecks { step_id } => gather_checks(workspace, &snapshot, *step_id)?,
        Work::CloseTask | Work::BlockTask(_) | Work::Wait => TurnOutcome::Noop,
    };
    let (mut next, commands) = apply_turn(&snapshot, &work, outcome);
    if let Err(error) = dispatch_effects(workspace, &mut next, &commands) {
        return settle_effect_error(conn, &snapshot, &work, error);
    }
    commit_turn(conn, &next, &commands, "now").map_err(|error| error.to_string())?;
    Ok(next)
}
