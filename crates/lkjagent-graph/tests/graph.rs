use lkjagent_graph::{
    admit_transition, compaction_plan, completion_decision, initial_state, render_graph_slice,
    source_graph, validate_graph, EvidenceKind, EvidenceRecord, GraphNodeId, TaskFamily,
    TransitionDecision,
};

#[test]
fn source_graph_validates_typed_guards() {
    let report = validate_graph(source_graph());

    assert!(report.is_ok(), "{:?}", report.violations);
    assert!(source_graph()
        .edges
        .iter()
        .all(|edge| !edge.guards.is_empty()));
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
fn context_package_selection_is_family_and_node_aware() {
    let docs = initial_state("Create about 100 docs for a handbook.", None);
    let code = initial_state("fix parser bug", None);

    assert!(docs
        .selected_packages()
        .contains(&"doc-construction".to_string()));
    assert!(!code
        .selected_packages()
        .contains(&"doc-construction".to_string()));
    assert!(source_graph()
        .packages
        .iter()
        .any(|package| package.id.0 == "recovery-policy"));
}

#[test]
fn render_graph_slice_names_allowed_and_blocked_tools() {
    let state = initial_state("write docs", None);
    let rendered = render_graph_slice(source_graph(), &state, 512);

    assert!(rendered.contains("phase=planning"));
    assert!(rendered.contains("missing_evidence=plan"));
    assert!(rendered.contains("allowed_tools="));
    assert!(rendered.contains("blocked_tools="));
    assert!(rendered.contains("graph.plan"));
    assert!(rendered.contains("fs.write"));
}

#[test]
fn compaction_preserves_rich_case_state() {
    let mut state = initial_state("fix parser bug", Some(4));
    state.active_node = GraphNodeId("execute");
    state.plan.ready = true;
    state.plan.steps.push(lkjagent_graph::case_plan::PlanStep {
        id: lkjagent_graph::case_plan::StepId("step-1".to_string()),
        title: "patch parser".to_string(),
        rationale: "required".to_string(),
        status: lkjagent_graph::case_plan::StepStatus::Active,
        node: GraphNodeId("execute"),
        target_paths: vec!["crates/lkjagent-protocol/src/parse.rs".to_string()],
        required_evidence: vec!["verification".to_string()],
        verification: Vec::new(),
    });
    state
        .workspace
        .touched_paths
        .push("crates/lkjagent-protocol/src/parse.rs".to_string());
    state
        .evidence
        .records
        .push(record("plan", EvidenceKind::Plan));

    let plan = compaction_plan(&state);
    assert_eq!(plan.case_id, Some(4));
    assert_eq!(plan.active_node, GraphNodeId("execute"));
    assert!(plan.plan_steps.contains(&"patch parser".to_string()));
    assert_eq!(plan.touched_paths, state.workspace.touched_paths);
    assert_eq!(plan.selected_packages, state.context.selected_packages);
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
