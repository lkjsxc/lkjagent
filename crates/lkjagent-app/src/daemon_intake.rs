use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::{Event, EventKind, StepKind, StepState, TaskSnapshot, TaskState};
use lkjagent_store::plan_access::{
    deliver_answer, deliver_forced_new, deliver_next, insert_step_tx, insert_task,
};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_hydrate::first_snapshot_with_state;
use rusqlite::Connection;

use crate::clock::Clock;

pub fn load_runtime_snapshot<C: Clock>(
    conn: &mut Connection,
    clock: &mut C,
) -> Result<Option<TaskSnapshot>, String> {
    if let Some(snapshot) = first_snapshot_with_state(conn, "open").map_err(|e| e.to_string())? {
        return Ok(Some(snapshot));
    }
    if let Some(waiting) = first_snapshot_with_state(conn, "waiting").map_err(|e| e.to_string())? {
        let resumed = resume_waiting(conn, waiting, clock)?;
        if resumed.task.state == TaskState::Open {
            return Ok(Some(resumed));
        }
        if let Some(snapshot) = intake(conn, true, clock)? {
            return Ok(Some(snapshot));
        }
        return Ok(Some(resumed));
    }
    intake(conn, false, clock)
}

pub fn idle_snapshot() -> TaskSnapshot {
    let mut snapshot = instantiate(0, "idle");
    snapshot.task.state = TaskState::Closed;
    snapshot
}

fn intake<C: Clock>(
    conn: &mut Connection,
    forced_only: bool,
    clock: &mut C,
) -> Result<Option<TaskSnapshot>, String> {
    let task_id = next_task_id(conn)?;
    let now = clock.now();
    let queue = if forced_only {
        deliver_forced_new(conn, task_id as i64, &now)
    } else {
        deliver_next(conn, task_id as i64, &now)
    }
    .map_err(|error| error.to_string())?;
    let Some(row) = queue else { return Ok(None) };
    let mut snapshot = instantiate(task_id, &row.content);
    assign_step_ids(&mut snapshot);
    insert_task(conn, &snapshot.task, Some(row.id), &now).map_err(|error| error.to_string())?;
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, &now).map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(Some(snapshot))
}

fn resume_waiting<C: Clock>(
    conn: &mut Connection,
    mut snapshot: TaskSnapshot,
    clock: &mut C,
) -> Result<TaskSnapshot, String> {
    if snapshot.task.state != TaskState::Waiting {
        return Ok(snapshot);
    }
    let now = clock.now();
    let Some(answer) =
        deliver_answer(conn, snapshot.task.id as i64, &now).map_err(|error| error.to_string())?
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
    commit_turn(conn, &snapshot, &[Command::RecordEvent(event)], &now)
        .map_err(|error| error.to_string())?;
    Ok(snapshot)
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
