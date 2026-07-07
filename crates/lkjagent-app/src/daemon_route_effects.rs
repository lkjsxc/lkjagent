use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::{
    CheckSpec, Event, EventKind, Step, StepKind, StepState, TaskSnapshot, TaskState, TemplateId,
};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_rows::QueueRow;
use rusqlite::Connection;

use crate::snapshot_state::persist_snapshot_cell;

pub fn routed_inspection(
    conn: &mut Connection,
    row: &QueueRow,
    task_id: u64,
    now: &str,
) -> Result<Option<TaskSnapshot>, String> {
    let summary = inspection_summary(conn)?;
    routed_terminal(conn, row, task_id, now, TerminalRoute::closed(summary))
}

pub fn artifact_request_snapshot(task_id: u64, objective: &str) -> TaskSnapshot {
    let mut snapshot = instantiate(task_id, objective);
    let path = format!("artifacts/requests/matter-{task_id}.md");
    snapshot.task.template = TemplateId::LegacyArtifact;
    snapshot.task.checks = vec![CheckSpec::FileExists { path: path.clone() }];
    snapshot.steps = vec![
        step(
            task_id,
            1,
            StepKind::Write,
            "write artifact",
            objective,
            Some(path),
        ),
        step(
            task_id,
            2,
            StepKind::Verify,
            "verify artifact",
            "run checks",
            None,
        ),
        step(
            task_id,
            3,
            StepKind::Respond,
            "respond",
            "report artifact path",
            None,
        ),
    ];
    snapshot.steps[1].checks = snapshot.task.checks.clone();
    snapshot
}

pub fn routed_system_operation(
    conn: &mut Connection,
    row: &QueueRow,
    task_id: u64,
    now: &str,
) -> Result<Option<TaskSnapshot>, String> {
    let summary = "system_operation: blocked unsupported_executor no_command_run";
    routed_terminal(conn, row, task_id, now, TerminalRoute::blocked(summary))
}

fn step(
    task_id: u64,
    ordinal: u32,
    kind: StepKind,
    title: &str,
    instruction: &str,
    output_path: Option<String>,
) -> Step {
    Step {
        id: ordinal as u64,
        task_id,
        ordinal,
        kind,
        title: title.to_string(),
        instruction: instruction.to_string(),
        inputs: String::new(),
        output_path,
        checks: Vec::new(),
        state: StepState::Pending,
        attempts_used: 0,
        actions_used: 0,
        action_budget: 0,
        split_used: false,
    }
}

fn routed_terminal(
    conn: &mut Connection,
    row: &QueueRow,
    task_id: u64,
    now: &str,
    route: TerminalRoute,
) -> Result<Option<TaskSnapshot>, String> {
    let mut snapshot = instantiate(task_id, &row.content);
    assign_step_ids(&mut snapshot);
    snapshot.task.state = route.task_state;
    snapshot.task.summary = route.summary;
    for step in &mut snapshot.steps {
        step.state = route.step_state;
    }
    insert_task(conn, &snapshot.task, Some(row.id), now).map_err(|error| error.to_string())?;
    let tx = conn.transaction().map_err(|error| error.to_string())?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, now).map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())?;
    let event = Event {
        kind: route.event_kind,
        content: snapshot.task.summary.clone(),
    };
    snapshot.events.push(event.clone());
    commit_turn(conn, &snapshot, &[Command::RecordEvent(event)], now)
        .map_err(|error| error.to_string())?;
    persist_snapshot_cell(conn, &snapshot, now)?;
    Ok(Some(snapshot))
}

struct TerminalRoute {
    task_state: TaskState,
    step_state: StepState,
    event_kind: EventKind,
    summary: String,
}

impl TerminalRoute {
    fn closed(summary: String) -> Self {
        Self {
            task_state: TaskState::Closed,
            step_state: StepState::Done,
            event_kind: EventKind::Notice,
            summary,
        }
    }

    fn blocked(summary: &str) -> Self {
        Self {
            task_state: TaskState::Blocked,
            step_state: StepState::Blocked,
            event_kind: EventKind::TaskBlocked,
            summary: summary.to_string(),
        }
    }
}

fn inspection_summary(conn: &Connection) -> Result<String, String> {
    let pending = count(conn, "SELECT COUNT(*) FROM queue WHERE state = 'pending'")?;
    let active = count(
        conn,
        "SELECT COUNT(*) FROM tasks WHERE state IN ('open', 'waiting')",
    )?;
    let records = count(conn, "SELECT COUNT(*) FROM workspace_records")?;
    Ok(format!(
        "inspection: pending_queue={pending} active_matters={active} records={records}"
    ))
}

fn count(conn: &Connection, sql: &str) -> Result<i64, String> {
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn assign_step_ids(snapshot: &mut TaskSnapshot) {
    let base = snapshot.task.id.saturating_mul(1_000);
    for step in &mut snapshot.steps {
        step.id = base.saturating_add(step.ordinal as u64);
    }
}
