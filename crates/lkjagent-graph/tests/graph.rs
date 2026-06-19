use lkjagent_graph::{
    admit_transition, compaction_plan, completion_decision, initial_state, render_graph_slice,
    source_graph, validate_graph, EvidenceKind, EvidenceRecord, GraphNodeId, TaskFamily,
    TransitionDecision,
};

#[test]
fn source_graph_validates() {
    let report = validate_graph(source_graph());

    assert!(report.is_ok(), "{:?}", report.violations);
}

#[test]
fn classification_opens_planning_case_with_requirements() {
    let state = initial_state("redesign architecture docs and runtime", Some(7));

    assert_eq!(state.family, TaskFamily::Architecture);
    assert_eq!(state.active_node, GraphNodeId("plan"));
    assert!(state.evidence_requirements.contains(&"plan".to_string()));
    assert!(state
        .selected_packages
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
        .selected_packages
        .contains(&"doc-construction".to_string()));
    assert!(state
        .evidence_requirements
        .contains(&"document-structure".to_string()));
}

#[test]
fn counted_japanese_story_request_selects_document_construction() {
    let state = initial_state(
        "100ファイルぐらいの大きな物語を、設計メモと本文に分けて作って。",
        Some(10),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_english_artifact_request_selects_document_construction() {
    let state = initial_state(
        "Create about 100 files total for a structured artifact with docs and main content.",
        Some(11),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_japanese_artifact_request_selects_document_construction() {
    let state = initial_state(
        "合計百ファイルほどの大きな成果物を、設計メモと本文に分けて作ってください。",
        Some(12),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_architecture_artifact_request_selects_document_construction() {
    let state = initial_state(
        "Create about 100 files total for a structured architecture artifact with docs and main content.",
        Some(13),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_test_corpus_request_selects_document_construction() {
    let state = initial_state(
        "Create about 100 files total for a test corpus with docs and main content.",
        Some(14),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_implementation_request_stays_code_change() {
    let state = initial_state("Create exactly 3 files implementing a Rust CLI.", Some(9));

    assert_eq!(state.family, TaskFamily::CodeChange);
    assert!(!state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_implementation_with_tests_stays_verification() {
    let state = initial_state(
        "Create exactly 3 files implementing tests for a Rust CLI.",
        Some(15),
    );

    assert_eq!(state.family, TaskFamily::Verification);
    assert!(!state
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn graph_slice_is_deterministic_and_budgeted() {
    let state = initial_state("write docs", None);
    let first = render_graph_slice(source_graph(), &state, 128);
    let second = render_graph_slice(source_graph(), &state, 128);

    assert_eq!(first, second);
    assert!(first.contains("phase=planning"));
    assert!(first.len() <= 512);
}

#[test]
fn completion_requires_evidence_and_pending_checks_clear() {
    let mut state = initial_state("fix parser bug", Some(1));
    assert!(matches!(
        completion_decision(&state),
        TransitionDecision::Defer { .. }
    ));

    state.evidence.push(record("plan"));
    state.evidence.push(record("observation"));
    state.evidence.push(record("verification"));
    state.pending_checks.clear();

    assert_eq!(
        completion_decision(&state),
        TransitionDecision::Admit {
            target: GraphNodeId("complete")
        }
    );
}

#[test]
fn transition_and_compaction_preserve_structured_state() {
    let mut state = initial_state("fix parser bug", Some(3));
    state.active_node = GraphNodeId("execute");
    state
        .touched_paths
        .push("crates/lkjagent-protocol/src/parse.rs".to_string());
    state.evidence.push(record("plan"));

    assert!(matches!(
        admit_transition(source_graph(), &state, GraphNodeId("verify")),
        TransitionDecision::Admit { .. }
    ));
    let plan = compaction_plan(&state);
    assert_eq!(plan.case_id, Some(3));
    assert_eq!(plan.active_node, GraphNodeId("execute"));
    assert!(plan.missing_evidence.contains(&"observation".to_string()));
    assert_eq!(plan.touched_paths, state.touched_paths);
}

fn record(requirement: &str) -> EvidenceRecord {
    EvidenceRecord {
        requirement: requirement.to_string(),
        kind: EvidenceKind::Observation,
        summary: format!("{requirement} observed"),
        path: None,
    }
}
