use lkjagent_core::model::{TaskSnapshot, TaskState};
use rusqlite::Connection;

use crate::state::load_snapshot;

pub fn status(conn: &Connection) -> Result<String, String> {
    let snapshot = load_snapshot(conn).map_err(|error| error.to_string())?;
    Ok(match snapshot {
        Some(snapshot) => render_status(&snapshot),
        None => "daemon: idle\ntask: none\nstep: none\nlast: none\nquestion: none\nqueue: 0 pending\ntokens: task in=0 out=0 cached=0".to_string(),
    })
}

pub fn render_status(snapshot: &TaskSnapshot) -> String {
    let daemon = match snapshot.task.state {
        TaskState::Open => "working",
        TaskState::Waiting => "waiting",
        TaskState::Blocked | TaskState::Closed => "stopped",
    };
    let active = snapshot
        .steps
        .iter()
        .find(|step| !matches!(step.state, lkjagent_core::model::StepState::Done));
    let step_line = active.map_or_else(
        || "step: none".to_string(),
        |step| {
            format!(
                "step: {}/{} {:?} attempt {}/3",
                step.ordinal,
                snapshot.steps.len(),
                step.kind,
                step.attempts_used
            )
        },
    );
    format!(
        "daemon: {daemon}\ntask: {} {:?} {:?} budget {}/{}\n{}\nlast: {}\nquestion: none\nqueue: 0 pending\ntokens: task in=0 out=0 cached=0",
        snapshot.task.id,
        snapshot.task.state,
        snapshot.task.template,
        snapshot.task.budget_used,
        snapshot.task.budget,
        step_line,
        snapshot.task.summary
    )
}

pub fn task_show(snapshot: &TaskSnapshot) -> String {
    let mut lines = vec![format!(
        "task {} {:?}",
        snapshot.task.id, snapshot.task.state
    )];
    for step in &snapshot.steps {
        lines.push(format!(
            "{} {:?} {:?} {}",
            step.ordinal, step.kind, step.state, step.title
        ));
    }
    lines.join("\n")
}

pub fn watch(snapshot: &TaskSnapshot) -> String {
    format!(
        "transcript\n{}\n---\nplan\n{}",
        snapshot.task.summary,
        task_show(snapshot)
    )
}
