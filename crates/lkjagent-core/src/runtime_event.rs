use crate::runtime_eligibility::causal_number;
use crate::runtime_state::{
    CurrentTime, MatterLifecycle, RuntimePhase, RuntimeSnapshot, RuntimeState, StateCell, StateKey,
    StateStatus,
};
use crate::runtime_state_edge::{StateEdge, StateEdgeStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReduceFault {
    MatterIdentity { expected: String, actual: String },
    CausalSequence { expected: u64, actual: Option<u64> },
    InvalidCurrentTime(String),
}

#[rustfmt::skip]
pub fn reduce(snapshot: RuntimeSnapshot, event: RuntimeEvent, now: CurrentTime) -> Result<RuntimeState, ReduceFault> {
    if snapshot.case_id != event.case_id {
        return Err(ReduceFault::MatterIdentity { expected: snapshot.case_id, actual: event.case_id });
    }
    if crate::runtime_eligibility::utc_millis(now.as_str()).is_none() { return Err(ReduceFault::InvalidCurrentTime(now.0)); }
    let mut state = RuntimeState::from_snapshot(snapshot); let actual = causal_number(&event.id);
    if actual != Some(state.causal_sequence + 1) {
        return Err(ReduceFault::CausalSequence { expected: state.causal_sequence + 1, actual });
    }
    state.snapshot = apply_patch(&state.snapshot, &reduce_event(&state.snapshot, &event));
    state.causal_sequence = actual.unwrap_or(state.causal_sequence);
    invalidate_revision_evidence(&mut state.snapshot, &event); derive_projections(&mut state); Ok(state)
}

#[rustfmt::skip]
pub fn reduce_event(_snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> StatePatch {
    let operations = match &event.payload {
        RuntimeEventPayload::UpsertCell(cell) => vec![StatePatchOp::Upsert(*cell.clone())],
        RuntimeEventPayload::SuppressCell { key, reason } => status_op(key, StateStatus::Suppressed, reason, event),
        RuntimeEventPayload::ResolveCell { key, reason } => status_op(key, StateStatus::Resolved, reason, event),
        RuntimeEventPayload::BlockCell { key, reason } => status_op(key, StateStatus::Blocked, reason, event),
        RuntimeEventPayload::UpsertEdge(edge) => vec![StatePatchOp::UpsertEdge(*edge.clone())],
        RuntimeEventPayload::SuppressEdge { edge_id, reason } => vec![StatePatchOp::SetEdgeStatus {
            edge_id: edge_id.clone(), status: StateEdgeStatus::Suppressed, reason: reason.clone(), source_event_id: event.id.clone() }],
        RuntimeEventPayload::Unknown { .. } => Vec::new(),
    }; StatePatch { event_id: event.id.clone(), operations }
}

pub fn apply_patch(snapshot: &RuntimeSnapshot, patch: &StatePatch) -> RuntimeSnapshot {
    let mut next = snapshot.clone();
    for operation in &patch.operations {
        apply_operation(&mut next, operation);
    }
    next
}
#[rustfmt::skip]
fn apply_operation(next: &mut RuntimeSnapshot, operation: &StatePatchOp) {
    match operation {
        StatePatchOp::Upsert(cell) => { next.cells.insert(cell.key.clone(), cell.clone()); }
        StatePatchOp::SetStatus { key, status, updated_at, source_event_id, .. } => if let Some(cell) = next.cells.get_mut(key) {
            cell.status = *status; cell.updated_at.clone_from(updated_at); cell.source_event_id.clone_from(source_event_id); },
        StatePatchOp::UpsertEdge(edge) => { next.edges.insert(edge.id.clone(), edge.clone()); }
        StatePatchOp::SetEdgeStatus { edge_id, status, reason, source_event_id } => if let Some(edge) = next.edges.get_mut(edge_id) {
            edge.status = *status; edge.suppression_reason = Some(reason.clone()); edge.source_event_id.clone_from(source_event_id); },
    }
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
        reason: reason.into(),
        updated_at: event.created_at.clone(),
        source_event_id: event.id.clone(),
    }]
}
#[rustfmt::skip]
fn invalidate_revision_evidence(snapshot: &mut RuntimeSnapshot, event: &RuntimeEvent) {
    let RuntimeEventPayload::UpsertCell(source) = &event.payload else { return }; if source.key.namespace != "source" { return; }
    let Some(current_revision) = revision(&source.payload_json) else { return };
    for cell in snapshot.cells.values_mut() {
        if matches!(cell.key.namespace.as_str(), "check" | "evidence")
            && revision(&cell.payload_json).is_some_and(|bound| bound != current_revision) {
            cell.status = StateStatus::Suppressed; cell.updated_at.clone_from(&event.created_at); cell.source_event_id.clone_from(&event.id);
        }
    }
}
fn revision(json: &str) -> Option<String> {
    serde_json::from_str::<Value>(json)
        .ok()?
        .get("revision")?
        .as_str()
        .map(str::to_string)
}
#[rustfmt::skip]
pub(crate) fn derive_projections(state: &mut RuntimeState) {
    let active = state.snapshot.active_cells();
    state.obligations = active.iter().filter(|cell| cell.key.namespace == "need").map(|cell| cell.key.name.clone()).collect();
    state.obligations.sort(); state.obligations.dedup();
    let has = |namespace: &str, name: &str| active.iter().any(|cell| cell.key.namespace == namespace && cell.key.name == name);
    let any = |namespace: &str| active.iter().any(|cell| cell.key.namespace == namespace);
    state.lifecycle = if has("matter", "closed") { MatterLifecycle::Closed }
        else if any("block") || active.iter().any(|cell| cell.status == StateStatus::Blocked) { MatterLifecycle::Blocked }
        else if has("need", "owner-fact") { MatterLifecycle::Waiting } else { MatterLifecycle::Open };
    state.phase = if has("response", "final-persisted") { RuntimePhase::Idle }
        else if has("check", "current-passed") { RuntimePhase::Respond }
        else if has("edit", "committed") { RuntimePhase::Review }
        else if has("source", "current") || any("fault") { RuntimePhase::Modify } else { RuntimePhase::Orient };
}
