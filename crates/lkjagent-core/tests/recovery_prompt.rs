use std::collections::BTreeSet;

use lkjagent_core::classify::instantiate;
use lkjagent_core::render::render_prompt_for_decision;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};

#[test]
fn strategy_changes_alter_prompt_and_smaller_prompt_bounds_context() {
    let mut snapshot = instantiate(3, "Recover without repeating the same causal conditions.");
    snapshot.task.brief = "durable evidence and operation detail ".repeat(2_000);
    snapshot.steps[0].instruction = "bounded operation evidence ".repeat(2_000);
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    let base = render_prompt_for_decision(
        &snapshot.task,
        &snapshot.steps,
        &snapshot.steps[0],
        &decision,
    );
    let policies = [
        "grammar-repair",
        "concrete-example",
        "constrained-grammar",
        "narrow-output",
        "reduce-unit",
        "continue-boundary",
        "split-section",
        "replan-artifact",
        "remove-hidden-tool",
        "correct-primitive",
        "select-target",
        "reinspect",
        "retry-backoff",
        "alternate-sampling",
        "smaller-prompt",
        "reconnect",
        "inspect-filesystem",
        "idempotent-replay",
        "compensate",
        "quarantine",
        "inspect-check",
        "repair-source",
        "rerun-check",
        "replan",
        "inspect-state",
        "split-work",
        "clarify",
    ];
    let mut fingerprints = BTreeSet::new();
    for policy in policies {
        decision.recovery_policy = policy.to_string();
        let prompt = render_prompt_for_decision(
            &snapshot.task,
            &snapshot.steps,
            &snapshot.steps[0],
            &decision,
        );
        assert!(prompt.user.contains("Strategy change:"), "policy={policy}");
        assert_ne!(prompt.fingerprint, base.fingerprint, "policy={policy}");
        fingerprints.insert(prompt.fingerprint);
        if policy == "smaller-prompt" {
            assert!(prompt.user.len() < base.user.len());
        }
    }
    assert_eq!(fingerprints.len(), policies.len());
}
