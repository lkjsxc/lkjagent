use crate::model::GraphNodeId;
use crate::state::{TaskGraphState, TransitionDecision};

pub fn missing_requirements(state: &TaskGraphState) -> Vec<String> {
    state
        .evidence_requirements
        .iter()
        .filter(|requirement| {
            !state
                .evidence
                .iter()
                .any(|record| record.requirement == **requirement)
        })
        .cloned()
        .collect()
}

pub fn completion_decision(state: &TaskGraphState) -> TransitionDecision {
    let missing = missing_requirements(state);
    if !missing.is_empty() {
        return TransitionDecision::Defer { missing };
    }
    if !state.pending_checks.is_empty() {
        return TransitionDecision::Defer {
            missing: state.pending_checks.clone(),
        };
    }
    TransitionDecision::Admit {
        target: GraphNodeId("complete"),
    }
}
