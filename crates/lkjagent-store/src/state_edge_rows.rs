use lkjagent_core::model::CheckResult;
use lkjagent_core::runtime_state_edge::{
    EdgeEvidenceRef, StateEdge, StateEdgeRelation, StateEdgeStatus, StateRef,
};
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

pub fn insert_check_artifact_edges_tx(
    tx: &rusqlite::Transaction<'_>,
    task_id: i64,
    check_id: i64,
    result: &CheckResult,
    now: &str,
) -> StoreResult<()> {
    for artifact_id in &result.artifact_refs {
        let edge = check_artifact_edge(task_id, check_id, artifact_id, result, now);
        insert_state_edge_tx(tx, Some(&task_id.to_string()), &edge)?;
    }
    Ok(())
}

fn insert_state_edge_tx(
    tx: &rusqlite::Transaction<'_>,
    case_id: Option<&str>,
    edge: &StateEdge,
) -> StoreResult<()> {
    let evidence_json = json_string(&edge.evidence_refs)?;
    let edge_json = json_string(edge)?;
    tx.execute(
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

fn check_artifact_edge(
    task_id: i64,
    check_id: i64,
    artifact_id: &str,
    result: &CheckResult,
    now: &str,
) -> StateEdge {
    let check_id = check_id.to_string();
    let mut edge = StateEdge::active(
        format!("check-artifact:{task_id}:{check_id}:{artifact_id}"),
        format!("case:{task_id}"),
        StateRef::new("check_result", check_id.clone()),
        StateRef::new("artifact", artifact_id),
        StateEdgeRelation::verifies(),
        format!("check-result-{check_id}"),
    )
    .with_reason("check result verifies artifact freshness");
    edge.created_at = now.to_string();
    if let Some(fingerprint) = &result.evidence_fingerprint {
        edge.evidence_refs = vec![EdgeEvidenceRef::new("check_result", check_id, fingerprint)];
    }
    edge
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
