use serde::{Deserialize, Serialize};

use crate::runtime_state::{RuntimeSnapshot, StateCell, StateKey, StateStatus};

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
