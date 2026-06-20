use rusqlite::{params, Connection, OptionalExtension};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphCaseRow {
    pub id: i64,
    pub objective: String,
    pub raw_owner_text: String,
    pub objective_version: u32,
    pub family: String,
    pub subroute: String,
    pub route_reason: String,
    pub phase: String,
    pub active_node: String,
    pub status: String,
    pub plan: String,
    pub evidence_requirements: Vec<String>,
    pub selected_packages: Vec<String>,
    pub pending_checks: Vec<String>,
    pub next_action_class: String,
    pub context_pressure: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCase {
    pub objective: String,
    pub raw_owner_text: String,
    pub objective_version: u32,
    pub family: String,
    pub subroute: String,
    pub route_reason: String,
    pub phase: String,
    pub active_node: String,
    pub plan: String,
    pub evidence_requirements: Vec<String>,
    pub selected_packages: Vec<String>,
    pub pending_checks: Vec<String>,
    pub next_action_class: String,
    pub context_pressure: String,
}

pub fn open_case(conn: &Connection, case: OpenCase, now: &str) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO graph_cases
         (objective, raw_owner_text, objective_version, family, subroute,
          route_reason, phase, active_node, status, plan, evidence_requirements,
          selected_packages, pending_checks, next_action_class, context_pressure,
          created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'active', ?9, ?10, ?11,
                 ?12, ?13, ?14, ?15, ?15)",
        params![
            case.objective,
            case.raw_owner_text,
            i64::from(case.objective_version),
            case.family,
            case.subroute,
            case.route_reason,
            case.phase,
            case.active_node,
            case.plan,
            join(&case.evidence_requirements),
            join(&case.selected_packages),
            join(&case.pending_checks),
            case.next_action_class,
            case.context_pressure,
            now
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn active_case(conn: &Connection) -> StoreResult<Option<GraphCaseRow>> {
    let row = conn
        .query_row(
            "SELECT id, objective, raw_owner_text, objective_version, family,
                    subroute, route_reason, phase, active_node, status, plan,
                    evidence_requirements, selected_packages, pending_checks,
                    next_action_class, context_pressure
             FROM graph_cases
             WHERE status = 'active'
             ORDER BY id DESC
             LIMIT 1",
            [],
            read_case_row,
        )
        .optional()?;
    Ok(row)
}

pub fn update_case(
    conn: &Connection,
    id: i64,
    phase: &str,
    active_node: &str,
    status: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE graph_cases
         SET phase = ?2, active_node = ?3, status = ?4, updated_at = ?5
         WHERE id = ?1",
        params![id, phase, active_node, status, now],
    )?;
    Ok(())
}

fn read_case_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<GraphCaseRow> {
    Ok(GraphCaseRow {
        id: row.get(0)?,
        objective: row.get(1)?,
        raw_owner_text: row.get(2)?,
        objective_version: row.get::<_, i64>(3)?.max(1) as u32,
        family: row.get(4)?,
        subroute: row.get(5)?,
        route_reason: row.get(6)?,
        phase: row.get(7)?,
        active_node: row.get(8)?,
        status: row.get(9)?,
        plan: row.get(10)?,
        evidence_requirements: split(&row.get::<_, String>(11)?),
        selected_packages: split(&row.get::<_, String>(12)?),
        pending_checks: split(&row.get::<_, String>(13)?),
        next_action_class: row.get(14)?,
        context_pressure: row.get(15)?,
    })
}

fn join(values: &[String]) -> String {
    values.join("\n")
}

fn split(value: &str) -> Vec<String> {
    value
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
