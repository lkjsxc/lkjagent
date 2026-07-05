use lkjagent_core::runtime_state_edge::{StateEdge, StateEdgeStatus};
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::row_json::{json_string, json_value};

pub fn insert_state_edge(
    conn: &Connection,
    case_id: Option<&str>,
    edge: &StateEdge,
) -> StoreResult<()> {
    let evidence_json = json_string(&edge.evidence_refs)?;
    let edge_json = json_string(edge)?;
    conn.execute(
        "INSERT OR REPLACE INTO state_edges
         (id, scope, case_id, from_ref_kind, from_ref_id, to_ref_kind,
          to_ref_id, relation, reason, evidence_json, edge_json, status,
          created_at, source_event_id, suppression_reason)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                 ?14, ?15)",
        params![
            edge.id,
            edge.scope,
            case_id,
            edge.from_ref.kind,
            edge.from_ref.id,
            edge.to_ref.kind,
            edge.to_ref.id,
            edge.relation.0,
            edge.reason,
            evidence_json,
            edge_json,
            format!("{:?}", edge.status),
            edge.created_at,
            edge.source_event_id,
            edge.suppression_reason,
        ],
    )?;
    Ok(())
}

pub fn state_edges(conn: &Connection, scope: &str) -> StoreResult<Vec<StateEdge>> {
    let mut statement = conn.prepare(
        "SELECT edge_json FROM state_edges
         WHERE scope = ?1 AND status = 'Active' ORDER BY relation, from_ref_kind,
         from_ref_id, to_ref_kind, to_ref_id, id",
    )?;
    let rows = statement.query_map([scope], |row| row.get::<_, String>(0))?;
    let mut edges = Vec::new();
    for row in rows {
        edges.push(json_value(&row?)?);
    }
    Ok(edges)
}

pub fn suppress_state_edge(conn: &Connection, id: &str, reason: &str) -> StoreResult<usize> {
    let changed = conn.execute(
        "UPDATE state_edges SET status = ?1, suppression_reason = ?2 WHERE id = ?3",
        params![format!("{:?}", StateEdgeStatus::Suppressed), reason, id],
    )?;
    Ok(changed)
}
