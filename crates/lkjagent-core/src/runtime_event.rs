use serde::{Deserialize, Serialize};

use crate::runtime_state::{RuntimeSnapshot, StateCell, StateKey, StateStatus};
use crate::runtime_state_edge::{StateEdge, StateEdgeStatus};

pub const RUNTIME_EVENT_KINDS: &[&str] = &[
    "owner-turn",
    "wake",
    "provider-outcome",
    "effect-outcome",
    "file-change",
];
pub const STATE_TRANSITION_EVENTS: &[&str] = &[
    "matter-opened",
    "source-need-met",
    "revision-observed",
    "measured-difference",
    "obligations-met",
    "close-eligible",
    "fault-recorded",
    "question-persisted",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub id: String,
    pub case_id: String,
    pub kind: String,
    pub payload: RuntimeEventPayload,
    pub source: String,
    pub created_at: String,
    pub decision_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeEventPayload {
    UpsertCell(Box<StateCell>),
    SuppressCell { key: StateKey, reason: String },
    ResolveCell { key: StateKey, reason: String },
    BlockCell { key: StateKey, reason: String },
    UpsertEdge(Box<StateEdge>),
    SuppressEdge { edge_id: String, reason: String },
    Unknown { schema: String, json: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePatch {
    pub event_id: String,
    pub operations: Vec<StatePatchOp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatePatchOp {
    Upsert(StateCell),
    SetStatus {
        key: StateKey,
        status: StateStatus,
        reason: String,
        updated_at: String,
        source_event_id: String,
    },
    UpsertEdge(StateEdge),
    SetEdgeStatus {
        edge_id: String,
        status: StateEdgeStatus,
        reason: String,
        source_event_id: String,
    },
}

pub fn reduce_event(_snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> StatePatch {
    let operations = match &event.payload {
        RuntimeEventPayload::UpsertCell(cell) => vec![StatePatchOp::Upsert(cell.as_ref().clone())],
        RuntimeEventPayload::SuppressCell { key, reason } => {
            status_op(key, StateStatus::Suppressed, reason, event)
        }
        RuntimeEventPayload::ResolveCell { key, reason } => {
            status_op(key, StateStatus::Resolved, reason, event)
        }
        RuntimeEventPayload::BlockCell { key, reason } => {
            status_op(key, StateStatus::Blocked, reason, event)
        }
        RuntimeEventPayload::UpsertEdge(edge) => {
            vec![StatePatchOp::UpsertEdge(edge.as_ref().clone())]
        }
        RuntimeEventPayload::SuppressEdge { edge_id, reason } => {
            vec![StatePatchOp::SetEdgeStatus {
                edge_id: edge_id.clone(),
                status: StateEdgeStatus::Suppressed,
                reason: reason.clone(),
                source_event_id: event.id.clone(),
            }]
        }
        RuntimeEventPayload::Unknown { .. } => Vec::new(),
    };
    StatePatch {
        event_id: event.id.clone(),
        operations,
    }
}

pub fn apply_patch(snapshot: &RuntimeSnapshot, patch: &StatePatch) -> RuntimeSnapshot {
    let mut next = snapshot.clone();
    for operation in &patch.operations {
        match operation {
            StatePatchOp::Upsert(cell) => {
                next.cells.insert(cell.key.clone(), cell.clone());
            }
            StatePatchOp::SetStatus {
                key,
                status,
                updated_at,
                source_event_id,
                ..
            } => {
                if let Some(cell) = next.cells.get_mut(key) {
                    cell.status = *status;
                    cell.updated_at = updated_at.clone();
                    cell.source_event_id = source_event_id.clone();
                }
            }
            StatePatchOp::UpsertEdge(edge) => {
                next.edges.insert(edge.id.clone(), edge.clone());
            }
            StatePatchOp::SetEdgeStatus {
                edge_id,
                status,
                reason,
                source_event_id,
            } => {
                if let Some(edge) = next.edges.get_mut(edge_id) {
                    edge.status = *status;
                    edge.suppression_reason = Some(reason.clone());
                    edge.source_event_id = source_event_id.clone();
                }
            }
        }
    }
    next
}

fn status_op(
    key: &StateKey,
    status: StateStatus,
    reason: &str,
    event: &RuntimeEvent,
) -> Vec<StatePatchOp> {
    vec![StatePatchOp::SetStatus {
        key: key.clone(),
        status,
        reason: reason.to_string(),
        updated_at: event.created_at.clone(),
        source_event_id: event.id.clone(),
    }]
}
