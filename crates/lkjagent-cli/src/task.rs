use std::path::Path;

use rusqlite::{params, OptionalExtension};

use crate::args::TaskCommand;
use crate::error::CliError;
use crate::store::open_store;

struct TaskRow {
    id: i64,
    objective: String,
    family: String,
    phase: String,
    active_node: String,
    status: String,
    evidence_requirements: String,
    pending_checks: String,
    next_action_class: String,
}

pub fn task(data_dir: &Path, command: TaskCommand) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    match command {
        TaskCommand::List { status, limit } => render_list(&conn, status.as_deref(), limit),
        TaskCommand::Show { id } => render_show(&conn, id),
    }
}

fn render_list(
    conn: &rusqlite::Connection,
    status: Option<&str>,
    limit: Option<usize>,
) -> Result<String, CliError> {
    let rows = list_rows(conn, status, limit)?;
    let mut lines = vec![format!("task_rows={}", rows.len())];
    for row in rows {
        lines.push(format!(
            "id={} status={} family={} phase={} node={} next={} objective={}",
            row.id,
            row.status,
            row.family,
            row.phase,
            row.active_node,
            row.next_action_class,
            preview(&row.objective)
        ));
    }
    Ok(lines.join("\n"))
}

fn render_show(conn: &rusqlite::Connection, id: i64) -> Result<String, CliError> {
    let Some(row) = row_by_id(conn, id)? else {
        return Err(CliError::failure(format!("task_not_found={id}")));
    };
    let mut lines = vec![
        format!("task_id={}", row.id),
        format!("status={}", row.status),
        format!("family={}", row.family),
        format!("phase={}", row.phase),
        format!("active_node={}", row.active_node),
        format!("next_action_class={}", row.next_action_class),
        format!(
            "evidence_requirements={}",
            list_text(&row.evidence_requirements)
        ),
        format!("pending_checks={}", list_text(&row.pending_checks)),
        format!("objective={}", row.objective),
    ];
    lines.extend(artifact_progress(conn, id)?);
    Ok(lines.join("\n"))
}

fn list_rows(
    conn: &rusqlite::Connection,
    status: Option<&str>,
    limit: Option<usize>,
) -> Result<Vec<TaskRow>, CliError> {
    let limit = limit.unwrap_or(20).min(200) as i64;
    let mut statement = match status {
        Some(_) => conn.prepare(LIST_WITH_STATUS)?,
        None => conn.prepare(LIST_ALL)?,
    };
    match status {
        Some(status) => collect_rows(statement.query_map(params![status, limit], read_row)?),
        None => collect_rows(statement.query_map(params![limit], read_row)?),
    }
}

fn row_by_id(conn: &rusqlite::Connection, id: i64) -> Result<Option<TaskRow>, CliError> {
    Ok(conn
        .query_row(SHOW_BY_ID, params![id], read_row)
        .optional()?)
}

fn collect_rows<F>(rows: rusqlite::MappedRows<'_, F>) -> Result<Vec<TaskRow>, CliError>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<TaskRow>,
{
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskRow> {
    Ok(TaskRow {
        id: row.get(0)?,
        objective: row.get(1)?,
        family: row.get(2)?,
        phase: row.get(3)?,
        active_node: row.get(4)?,
        status: row.get(5)?,
        evidence_requirements: row.get(6)?,
        pending_checks: row.get(7)?,
        next_action_class: row.get(8)?,
    })
}

fn preview(value: &str) -> String {
    let mut out = value.chars().take(80).collect::<String>();
    if value.chars().count() > 80 {
        out.push_str("...");
    }
    out.replace('\n', " ")
}

fn artifact_progress(conn: &rusqlite::Connection, id: i64) -> Result<Vec<String>, CliError> {
    let Some(row) = lkjagent_store::artifact_graph::readiness_for_case(conn, id)? else {
        return Ok(vec!["artifact.progress=none".to_string()]);
    };
    Ok(vec![
        format!("artifact.root={}", row.root),
        format!("artifact.profile={}", row.profile),
        format!("artifact.plan_status={}", row.plan_status),
        format!("artifact.atom_total={}", row.atom_total),
        format!("artifact.atom_ready={}", row.atom_ready),
        format!("artifact.atom_missing={}", row.atom_missing),
        format!("artifact.next_atom={}", row.next_atom_id),
        format!("artifact.next_path={}", row.next_path),
        format!("artifact.active_contract={}", row.active_contract_id),
        format!("artifact.measured_total={}", row.measured_total),
        format!("artifact.accepted_floor={}", row.accepted_floor),
        format!("artifact.assembly_pending={}", row.assembly_pending),
        format!("artifact.readiness={}", row.status),
        format!(
            "artifact.completion_blockers={}",
            row.completion_blockers.replace('\n', ";")
        ),
    ])
}

fn list_text(value: &str) -> String {
    let parts = value
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(",")
    }
}

const LIST_ALL: &str = "SELECT id, objective, family, phase, active_node, status,
    evidence_requirements, pending_checks, next_action_class
    FROM graph_cases ORDER BY id DESC LIMIT ?1";
const LIST_WITH_STATUS: &str = "SELECT id, objective, family, phase, active_node, status,
    evidence_requirements, pending_checks, next_action_class
    FROM graph_cases WHERE status = ?1 ORDER BY id DESC LIMIT ?2";
const SHOW_BY_ID: &str = "SELECT id, objective, family, phase, active_node, status,
    evidence_requirements, pending_checks, next_action_class
    FROM graph_cases WHERE id = ?1";
