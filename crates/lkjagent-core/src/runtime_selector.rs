use crate::runtime_candidate::{selected_candidate, selector_candidates, SelectorCandidate};
use serde_json::Value;

use crate::runtime_decision::{
    EffectCommand, OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use crate::runtime_fingerprint::FingerprintError;
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_state::{RuntimeSnapshot, StateCell};

pub fn select_runtime_decision(
    snapshot: &RuntimeSnapshot,
    decision_id: &str,
    context_frame_fingerprint: &str,
    unfinished: &[RuntimeDecision],
) -> Result<RuntimeDecision, FingerprintError> {
    if let Some(decision) = unfinished.first() {
        return Ok(decision.clone());
    }
    let candidate = selected_candidate(snapshot);
    let operation = candidate.operation;
    let mut decision = RuntimeDecision::new(
        decision_id,
        snapshot.case_id.clone(),
        OperationKey(operation.key),
        operation.tool_view,
        operation.expected_envelope,
    );
    decision.selected_state_key = candidate.state_key.as_ref().map(|key| key.as_label());
    decision.effect_command = operation.effect_command;
    let snapshot_fingerprint = snapshot.fingerprint()?;
    decision.snapshot_fingerprint = snapshot_fingerprint.clone();
    decision.state_vector_fingerprint = snapshot_fingerprint;
    decision.context_frame_fingerprint = context_frame_fingerprint.to_string();
    decision.model_budget_tokens = operation.model_budget_tokens;
    decision.evidence_requirements = operation.evidence_requirements;
    decision
        .evidence_requirements
        .insert(0, format!("selector:{}", candidate.reason));
    if candidate
        .state_key
        .as_ref()
        .and_then(|key| snapshot.cells.get(key))
        .map(|cell| payload_tool_budget_exhausted(&payload_value(cell)))
        .unwrap_or(false)
    {
        decision
            .evidence_requirements
            .push("tool-budget:suppressed".to_string());
    }
    decision.recovery_policy = operation.recovery_policy;
    Ok(decision)
}

pub fn select_operation(snapshot: &RuntimeSnapshot) -> RuntimeOperation {
    selected_candidate(snapshot).operation
}

pub fn candidates(snapshot: &RuntimeSnapshot) -> Vec<SelectorCandidate> {
    selector_candidates(snapshot)
}

pub(crate) fn apply_edge_blocks(
    snapshot: &RuntimeSnapshot,
    mut item: SelectorCandidate,
) -> SelectorCandidate {
    let Some(key) = &item.state_key else {
        return item;
    };
    let label = key.as_label();
    item.blocked_by = snapshot
        .active_edges()
        .into_iter()
        .filter(|edge| edge.relation.0 == "blocks")
        .filter(|edge| edge.to_ref.kind == "state" && edge.to_ref.id == label)
        .map(|edge| edge.id)
        .collect();
    item
}

pub(crate) fn payload_value(cell: &StateCell) -> Value {
    serde_json::from_str(&cell.payload_json).unwrap_or(Value::Null)
}

pub(crate) fn payload_envelope(payload: &Value) -> OutputEnvelope {
    match payload_text(payload, "expected_envelope") {
        Some("Content") => OutputEnvelope::Content,
        Some("Plan") => OutputEnvelope::Plan,
        Some("Action") => OutputEnvelope::Action,
        Some("Message") => OutputEnvelope::Message,
        Some("Verdict") => OutputEnvelope::Verdict,
        _ => OutputEnvelope::None,
    }
}

pub(crate) fn payload_tool_view(payload: &Value) -> ToolSetView {
    if payload_tool_budget_exhausted(payload) {
        return ToolSetView::empty();
    }
    let entries = payload
        .get("tool_view")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(tool_entry).collect())
        .unwrap_or_default();
    ToolSetView::new(entries)
}

pub(crate) fn payload_text<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

pub(crate) fn payload_number(payload: &Value, key: &str) -> Option<u32> {
    payload
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
}

pub(crate) fn payload_tool_budget_exhausted(payload: &Value) -> bool {
    payload_number(payload, "tool_budget_remaining") == Some(0)
}

pub(crate) fn payload_tier(payload: &Value, default: u8) -> u8 {
    payload_number(payload, "selector_tier")
        .and_then(|value| u8::try_from(value).ok())
        .unwrap_or(default)
}

pub(crate) fn payload_evidence_requirements(payload: &Value) -> Vec<String> {
    strings(payload.get("evidence_requirements"))
}

pub(crate) fn payload_effect_command(payload: &Value) -> Option<EffectCommand> {
    let value = payload.get("effect_command")?;
    Some(EffectCommand {
        name: payload_text(value, "name")?.to_string(),
        path: payload_text(value, "path").map(str::to_string),
        content: payload_text(value, "content").map(str::to_string),
    })
}

fn tool_entry(value: &Value) -> Option<ToolViewEntry> {
    let mut entry = ToolViewEntry::new(
        payload_text(value, "name")?,
        payload_text(value, "purpose").unwrap_or(""),
    );
    entry.required_params = strings(value.get("required_params"));
    entry.optional_params = strings(value.get("optional_params"));
    Some(entry)
}

fn strings(value: Option<&Value>) -> Vec<String> {
    let mut strings: Vec<String> = value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    strings.sort();
    strings
}
