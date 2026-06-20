use lkjagent_graph::case_recovery::FaultKind;
use lkjagent_graph::policy::ContextPressureLevel;
use lkjagent_graph::transition_history::{TransitionOutcome, TransitionRecord};
use lkjagent_graph::{
    best_next_transition, initial_state, source_graph, EvidenceKind, EvidenceRecord, GraphNodeId,
    TransitionIntent, TransitionLegality,
};

#[test]
fn best_transition_prefers_plan_review_after_plan_evidence() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(1));
    state.plan.ready = true;

    let selected = best_next_transition(&graph, &state, TransitionIntent::AfterPlan);

    assert_eq!(selected.target, Some(GraphNodeId("review-plan")));
    assert_eq!(selected.legality, TransitionLegality::Legal);
}

#[test]
fn best_transition_prefers_execute_after_plan_and_context() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(2));
    state.active_node = GraphNodeId("review-plan");
    state.plan.ready = true;

    let selected = best_next_transition(&graph, &state, TransitionIntent::AfterPlan);

    assert_eq!(selected.target, Some(GraphNodeId("execute")));
    assert_eq!(selected.legality, TransitionLegality::Legal);
}

#[test]
fn best_transition_prefers_verify_after_observation() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(3));
    state.active_node = GraphNodeId("advance-plan");
    state
        .evidence
        .records
        .push(record("observation", EvidenceKind::Observation));

    let selected = best_next_transition(&graph, &state, TransitionIntent::AfterObservation);

    assert_eq!(selected.target, Some(GraphNodeId("verify")));
    assert_eq!(selected.legality, TransitionLegality::Legal);
}

#[test]
fn best_transition_rejects_completion_without_verification() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(4));
    state.active_node = GraphNodeId("verify");

    let selected = best_next_transition(&graph, &state, TransitionIntent::AttemptCompletion);

    assert_eq!(selected.target, Some(GraphNodeId("complete")));
    assert_eq!(selected.legality, TransitionLegality::Blocked);
    assert!(selected.missing.contains(&"completion-ready".to_string()));
}

#[test]
fn best_transition_penalizes_repeated_target() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(5));
    state.active_node = GraphNodeId("recover-tool");
    state.transitions.push(TransitionRecord {
        from: GraphNodeId("execute"),
        to: GraphNodeId("plan"),
        decision: TransitionOutcome::Recovered,
        reason: "retry".to_string(),
    });

    let selected = best_next_transition(&graph, &state, TransitionIntent::Continue);

    assert_eq!(
        selected.target,
        Some(GraphNodeId("recover-by-state-inspection"))
    );
    assert_eq!(selected.legality, TransitionLegality::Legal);
}

#[test]
fn best_transition_for_faults_choose_specific_recovery_nodes() {
    let graph = source_graph();

    assert_fault_target(
        &graph,
        FaultKind::Params,
        TransitionIntent::AfterParamFault,
        "recover-params",
    );
    assert_fault_target(
        &graph,
        FaultKind::Tool,
        TransitionIntent::AfterToolFault,
        "recover-tool",
    );
    assert_fault_target(
        &graph,
        FaultKind::Repeat,
        TransitionIntent::AfterRepeatFault,
        "recover-repeat",
    );
}

#[test]
fn best_transition_under_pressure_rejects_blocked_compaction() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(6));
    state.active_node = GraphNodeId("execute");
    state.context.pressure = ContextPressureLevel::Green;

    let selected = best_next_transition(&graph, &state, TransitionIntent::UnderContextPressure);

    assert_ne!(selected.target, Some(GraphNodeId("compact-soft")));
}

fn assert_fault_target(
    graph: &lkjagent_graph::GraphDefinition,
    fault: FaultKind,
    intent: TransitionIntent,
    target: &'static str,
) {
    let mut state = initial_state("fix parser bug", Some(7));
    state.active_node = GraphNodeId("execute");
    match fault {
        FaultKind::Params => state.recovery.param_failures = 1,
        FaultKind::Tool => state.recovery.tool_failures = 1,
        FaultKind::Repeat => state.recovery.repeat_failures = 1,
        _ => {}
    }

    let selected = best_next_transition(graph, &state, intent);

    assert_eq!(selected.target, Some(GraphNodeId(target)));
    assert_eq!(selected.legality, TransitionLegality::Legal);
}

fn record(requirement: &str, kind: EvidenceKind) -> EvidenceRecord {
    EvidenceRecord {
        requirement: requirement.to_string(),
        kind,
        summary: format!("{requirement} observed"),
        path: None,
        frame_ref: None,
        event_ref: None,
        confidence: 80,
        satisfies_completion: true,
    }
}
