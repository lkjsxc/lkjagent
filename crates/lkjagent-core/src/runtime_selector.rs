use serde_json::Value;

use crate::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_state::{RuntimeSnapshot, StateCell};

pub fn select_runtime_decision(
    snapshot: &RuntimeSnapshot,
    decision_id: &str,
    unfinished: &[RuntimeDecision],
) -> Result<RuntimeDecision, FingerprintError> {
    if let Some(decision) = unfinished.first() {
        return Ok(decision.clone());
    }
    let operation = select_operation(snapshot);
    let mut decision = RuntimeDecision::new(
        decision_id,
        snapshot.case_id.clone(),
        OperationKey(operation.key),
        operation.tool_view,
        operation.expected_envelope,
    );
    let snapshot_fingerprint = snapshot.fingerprint()?;
    decision.snapshot_fingerprint = snapshot_fingerprint.clone();
    decision.state_vector_fingerprint = snapshot_fingerprint;
    decision.context_frame_fingerprint = context_fingerprint(snapshot)?;
    decision.model_budget_tokens = operation.model_budget_tokens;
    decision.evidence_requirements = operation.evidence_requirements;
    decision.recovery_policy = operation.recovery_policy;
    Ok(decision)
}

pub fn select_operation(snapshot: &RuntimeSnapshot) -> RuntimeOperation {
    if cell(snapshot, "case", "owner-intake").is_some() {
        return RuntimeOperation::model_free("owner.intake", vec!["queue row".to_string()]);
    }
    if cell(snapshot, "case", "waiting-answer").is_some() {
        return RuntimeOperation::model_free("owner.answer", vec!["answer row".to_string()]);
    }
    if let Some(cell) = namespace_cell(snapshot, "recovery") {
        return RuntimeOperation::model_free(
            format!("recovery.handle/{}", cell.key.name),
            evidence(cell),
        );
    }
    if let Some(cell) = namespace_cell(snapshot, "effect") {
        return RuntimeOperation::model_free(
            format!("effect.run/{}", cell.key.name),
            evidence(cell),
        );
    }
    if let Some(cell) = namespace_cell(snapshot, "model") {
        return RuntimeOperation::model_call(
            format!("model.call/{}", cell.key.name),
            envelope(cell),
            tool_view(cell),
            model_budget(cell),
            evidence(cell),
        );
    }
    if let Some(cell) = namespace_cell(snapshot, "check") {
        return RuntimeOperation::model_free(
            format!("check.run/{}", cell.key.name),
            evidence(cell),
        );
    }
    if cell(snapshot, "completion", "close-candidate").is_some() {
        return RuntimeOperation::model_free("completion.close", vec!["fresh checks".to_string()]);
    }
    RuntimeOperation::idle()
}

fn cell<'a>(snapshot: &'a RuntimeSnapshot, namespace: &str, name: &str) -> Option<&'a StateCell> {
    snapshot
        .active_cells()
        .into_iter()
        .find(|cell| cell.key.namespace == namespace && cell.key.name == name)
}

fn namespace_cell<'a>(snapshot: &'a RuntimeSnapshot, namespace: &str) -> Option<&'a StateCell> {
    snapshot
        .active_cells()
        .into_iter()
        .find(|cell| cell.key.namespace == namespace)
}

fn envelope(cell: &StateCell) -> OutputEnvelope {
    envelope_from_value(&payload(cell)).unwrap_or(OutputEnvelope::Content)
}

fn envelope_from_value(payload: &Value) -> Option<OutputEnvelope> {
    match text(payload, "expected_envelope")? {
        "Content" => Some(OutputEnvelope::Content),
        "Plan" => Some(OutputEnvelope::Plan),
        "Action" => Some(OutputEnvelope::Action),
        "Message" => Some(OutputEnvelope::Message),
        "Verdict" => Some(OutputEnvelope::Verdict),
        "None" => Some(OutputEnvelope::None),
        _ => None,
    }
}

fn tool_view(cell: &StateCell) -> ToolSetView {
    tool_view_from_value(&payload(cell))
}

fn tool_view_from_value(payload: &Value) -> ToolSetView {
    let entries = payload
        .get("tool_view")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(tool_entry).collect())
        .unwrap_or_default();
    ToolSetView::new(entries)
}

fn tool_entry(value: &Value) -> Option<ToolViewEntry> {
    let mut entry = ToolViewEntry::new(text(value, "name")?, text(value, "purpose").unwrap_or(""));
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

fn model_budget(cell: &StateCell) -> Option<u32> {
    number(&payload(cell), "model_budget_tokens")
}

fn evidence(cell: &StateCell) -> Vec<String> {
    if cell.evidence_refs.is_empty() {
        return vec![cell.key.as_label()];
    }
    cell.evidence_refs
        .iter()
        .map(|item| format!("{}:{}", item.source_type, item.source_id))
        .collect()
}

fn payload(cell: &StateCell) -> Value {
    serde_json::from_str(&cell.payload_json).unwrap_or(Value::Null)
}

fn text<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

fn number(payload: &Value, key: &str) -> Option<u32> {
    payload
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
}

fn context_fingerprint(snapshot: &RuntimeSnapshot) -> Result<String, FingerprintError> {
    stable_fingerprint(&serde_json::json!({"case_id": snapshot.case_id, "context": []}))
}
