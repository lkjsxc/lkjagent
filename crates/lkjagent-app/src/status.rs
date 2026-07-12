use std::path::Path;

use lkjagent_core::model::{EventKind, StepState, TaskSnapshot, TaskState};
use rusqlite::Connection;

use crate::lease_status;
use crate::state::load_snapshot;

pub fn status(conn: &Connection) -> Result<String, String> {
    status_inner(conn, None)
}

pub fn status_with_roots(conn: &Connection, data_dir: &Path) -> Result<String, String> {
    status_inner(conn, Some(data_dir))
}

fn status_inner(conn: &Connection, data_dir: Option<&Path>) -> Result<String, String> {
    let snapshot = load_snapshot(conn).map_err(|error| error.to_string())?;
    let pending =
        lkjagent_store::plan_hydrate::pending_count(conn).map_err(|error| error.to_string())?;
    let tokens = crate::lease_status::token_line(conn)?;
    let ledger = state_ledger_lines(conn)?;
    let lease = lease_status::line(conn)?;
    let roots = match data_dir {
        Some(data_dir) => {
            let workspace = crate::config::workspace_root(data_dir)?;
            format!(
                "roots: data={} workspace={} workspace_present={}",
                data_dir.display(),
                workspace.display(),
                workspace.is_dir()
            )
        }
        None => "roots: unavailable".to_string(),
    };
    Ok(match snapshot {
        Some(snapshot) => format!(
            "{}\n{}\n{}\n{}",
            render_status_with(&snapshot, pending, &tokens),
            lease,
            roots,
            ledger
        ),
        None => format!(
            "daemon: idle\nmatter: none\noperation: none\nlast: none\nquestion: none\nqueue: {pending} pending\ntokens: {tokens}\n{lease}\n{roots}\n{ledger}"
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
        || "operation: none".to_string(),
        |step| {
            format!(
                "operation: {}/{} {:?} attempt {}/3",
                step.ordinal,
                snapshot.steps.len(),
                step.kind,
                step.attempts_used
            )
        },
    );
    format!(
        "daemon: {daemon}\nmatter: {} {:?} {:?} budget {}/{}\n{}\nlast: {}\nquestion: {}\nqueue: {pending} pending\ntokens: {tokens}",
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
        "matter {} {:?}",
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
        "transcript\n{}\n---\nmatter\n{}",
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
    let observations = count_table(conn, "observations")?;
    let exchanges = count_table(conn, "provider_exchanges")?;
    let artifacts = count_table(conn, "artifacts")?;
    let blocked = count_sql(conn, "SELECT COUNT(*) FROM tasks WHERE state = 'blocked'")?;
    let refused = count_sql(
        conn,
        "SELECT COUNT(*) FROM tool_admissions WHERE status = 'Rejected'",
    )?;
    let stale = count_sql(
        conn,
        "SELECT COUNT(*) FROM state_edges WHERE status = 'Suppressed'",
    )?;
    Ok(format!(
        "state: active={active} conflicts={conflicts}\ndecision: {}\ncontext_lanes: {}\nadmissions: {admissions} observations: {observations} exchanges: {exchanges} artifacts: {artifacts}\nevidence: blocked={blocked} refused={refused} stale_edges={stale}",
        decision_line(conn)?,
        context_lanes(conn)?
    ))
}

fn context_lanes(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT reason FROM prompt_cards WHERE kind = 'facts' ORDER BY created_at DESC, id DESC LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    );
    match row {
        Ok(line) => Ok(line),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("none".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn decision_line(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT id, operation_key, status, substr(context_frame_fingerprint, 1, 16),
         substr(tool_view_fingerprint, 1, 16)
         FROM runtime_decisions ORDER BY selected_at DESC, id DESC LIMIT 1",
        [],
        |row| {
            Ok(format!(
                "{} {} status={} ctx={} tools={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?
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
