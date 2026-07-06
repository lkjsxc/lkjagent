use serde::{Deserialize, Serialize};

use crate::runtime_state::{RuntimeSnapshot, StateStatus};
use crate::runtime_state_edge::{StateEdge, StateRef};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphQueryRow {
    pub subject: String,
    pub relation: String,
    pub object: String,
    pub reason: String,
    pub evidence: Vec<String>,
}

pub fn blockers(snapshot: &RuntimeSnapshot, state_label: &str) -> Vec<GraphQueryRow> {
    rows(snapshot, |edge| {
        edge.relation.0 == "blocks" && edge.to_ref.kind == "state" && edge.to_ref.id == state_label
    })
}

pub fn conflicts(snapshot: &RuntimeSnapshot) -> Vec<GraphQueryRow> {
    rows(snapshot, |edge| edge.relation.0 == "conflicts-with")
}

pub fn stale_dependencies(snapshot: &RuntimeSnapshot) -> Vec<GraphQueryRow> {
    rows(snapshot, |edge| {
        edge.relation.0 == "depends-on" && inactive_state(snapshot, &edge.to_ref)
    })
}

pub fn lineage(snapshot: &RuntimeSnapshot, subject: &StateRef) -> Vec<GraphQueryRow> {
    rows(snapshot, |edge| {
        (edge.from_ref == *subject || edge.to_ref == *subject)
            && matches!(
                edge.relation.0.as_str(),
                "derived-from" | "supersedes" | "owns"
            )
    })
}

fn inactive_state(snapshot: &RuntimeSnapshot, reference: &StateRef) -> bool {
    if reference.kind != "state" {
        return false;
    }
    snapshot
        .cells
        .iter()
        .find(|(key, _)| key.as_label() == reference.id)
        .is_some_and(|(_, cell)| cell.status != StateStatus::Active)
}

fn rows<F>(snapshot: &RuntimeSnapshot, keep: F) -> Vec<GraphQueryRow>
where
    F: Fn(&StateEdge) -> bool,
{
    snapshot
        .active_edges()
        .into_iter()
        .filter(keep)
        .map(row)
        .collect()
}

fn row(edge: StateEdge) -> GraphQueryRow {
    GraphQueryRow {
        subject: edge.from_ref.label(),
        relation: edge.relation.0,
        object: edge.to_ref.label(),
        reason: edge.reason,
        evidence: edge
            .evidence_refs
            .into_iter()
            .map(|item| format!("{}:{}", item.source_type, item.source_id))
            .collect(),
    }
}
