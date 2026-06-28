use lkjagent_graph::{TaskGraphState, TransitionDecision};
use rusqlite::Connection;

use super::super::graph_policy::completion_decision;
use crate::error::RuntimeResult;
use crate::graph_state::active_state;

pub(super) struct GraphSnapshotFields {
    pub case_id: Option<i64>,
    pub node: Option<String>,
    pub phase: Option<String>,
    pub artifact_root: Option<String>,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
}

pub(super) fn graph_snapshot(conn: &Connection) -> RuntimeResult<GraphSnapshotFields> {
    let Some(graph) = active_state(conn)? else {
        return Ok(GraphSnapshotFields {
            case_id: None,
            node: None,
            phase: None,
            artifact_root: None,
            required_evidence: Vec::new(),
            missing_evidence: Vec::new(),
        });
    };
    Ok(GraphSnapshotFields {
        case_id: graph.case_id,
        node: Some(graph.active_node.0.to_string()),
        phase: Some(graph.phase.as_str().to_string()),
        artifact_root: graph.document.as_ref().map(|doc| doc.root.clone()),
        required_evidence: graph.evidence.requirement_ids(),
        missing_evidence: graph_missing_evidence(conn, &graph),
    })
}

fn graph_missing_evidence(conn: &Connection, graph: &TaskGraphState) -> Vec<String> {
    match completion_decision(conn, graph) {
        TransitionDecision::Admit { .. } => Vec::new(),
        TransitionDecision::Defer { missing } => missing,
        TransitionDecision::Recover { reason, .. } | TransitionDecision::Refuse { reason } => {
            vec![reason]
        }
    }
}
