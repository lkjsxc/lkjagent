use lkjagent_core::model::{EventKind, StepState, TaskSnapshot, TaskState};
use rusqlite::Connection;

use crate::state::load_snapshot;

pub fn status(conn: &Connection) -> Result<String, String> {
    let snapshot = load_snapshot(conn).map_err(|error| error.to_string())?;
    let pending =
        lkjagent_store::plan_hydrate::pending_count(conn).map_err(|error| error.to_string())?;
    let tokens = token_line(conn)?;
    let ledger = state_ledger_lines(conn)?;
    Ok(match snapshot {
        Some(snapshot) => format!("{}\n{}", render_status_with(&snapshot, pending, &tokens), ledger),
        None => format!(
            "daemon: idle\ntask: none\nstep: none\nlast: none\nquestion: none\nqueue: {pending} pending\ntokens: {tokens}\n{ledger}"
        ),
    })
}

pub fn render_status(snapshot: &TaskSnapshot) -> String {
    render_status_with(snapshot, 0, "unknown")
}

fn render_status_with(snapshot: &TaskSnapshot, pending: usize, tokens: &str) -> String {
    let daemon = match snapshot.task.state {
        TaskState::Open => "working",
        TaskState::Waiting => "waiting",
        TaskState::Blocked | TaskState::Closed => "stopped",
    };
    let active = snapshot
        .steps
        .iter()
        .find(|step| !matches!(step.state, StepState::Done | StepState::Skipped));
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
        "daemon: {daemon}\ntask: {} {:?} {:?} budget {}/{}\n{}\nlast: {}\nquestion: {}\nqueue: {pending} pending\ntokens: {tokens}",
        snapshot.task.id,
        snapshot.task.state,
        snapshot.task.template,
        snapshot.task.budget_used,
        snapshot.task.budget,
        step_line,
        last_event(snapshot),
        question(snapshot)
    )
}

pub fn task_show(snapshot: &TaskSnapshot) -> String {
    let mut lines = vec![format!(
        "task {} {:?}",
        snapshot.task.id, snapshot.task.state
    )];
    for step in &snapshot.steps {
        lines.push(format!(
            "{} {:?} {:?} {} attempts={} actions={} checks={}",
            step.ordinal,
            step.kind,
            step.state,
            step.title,
            step.attempts_used,
            step.actions_used,
            step.checks.len()
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

fn last_event(snapshot: &TaskSnapshot) -> String {
    snapshot.events.last().map_or_else(
        || "none".to_string(),
        |event| format!("{:?} {}", event.kind, event.content),
    )
}

fn question(snapshot: &TaskSnapshot) -> String {
    snapshot
        .events
        .iter()
        .rev()
        .find(|event| event.kind == EventKind::Question)
        .map_or_else(|| "none".to_string(), |event| event.content.clone())
}

fn token_line(conn: &Connection) -> Result<String, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM token_usage", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Ok("unknown".to_string());
    }
    let (prompt, completion, cached): (Option<i64>, Option<i64>, Option<i64>) = conn
        .query_row(
            "SELECT SUM(prompt_tokens), SUM(completion_tokens), SUM(cached_tokens) FROM token_usage",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "task in={} out={} cached={}",
        fmt_token(prompt),
        fmt_token(completion),
        fmt_token(cached)
    ))
}

fn state_ledger_lines(conn: &Connection) -> Result<String, String> {
    let active = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE status = 'Active'",
    )?;
    let conflicts = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_cells WHERE key_label LIKE 'context:conflict/%'",
    )?;
    let admissions = count_table(conn, "tool_admissions")?;
    let exchanges = count_table(conn, "provider_exchanges")?;
    let artifacts = count_table(conn, "artifacts")?;
    Ok(format!(
        "state: active={active} conflicts={conflicts}\ndecision: {}\nadmissions: {admissions} exchanges: {exchanges} artifacts: {artifacts}",
        decision_line(conn)?
    ))
}

fn decision_line(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT id, operation_key, status, substr(tool_view_fingerprint, 1, 16)
         FROM runtime_decisions ORDER BY selected_at DESC, id DESC LIMIT 1",
        [],
        |row| {
            Ok(format!(
                "{} {} status={} tools={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    );
    match row {
        Ok(line) => Ok(line),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("none".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn count_table(conn: &Connection, table: &str) -> Result<i64, String> {
    count_sql(conn, &format!("SELECT COUNT(*) FROM {table}"))
}

fn count_sql(conn: &Connection, sql: &str) -> Result<i64, String> {
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn fmt_token(value: Option<i64>) -> String {
    value.map_or_else(|| "unknown".to_string(), |value| value.to_string())
}
