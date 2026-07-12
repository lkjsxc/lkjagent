use crate::runtime_decision::{EffectCommand, OutputEnvelope, ToolSetView, ToolViewEntry};
use crate::runtime_eligibility::causal_number;
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_selector as selector;
use crate::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
use crate::runtime_tool_catalog::{descriptor_entry, explore_catalog};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorCandidate {
    pub operation: RuntimeOperation,
    pub state_key: Option<StateKey>,
    pub reason: String,
    pub blocked_by: Vec<String>,
    score: CandidateScore,
}
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CandidateScore { tier: u8, priority_rank: i32, causal_sequence: u64, operation_key: String }

pub fn selector_candidates(snapshot: &RuntimeSnapshot) -> Vec<SelectorCandidate> {
    selector_candidates_for(snapshot, None)
}
pub fn selector_candidates_at(snapshot: &RuntimeSnapshot, now: &str) -> Vec<SelectorCandidate> {
    selector_candidates_for(snapshot, Some(now))
}
#[rustfmt::skip]
fn selector_candidates_for(snapshot: &RuntimeSnapshot, now: Option<&str>) -> Vec<SelectorCandidate> {
    let mut items = snapshot.active_cells().into_iter()
        .filter(|cell| crate::runtime_eligibility::cell_is_due(cell, now)).filter_map(cell_candidate)
        .map(|item| selector::apply_edge_blocks(snapshot, item)).collect::<Vec<_>>();
    if items.is_empty() {
        let fallback = if crate::runtime_eligibility::has_invalid_cooldown(snapshot) { blocked_candidate(&[]) }
            else { now.and_then(|value| crate::runtime_eligibility::next_wake(snapshot, value)).map_or_else(idle_candidate, wait_candidate) };
        items.push(fallback);
    }
    items.sort_by(|left, right| left.score.cmp(&right.score)); items
}
pub fn selected_candidate(snapshot: &RuntimeSnapshot) -> SelectorCandidate {
    select_candidate(selector_candidates(snapshot))
}
pub fn selected_candidate_at(snapshot: &RuntimeSnapshot, now: &str) -> SelectorCandidate {
    select_candidate(selector_candidates_at(snapshot, now))
}
fn select_candidate(items: Vec<SelectorCandidate>) -> SelectorCandidate {
    items
        .iter()
        .find(|item| item.blocked_by.is_empty())
        .cloned()
        .unwrap_or_else(|| blocked_candidate(&items))
}
#[rustfmt::skip]
fn cell_candidate(cell: &StateCell) -> Option<SelectorCandidate> {
    let body = payload_value(cell);
    if known_payload(cell) { if let Some(operation) = operation_from_payload(&body) {
        return Some(candidate(cell, operation, payload_tier(&body, 80), "payload"));
    } } else if payload_text(&body, "operation_key").is_some() { return None; }
    match (cell.key.namespace.as_str(), cell.key.name.as_str()) {
        ("case", "owner-intake") => Some(model_free(cell, "owner.intake", 10, "owner intake")),
        ("case", "waiting-answer") => Some(model_free(cell, "owner.answer", 20, "owner answer")),
        ("completion", "blocked") => Some(model_free(cell, "completion.blocked", 65, "completion blocked")),
        ("completion", "close-candidate") => Some(model_free(cell, "completion.close", 70, "completion candidate")),
        _ => namespace_candidate(cell, &body),
    }
}
fn known_payload(cell: &StateCell) -> bool {
    matches!(
        cell.payload_schema.as_str(),
        "state.operation.v1" | "state.completion-check"
    ) || matches!(
        cell.key.namespace.as_str(),
        "model" | "effect" | "check" | "recovery" | "work"
    )
}
#[rustfmt::skip]
fn namespace_candidate(cell: &StateCell, body: &Value) -> Option<SelectorCandidate> {
    match cell.key.namespace.as_str() {
        "recovery" => Some(model_free(cell, &format!("recovery.handle/{}", cell.key.name), 30, "recovery")),
        "effect" => Some(model_free(cell, &format!("effect.run/{}", cell.key.name), 40, "effect")),
        "model" => Some(candidate(cell, RuntimeOperation::model_call(format!("model.call/{}", cell.key.name), payload_envelope(body),
            payload_tool_view(body), payload_number(body, "model_budget_tokens"), evidence(cell)), 50, "model")),
        "check" => Some(model_free(cell, &format!("check.run/{}", cell.key.name), 60, "check")),
        namespace => crate::runtime_operation::legacy_workspace_family(namespace)
            .map(|(operation, tier)| model_free(cell, &format!("{operation}/{}", cell.key.name), tier, namespace)),
    }
}
#[rustfmt::skip]
fn operation_from_payload(body: &Value) -> Option<RuntimeOperation> {
    let key = payload_text(body, "operation_key")?; let expected = payload_envelope(body);
    let mut operation = if expected == OutputEnvelope::None {
        RuntimeOperation::model_free_effect(key, payload_evidence_requirements(body), payload_effect_command(body))
    } else { RuntimeOperation::model_call(key, expected, payload_tool_view(body),
        payload_number(body, "model_budget_tokens"), payload_evidence_requirements(body)) };
    if let Some(policy) = payload_text(body, "recovery_policy") { operation.recovery_policy = policy.into(); } Some(operation)
}
fn model_free(cell: &StateCell, key: &str, tier: u8, reason: &str) -> SelectorCandidate {
    candidate(
        cell,
        RuntimeOperation::model_free(key, evidence(cell)),
        tier,
        reason,
    )
}
#[rustfmt::skip]
fn candidate(cell: &StateCell, operation: RuntimeOperation, tier: u8, reason: &str) -> SelectorCandidate {
    let operation_key = operation.key.clone();
    SelectorCandidate { operation, state_key: Some(cell.key.clone()), reason: reason.into(), blocked_by: Vec::new(),
        score: CandidateScore { tier, priority_rank: -cell.priority,
            causal_sequence: causal_number(&cell.source_event_id).unwrap_or(u64::MAX), operation_key } }
}
fn idle_candidate() -> SelectorCandidate {
    synthetic(
        RuntimeOperation::idle(),
        "no executable state candidate",
        "runtime.idle",
        100,
    )
}
fn wait_candidate(due: &str) -> SelectorCandidate {
    synthetic(
        RuntimeOperation::model_free("runtime.wait", vec![format!("wake:{due}")]),
        &format!("waiting until {due}"),
        "runtime.wait",
        99,
    )
}
fn blocked_candidate(items: &[SelectorCandidate]) -> SelectorCandidate {
    let evidence = items
        .iter()
        .flat_map(|item| item.blocked_by.clone())
        .collect();
    synthetic(
        RuntimeOperation::model_free("completion.blocked", evidence),
        "all state candidates are blocked",
        "completion.blocked",
        65,
    )
}
fn synthetic(operation: RuntimeOperation, reason: &str, key: &str, tier: u8) -> SelectorCandidate {
    SelectorCandidate {
        operation,
        state_key: None,
        reason: reason.into(),
        blocked_by: Vec::new(),
        score: CandidateScore {
            tier,
            priority_rank: 0,
            causal_sequence: u64::MAX,
            operation_key: key.into(),
        },
    }
}
#[rustfmt::skip]
fn evidence(cell: &StateCell) -> Vec<String> {
    if cell.evidence_refs.is_empty() { vec![cell.key.as_label()] } else {
        cell.evidence_refs.iter().map(|item| format!("{}:{}", item.source_type, item.source_id)).collect()
    }
}
fn payload_value(cell: &StateCell) -> Value {
    serde_json::from_str(&cell.payload_json).unwrap_or(Value::Null)
}
fn payload_text<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}
#[rustfmt::skip]
fn payload_number(value: &Value, key: &str) -> Option<u32> {
    value.get(key).and_then(Value::as_u64).and_then(|number| u32::try_from(number).ok())
}
#[rustfmt::skip]
fn payload_tier(value: &Value, default: u8) -> u8 {
    payload_number(value, "selector_tier").and_then(|number| u8::try_from(number).ok()).unwrap_or(default)
}
#[rustfmt::skip]
fn payload_envelope(value: &Value) -> OutputEnvelope { match payload_text(value, "expected_envelope") {
    Some("Content") => OutputEnvelope::Content, Some("Plan") => OutputEnvelope::Plan, Some("Action") => OutputEnvelope::Action,
    Some("Message") => OutputEnvelope::Message, Some("Verdict") => OutputEnvelope::Verdict, _ => OutputEnvelope::None,
} }
#[rustfmt::skip]
fn payload_tool_view(value: &Value) -> ToolSetView {
    if payload_number(value, "tool_budget_remaining") == Some(0) { return ToolSetView::empty(); }
    ToolSetView::new(value.get("tool_view").and_then(Value::as_array)
        .map(|items| items.iter().filter_map(tool_entry).collect()).unwrap_or_default())
}
#[rustfmt::skip]
fn tool_entry(value: &Value) -> Option<ToolViewEntry> {
    let name = payload_text(value, "name")?;
    explore_catalog().iter().find(|descriptor| descriptor.name == name).map(descriptor_entry)
}
#[rustfmt::skip]
fn payload_evidence_requirements(value: &Value) -> Vec<String> {
    let mut values: Vec<String> = value.get("evidence_requirements").and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    values.sort(); values
}
#[rustfmt::skip]
fn payload_effect_command(value: &Value) -> Option<EffectCommand> {
    let value = value.get("effect_command")?;
    Some(EffectCommand { name: payload_text(value, "name")?.into(), path: payload_text(value, "path").map(str::to_string),
        content: payload_text(value, "content").map(str::to_string) })
}
