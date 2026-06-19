mod links;

pub use links::{link_memory, memory_links_for_case, GraphMemoryLinkRow};

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphCaseRow {
    pub id: i64,
    pub objective: String,
    pub family: String,
    pub phase: String,
    pub active_node: String,
    pub status: String,
    pub plan: String,
    pub evidence_requirements: Vec<String>,
    pub selected_packages: Vec<String>,
    pub pending_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEvidenceRow {
    pub requirement: String,
    pub kind: String,
    pub summary: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCase<'a> {
    pub objective: &'a str,
    pub family: &'a str,
    pub phase: &'a str,
    pub active_node: &'a str,
    pub plan: &'a str,
    pub evidence_requirements: &'a [String],
    pub selected_packages: &'a [String],
    pub pending_checks: &'a [String],
}

pub fn open_case(conn: &Connection, case: OpenCase<'_>, now: &str) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO graph_cases
         (objective, family, phase, active_node, status, plan,
          evidence_requirements, selected_packages, pending_checks, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, 'active', ?5, ?6, ?7, ?8, ?9, ?9)",
        params![
            case.objective,
            case.family,
            case.phase,
            case.active_node,
            case.plan,
            join(case.evidence_requirements),
            join(case.selected_packages),
            join(case.pending_checks),
            now
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn active_case(conn: &Connection) -> StoreResult<Option<GraphCaseRow>> {
    let row = conn
        .query_row(
            "SELECT id, objective, family, phase, active_node, status, plan,
                    evidence_requirements, selected_packages, pending_checks
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

pub fn record_event(
    conn: &Connection,
    case_id: i64,
    kind: &str,
    node: &str,
    phase: &str,
    summary: &str,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO graph_events (case_id, kind, node, phase, summary, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![case_id, kind, node, phase, summary, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_evidence(
    conn: &Connection,
    case_id: i64,
    evidence: &GraphEvidenceRow,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO graph_evidence (case_id, requirement, kind, summary, path, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            case_id,
            evidence.requirement,
            evidence.kind,
            evidence.summary,
            evidence.path,
            now
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn evidence_for_case(conn: &Connection, case_id: i64) -> StoreResult<Vec<GraphEvidenceRow>> {
    let mut statement = conn.prepare(
        "SELECT requirement, kind, summary, path
         FROM graph_evidence
         WHERE case_id = ?1
         ORDER BY id",
    )?;
    let rows = statement.query_map(params![case_id], |row| {
        Ok(GraphEvidenceRow {
            requirement: row.get(0)?,
            kind: row.get(1)?,
            summary: row.get(2)?,
            path: row.get(3)?,
        })
    })?;
    collect_rows(rows)
}

fn read_case_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<GraphCaseRow> {
    Ok(GraphCaseRow {
        id: row.get(0)?,
        objective: row.get(1)?,
        family: row.get(2)?,
        phase: row.get(3)?,
        active_node: row.get(4)?,
        status: row.get(5)?,
        plan: row.get(6)?,
        evidence_requirements: split(&row.get::<_, String>(7)?),
        selected_packages: split(&row.get::<_, String>(8)?),
        pending_checks: split(&row.get::<_, String>(9)?),
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

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> StoreResult<Vec<T>> {
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}
