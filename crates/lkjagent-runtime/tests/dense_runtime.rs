#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::{RuntimeDecisionKind, RuntimeEffectCommand, RuntimeEvent};
use obligation_network_support::*;

#[test]
fn deterministic_audit_is_runtime_effect_without_provider_surface() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    let decision = decision(input, RuntimeEvent::OwnerMessageReceived)?;

    assert_eq!(next_tool(&decision)?, "doc.audit");
    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert!(matches!(
        decision.runtime_effect,
        Some(RuntimeEffectCommand::DeterministicInspection { .. })
    ));
    assert!(decision
        .resolver_plan
        .as_deref()
        .unwrap_or("")
        .contains("audit"));
    assert!(decision.progress_key.is_some());
    Ok(())
}

#[test]
fn completion_refusal_uses_typed_gate_inputs() -> Result<(), String> {
    let mut input = owner_input();
    input.required_evidence = vec!["artifact-readiness".to_string()];
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    let decision = decision(input, RuntimeEvent::CompletionRequested)?;

    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert_eq!(decision.completion_blockers, vec!["artifact-readiness"]);
    assert!(decision
        .completion_gate_inputs
        .iter()
        .any(|input| input == "artifact_required=true"));
    Ok(())
}
