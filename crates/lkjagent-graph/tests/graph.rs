use lkjagent_graph::transition_history::{TransitionOutcome, TransitionRecord};
use lkjagent_graph::{
    admit_transition, completion_decision, initial_state, render_graph_slice, source_graph,
    transition_quality, validate_graph, EvidenceKind, EvidenceRecord, GraphNodeId, TaskFamily,
    TransitionDecision, TransitionLegality,
};

#[test]
fn source_graph_validates_typed_guards() {
    let report = validate_graph(source_graph());

    assert!(report.is_ok(), "{:?}", report.violations);
    assert!(source_graph()
        .edges
        .iter()
        .all(|edge| !edge.guards.is_empty()));
    assert!(source_graph()
        .nodes
        .iter()
        .any(|node| node.id == GraphNodeId("recover-by-shell-escape")));
    assert!(source_graph()
        .nodes
        .iter()
        .any(|node| node.id == GraphNodeId("document-topology")));
}

#[test]
fn initial_case_does_not_satisfy_plan_evidence() {
    let state = initial_state("redesign architecture docs and runtime", Some(7));

    assert_eq!(state.family, TaskFamily::Architecture);
    assert_eq!(state.active_node, GraphNodeId("plan"));
    assert!(state.evidence.knows_requirement("plan"));
    assert!(!state.evidence.has("plan"));
    assert!(!state.plan.ready);
    assert!(state
        .selected_packages()
        .contains(&"planning-checklist".to_string()));
}

#[test]
fn counted_content_deliverable_selects_document_construction() {
    let state = initial_state(
        "Create about 100 files total for a structured story corpus with chapters.",
        Some(8),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
    assert!(state.evidence.knows_requirement("document-structure"));
}

#[test]
fn counted_implementation_request_stays_code_change() {
    let state = initial_state("Create exactly 3 files implementing a Rust CLI.", Some(9));

    assert_eq!(state.family, TaskFamily::CodeChange);
    assert!(!state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn transition_guards_block_illegal_execution() {
    let mut state = initial_state("fix parser bug", Some(1));
    state.active_node = GraphNodeId("review-plan");

    assert!(matches!(
        admit_transition(source_graph(), &state, GraphNodeId("execute")),
        TransitionDecision::Defer { missing } if missing.contains(&"plan".to_string())
    ));
}

#[test]
fn graph_plan_satisfies_planning_gate_but_not_completion() {
    let mut state = initial_state("fix parser bug", Some(2));
    state.active_node = GraphNodeId("plan");
    state.plan.ready = true;
    state
        .evidence
        .records
        .push(record("plan", EvidenceKind::Plan));

    assert!(matches!(
        admit_transition(source_graph(), &state, GraphNodeId("review-plan")),
        TransitionDecision::Admit { .. }
    ));
    assert!(matches!(
        completion_decision(&state),
        TransitionDecision::Defer { missing } if missing.contains(&"verification".to_string())
    ));
}

#[test]
fn completion_requires_typed_evidence_and_checks() {
    let mut state = initial_state("fix parser bug", Some(3));
    state
        .evidence
        .records
        .push(record("plan", EvidenceKind::Plan));
    state
        .evidence
        .records
        .push(record("observation", EvidenceKind::Observation));

    assert!(matches!(
        completion_decision(&state),
        TransitionDecision::Defer { .. }
    ));

    state
        .evidence
        .records
        .push(record("verification", EvidenceKind::Verification));
    state.evidence.pending_checks.clear();

    assert_eq!(
        completion_decision(&state),
        TransitionDecision::Admit {
            target: GraphNodeId("complete")
        }
    );
}

#[test]
fn render_graph_slice_names_allowed_and_blocked_tools() {
    let state = initial_state("write docs", None);
    let rendered = render_graph_slice(source_graph(), &state, 512);

    assert!(rendered.contains("phase: planning"));
    assert!(rendered.contains("Missing evidence: plan"));
    assert!(rendered.contains("Allowed tools now:"));
    assert!(rendered.contains("Blocked tools now:"));
    assert!(rendered.contains("graph.plan"));
    assert!(rendered.contains("fs.write"));
    assert!(rendered.contains("Legal transitions:"));
}

#[test]
fn transition_quality_scores_legal_and_blocked_candidates() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(4));

    let blocked = transition_quality(&graph, &state, GraphNodeId("review-plan"));
    assert_eq!(blocked.legality, TransitionLegality::Blocked);
    assert!(blocked.reason.contains("plan"));

    state.plan.ready = true;
    let legal = transition_quality(&graph, &state, GraphNodeId("review-plan"));
    assert_eq!(legal.legality, TransitionLegality::Legal);
    assert!(legal.evidence_delta > 0);
    assert!(legal.expected_next_observation.is_some());
}

#[test]
fn transition_quality_penalizes_repeated_targets() {
    let graph = source_graph();
    let mut state = initial_state("fix parser bug", Some(5));
    state.active_node = GraphNodeId("recover-tool");
    state.transitions.push(TransitionRecord {
        from: GraphNodeId("execute"),
        to: GraphNodeId("plan"),
        decision: TransitionOutcome::Recovered,
        reason: "retry".to_string(),
    });

    let quality = transition_quality(&graph, &state, GraphNodeId("plan"));

    assert_eq!(quality.legality, TransitionLegality::Legal);
    assert!(quality.repetition_penalty > 0);
}

#[test]
fn transition_quality_rejects_non_edges() {
    let graph = source_graph();
    let state = initial_state("fix parser bug", Some(6));

    let quality = transition_quality(&graph, &state, GraphNodeId("complete"));

    assert_eq!(quality.legality, TransitionLegality::Illegal);
    assert!(quality.risk_delta > 0);
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
