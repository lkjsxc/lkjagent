use crate::model::{GraphNodeId, TaskFamily};
use crate::state::{TaskGraphState, TransitionDecision};

pub fn missing_requirements(state: &TaskGraphState) -> Vec<String> {
    let mut missing = state
        .evidence
        .requirements
        .iter()
        .filter(|requirement| {
            requirement.required_for_completion && !state.evidence.has(&requirement.id)
        })
        .map(|requirement| requirement.id.clone())
        .collect::<Vec<_>>();
    if needs_real_verification(state) && !state.evidence.has("verification") {
        push_unique(&mut missing, "verification");
    }
    if needs_document_audit(state) && !state.evidence.has("document-structure") {
        push_unique(&mut missing, "document-structure");
    }
    missing
}

pub fn refresh_completion_state(state: &mut TaskGraphState) {
    let missing = missing_requirements(state);
    state.completion.missing_requirements = missing.clone();
    state.completion.pending_checks = state.evidence.pending_checks.clone();
    state.completion.ready = missing.is_empty() && state.evidence.pending_checks.is_empty();
    state.completion.refusal_reason = if state.completion.ready {
        None
    } else {
        Some(refusal_reason(&missing, &state.evidence.pending_checks))
    };
}

pub fn completion_decision(state: &TaskGraphState) -> TransitionDecision {
    let missing = missing_requirements(state);
    if !missing.is_empty() {
        return TransitionDecision::Defer { missing };
    }
    if !state.evidence.pending_checks.is_empty() {
        return TransitionDecision::Defer {
            missing: state.evidence.pending_checks.clone(),
        };
    }
    TransitionDecision::Admit {
        target: GraphNodeId("complete"),
    }
}

fn needs_real_verification(state: &TaskGraphState) -> bool {
    matches!(
        state.family,
        TaskFamily::CodeChange
            | TaskFamily::BugFix
            | TaskFamily::Architecture
            | TaskFamily::Benchmark
            | TaskFamily::Verification
    )
}

fn needs_document_audit(state: &TaskGraphState) -> bool {
    matches!(
        state.family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    )
}

fn refusal_reason(missing: &[String], checks: &[String]) -> String {
    if !missing.is_empty() {
        return format!("missing evidence: {}", missing.join(", "));
    }
    format!("pending checks: {}", checks.join(", "))
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|item| item == value) {
        values.push(value.to_string());
    }
}
