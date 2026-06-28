use lkjagent_runtime::kernel::{
    reduce, ActionTemplate, ArtifactFacts, AuthorityFingerprint, CaseFacts, ContextFacts,
    EvidenceFacts, GraphFacts, MaintenanceFacts, ProviderFacts, QueueFacts, RuntimeEvent,
    RuntimeFault, RuntimeSnapshot, RuntimeSnapshotId, RuntimeSnapshotInput, StalenessFingerprint,
};

fn snapshot() -> Result<RuntimeSnapshot, String> {
    let authority_fingerprint = AuthorityFingerprint::new("authority:1").map_err(format_error)?;
    let staleness_fingerprint = StalenessFingerprint::new("stale:1").map_err(format_error)?;
    Ok(RuntimeSnapshot::new(RuntimeSnapshotInput {
        snapshot_id: RuntimeSnapshotId(1),
        case: CaseFacts::default(),
        graph: GraphFacts::default(),
        queue: QueueFacts::default(),
        evidence: EvidenceFacts::default(),
        artifact: ArtifactFacts::default(),
        context: ContextFacts::default(),
        maintenance: MaintenanceFacts::default(),
        provider: ProviderFacts::default(),
        authority_fingerprint,
        staleness_fingerprint,
    }))
}

#[test]
fn code_change_file_task_forces_direct_write_after_plan() -> Result<(), String> {
    let mut state = snapshot()?;
    state.case.case_id = Some("case-1".to_string());
    state.case.task_family = Some("code-change".to_string());
    state.case.owner_objective = Some("Create hello.md with one hello sentence.".to_string());
    state.evidence.existing = vec!["plan".to_string()];
    state.evidence.missing = vec!["observation".to_string(), "verification".to_string()];
    let decision = reduce(&state, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;

    match decision.forced_next_action {
        Some(ActionTemplate::ExactTool { tool, body }) => {
            assert_eq!(tool.as_str(), "fs.write");
            assert!(body.contains("<path>hello.md</path>"));
            assert!(body.contains("Hello."));
            Ok(())
        }
        _ => Err("expected direct write action".to_string()),
    }
}

#[test]
fn audit_owned_gap_blocks_direct_graph_evidence() -> Result<(), String> {
    let mut state = snapshot()?;
    state.case.case_id = Some("case-1".to_string());
    state.artifact.root = Some("stories/chronos-fracture".to_string());
    state.evidence.missing = vec!["artifact-readiness".to_string()];
    let decision = reduce(&state, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    let tools = tool_names(&decision);

    assert!(!tools.contains(&"graph.evidence"));
    assert!(decision
        .admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == "graph.evidence"));
    match decision.forced_next_action {
        Some(ActionTemplate::ExactTool { tool, body }) => {
            assert_eq!(tool.as_str(), "artifact.audit");
            assert!(!body.contains("<tool>graph.evidence</tool>"));
            Ok(())
        }
        _ => Err("expected artifact audit repair action".to_string()),
    }
}

#[test]
fn repeated_recovery_forces_different_progress_action() -> Result<(), String> {
    let mut state = snapshot()?;
    state.case.case_id = Some("case-1".to_string());
    state.artifact.root = Some("stories/chronos-fracture".to_string());
    state.latest_fault = Some(RuntimeFault::Repeat);
    state.retry_count = 2;
    state.prior_action_fingerprint = Some("graph-state-repeat".to_string());
    let decision = reduce(&state, RuntimeEvent::TurnBudgetExhausted).map_err(format_error)?;

    match decision.forced_next_action {
        Some(ActionTemplate::ExactTool { tool, body }) => {
            assert_eq!(tool.as_str(), "artifact.audit");
            assert!(!body.contains("<tool>graph.state</tool>"));
            Ok(())
        }
        _ => Err("expected different recovery action".to_string()),
    }
}

fn tool_names(decision: &lkjagent_runtime::kernel::RuntimeDecision) -> Vec<&str> {
    decision
        .admission_view
        .admitted_tools
        .iter()
        .map(|tool| tool.as_str())
        .collect()
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
