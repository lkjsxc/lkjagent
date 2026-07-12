use crate::runtime_candidate::{
    selected_candidate, selected_candidate_at, selector_candidates, SelectorCandidate,
};
use crate::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use crate::runtime_fingerprint::FingerprintError;
use crate::runtime_operation::{
    BlockReason, RuntimeBlock, RuntimeDecisionSpec, RuntimeOperation, RuntimePolicy, RuntimeWait,
    Selection, WakeCondition,
};
pub use crate::runtime_state::{can_close, CheckEvidence, CompletionRequirement};
use crate::runtime_state::{CurrentTime, RuntimePhase, RuntimeSnapshot, RuntimeState, StateCell};
use serde_json::Value;
use std::collections::BTreeMap;

pub const FILE_CHECK_KINDS: &[&str] = &[
    "regular-utf8",
    "intended-sha256",
    "occurrence-counts",
    "admitted-diff",
    "preserved-mode",
    "allowed-changed-paths",
    "effects-settled",
];
pub const EXIT_GUARDS: &[&str] = &[
    "required-current-checks-passed",
    "no-blocking-operation",
    "effects-settled",
    "final-message-persisted",
];

#[rustfmt::skip]
pub fn select(state: RuntimeState, policy: RuntimePolicy, now: CurrentTime) -> Selection {
    if crate::runtime_eligibility::utc_millis(now.as_str()).is_none() { return blocked(BlockReason::InvalidCooldown(now.0)); }
    if let Some(items) = active_conflicts(&state) { return blocked(BlockReason::Conflict(items)); }
    if let Some(key) = unknown_executable(&state) { return blocked(BlockReason::UnknownExecutable(key)); }
    if has(&state, "need", "owner-fact") { return Selection::Wait(RuntimeWait { reason: "missing owner fact".into(),
        wake: WakeCondition::OwnerInput { matter_id: state.snapshot.case_id.clone() } }); }
    if let Some(wake) = next_wait(&state, &now) { return wake; }
    if crate::runtime_eligibility::has_invalid_cooldown(&state.snapshot) {
        return blocked(BlockReason::InvalidCooldown("state cell cooldown".into()));
    }
    if equal_progress(&policy) { return match crate::runtime_eligibility::no_progress_strategy(policy.recovery_attempt) {
        Some(strategy) => decision(&state, RuntimePhase::Modify, &format!("recovery.{strategy}"), true,
            OutputEnvelope::Action, policy.model_budget_tokens, strategy), None => blocked(BlockReason::Stasis), }; }
    if has(&state, "response", "final-persisted") { return closure(&state); }
    if has(&state, "check", "current-passed") { return decision(&state, RuntimePhase::Respond, "respond.final", true,
        OutputEnvelope::Message, policy.model_budget_tokens, "do-not-retry-final"); }
    if has(&state, "check", "failed") || any(&state, "fault") { return decision(&state, RuntimePhase::Modify,
        "recovery.modify", true, OutputEnvelope::Action, policy.model_budget_tokens, "change-operation"); }
    if has(&state, "edit", "committed") { return decision(&state, RuntimePhase::Review, "check.run/current", false,
        OutputEnvelope::None, policy.model_budget_tokens, "commit-or-recover"); }
    if has(&state, "source", "current") { return decision(&state, RuntimePhase::Modify, "modify.source", true,
        OutputEnvelope::Action, policy.model_budget_tokens, "change-operation"); }
    if has(&state, "matter", "opened") { return decision(&state, RuntimePhase::Orient, "orient.matter", true,
        OutputEnvelope::Plan, policy.model_budget_tokens, "change-operation"); }
    let candidate = selected_candidate_at(&state.snapshot, now.as_str()); match candidate.operation.key.as_str() {
        "runtime.idle" => Selection::Idle,
        "completion.blocked" => blocked(BlockReason::MissingEvidence(candidate.operation.evidence_requirements)),
        _ => operation_decision(&state, candidate.operation),
    }
}
#[rustfmt::skip]
fn next_wait(state: &RuntimeState, now: &CurrentTime) -> Option<Selection> {
    let wake = crate::runtime_eligibility::next_wake(&state.snapshot, now.as_str())?;
    let runnable = state.snapshot.active_cells().iter().any(|cell|
        crate::runtime_eligibility::cell_is_due(cell, Some(now.as_str())) && executable(cell));
    (!runnable).then(|| Selection::Wait(RuntimeWait { reason: "operation cooling down".into(), wake: WakeCondition::Time { at: wake.into() } }))
}
#[rustfmt::skip]
fn closure(state: &RuntimeState) -> Selection {
    let effects = active_names(state, "effect"); if !effects.is_empty() { return blocked(BlockReason::UnsettledEffects(effects)); }
    let operations = active_names(state, "operation");
    let checks = has(state, "check", "current-passed") && !has(state, "check", "stale");
    if checks && operations.is_empty() { return Selection::Idle; }
    let mut missing = Vec::new(); if !checks { missing.push("required-current-checks-passed".into()); }
    if !operations.is_empty() { missing.push("no-blocking-operation".into()); } blocked(BlockReason::MissingEvidence(missing))
}
#[rustfmt::skip]
fn decision(state: &RuntimeState, phase: RuntimePhase, key: &str, model: bool,
    envelope: OutputEnvelope, budget: u32, recovery: &str) -> Selection {
    Selection::Decision(RuntimeDecisionSpec { phase, operation_key: key.into(), causal_sequence: state.causal_sequence,
        model_required: model, expected_envelope: envelope, tool_view: ToolSetView::empty(),
        model_budget_tokens: model.then_some(budget), recovery_policy: recovery.into() })
}
#[rustfmt::skip]
fn operation_decision(state: &RuntimeState, operation: RuntimeOperation) -> Selection {
    Selection::Decision(RuntimeDecisionSpec { phase: state.phase, operation_key: operation.key,
        causal_sequence: state.causal_sequence, model_required: operation.expected_envelope != OutputEnvelope::None,
        expected_envelope: operation.expected_envelope, tool_view: operation.tool_view,
        model_budget_tokens: operation.model_budget_tokens, recovery_policy: operation.recovery_policy })
}
fn blocked(reason: BlockReason) -> Selection {
    Selection::Block(RuntimeBlock { reason })
}
fn has(state: &RuntimeState, ns: &str, name: &str) -> bool {
    state
        .snapshot
        .active_cells()
        .iter()
        .any(|cell| cell.key.namespace == ns && cell.key.name == name)
}
fn any(state: &RuntimeState, ns: &str) -> bool {
    state
        .snapshot
        .active_cells()
        .iter()
        .any(|cell| cell.key.namespace == ns)
}
fn active_names(state: &RuntimeState, ns: &str) -> Vec<String> {
    state
        .snapshot
        .active_cells()
        .iter()
        .filter(|cell| cell.key.namespace == ns)
        .map(|cell| cell.key.as_label())
        .collect()
}
fn executable(cell: &StateCell) -> bool {
    matches!(
        cell.key.namespace.as_str(),
        "matter" | "source" | "edit" | "check" | "fault" | "model" | "effect" | "recovery"
    )
}
fn equal_progress(policy: &RuntimePolicy) -> bool {
    policy.prior_progress_fingerprint.is_some()
        && policy.prior_progress_fingerprint == policy.current_progress_fingerprint
}
#[rustfmt::skip]
fn active_conflicts(state: &RuntimeState) -> Option<Vec<String>> {
    let mut groups: BTreeMap<&str, Vec<String>> = BTreeMap::new(); for cell in state.snapshot.active_cells() {
        if let Some(group) = cell.conflict_group.as_deref() { groups.entry(group).or_default().push(cell.key.as_label()); }
    } groups.into_values().find(|items| items.len() > 1)
}
#[rustfmt::skip]
fn unknown_executable(state: &RuntimeState) -> Option<String> {
    state.snapshot.active_cells().into_iter().find(|cell| {
        let operation = serde_json::from_str::<Value>(&cell.payload_json).ok()
            .and_then(|value| value.get("operation_key").cloned()).is_some();
        operation && cell.payload_schema != "state.operation.v1"
            && !matches!(cell.key.namespace.as_str(), "model" | "effect" | "check" | "recovery")
    }).map(|cell| cell.key.as_label())
}

#[rustfmt::skip]
pub fn select_runtime_decision(snapshot: &RuntimeSnapshot, id: &str, context: &str,
    unfinished: &[RuntimeDecision]) -> Result<RuntimeDecision, FingerprintError> {
    if let Some(decision) = unfinished.first() { return Ok(decision.clone()); }
    decision_from(snapshot, id, context, selected_candidate(snapshot))
}
#[rustfmt::skip]
pub fn select_runtime_decision_at(snapshot: &RuntimeSnapshot, id: &str, context: &str,
    unfinished: &[RuntimeDecision], now: &str) -> Result<RuntimeDecision, FingerprintError> {
    if let Some(decision) = unfinished.first() { return Ok(decision.clone()); }
    decision_from(snapshot, id, context, selected_candidate_at(snapshot, now))
}
#[rustfmt::skip]
fn decision_from(snapshot: &RuntimeSnapshot, id: &str, context: &str, candidate: SelectorCandidate) -> Result<RuntimeDecision, FingerprintError> {
    let operation = candidate.operation; let mut output = RuntimeDecision::new(id, snapshot.case_id.clone(),
        OperationKey(operation.key), operation.tool_view, operation.expected_envelope);
    output.selected_state_key = candidate.state_key.as_ref().map(|key| key.as_label()); output.effect_command = operation.effect_command;
    let fingerprint = snapshot.fingerprint()?; output.snapshot_fingerprint = fingerprint.clone(); output.state_vector_fingerprint = fingerprint;
    output.context_frame_fingerprint = context.into(); output.model_budget_tokens = operation.model_budget_tokens;
    output.evidence_requirements = operation.evidence_requirements; output.evidence_requirements.insert(0, format!("selector:{}", candidate.reason));
    if candidate.state_key.as_ref().and_then(|key| snapshot.cells.get(key)).is_some_and(tool_exhausted) {
        output.evidence_requirements.push("tool-budget:suppressed".into());
    }
    output.recovery_policy = operation.recovery_policy; output.refresh_harness_state(); Ok(output)
}
fn tool_exhausted(cell: &StateCell) -> bool {
    serde_json::from_str::<Value>(&cell.payload_json)
        .ok()
        .and_then(|value| value.get("tool_budget_remaining").and_then(Value::as_u64))
        == Some(0)
}
pub fn select_operation(snapshot: &RuntimeSnapshot) -> RuntimeOperation {
    selected_candidate(snapshot).operation
}
pub fn candidates(snapshot: &RuntimeSnapshot) -> Vec<SelectorCandidate> {
    selector_candidates(snapshot)
}
#[rustfmt::skip]
pub(crate) fn apply_edge_blocks(snapshot: &RuntimeSnapshot, mut item: SelectorCandidate) -> SelectorCandidate {
    let Some(key) = &item.state_key else { return item }; let label = key.as_label();
    item.blocked_by = snapshot.active_edges().into_iter().filter(|edge| edge.relation.0 == "blocks"
        && edge.to_ref.kind == "state" && edge.to_ref.id == label).map(|edge| edge.id).collect(); item
}
