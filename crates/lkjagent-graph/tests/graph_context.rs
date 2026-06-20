use lkjagent_graph::{
    compaction_plan, initial_state, select_context_packages, source_graph, ContextPressureLevel,
    EvidenceKind, EvidenceRecord, GraphNodeId,
};

#[test]
fn context_package_selection_is_family_and_node_aware() {
    let mut docs = initial_state("Create about 100 docs for a handbook.", None);
    docs.active_node = GraphNodeId("document-topology");
    let mut code = initial_state("fix parser bug", None);
    code.active_node = GraphNodeId("code-edit");

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
    assert!(select_context_packages(&source_graph(), &docs)
        .iter()
        .any(|binding| binding.package == "doc-construction"));
    assert!(select_context_packages(&source_graph(), &code)
        .iter()
        .any(|binding| binding.package == "execution-order"));
}

#[test]
fn pressure_changes_context_package_selection() {
    let mut state = initial_state("fix parser bug", None);
    state.active_node = GraphNodeId("execute");
    let green = select_context_packages(&source_graph(), &state);
    assert!(green.iter().any(|binding| binding.priority == "helpful"));

    state.context.pressure = ContextPressureLevel::Yellow;
    let yellow = select_context_packages(&source_graph(), &state);
    assert!(!yellow.iter().any(|binding| binding.priority == "helpful"));

    state.context.pressure = ContextPressureLevel::BlackInvalid;
    assert!(select_context_packages(&source_graph(), &state).is_empty());
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
    assert_eq!(plan.non_goals, state.objective.non_goals);
    assert!(plan
        .legal_next_transitions
        .iter()
        .any(|node| node == "execute-step"));
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
