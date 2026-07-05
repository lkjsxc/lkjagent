use crate::runtime_candidate::{selected_candidate, selector_candidates, SelectorCandidate};
use crate::runtime_decision::{OperationKey, RuntimeDecision};
use crate::runtime_fingerprint::FingerprintError;
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_state::RuntimeSnapshot;

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
    let snapshot_fingerprint = snapshot.fingerprint()?;
    decision.snapshot_fingerprint = snapshot_fingerprint.clone();
    decision.state_vector_fingerprint = snapshot_fingerprint;
    decision.context_frame_fingerprint = context_frame_fingerprint.to_string();
    decision.model_budget_tokens = operation.model_budget_tokens;
    decision.evidence_requirements = operation.evidence_requirements;
    decision
        .evidence_requirements
        .insert(0, format!("selector:{}", candidate.reason));
    decision.recovery_policy = operation.recovery_policy;
    Ok(decision)
}

pub fn select_operation(snapshot: &RuntimeSnapshot) -> RuntimeOperation {
    selected_candidate(snapshot).operation
}

pub fn candidates(snapshot: &RuntimeSnapshot) -> Vec<SelectorCandidate> {
    selector_candidates(snapshot)
}
