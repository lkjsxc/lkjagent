pub mod artifacts;
pub mod cases;
pub mod context;
pub mod documents;
pub mod faults;
mod links;
pub mod notes;
pub mod plan;
pub mod snapshots;
pub mod state_tracks;
pub mod transitions;

pub use cases::{active_case, open_case, update_case, GraphCaseRow, OpenCase};
pub use links::{link_memory, memory_links_for_case, GraphMemoryLinkRow};

use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEvidenceRow {
    pub requirement: String,
    pub kind: String,
    pub summary: String,
    pub path: Option<String>,
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

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> StoreResult<Vec<T>> {
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}
