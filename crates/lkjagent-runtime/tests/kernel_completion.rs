use lkjagent_runtime::kernel::{
    admit_requested_tool, build_snapshot, reduce, AdmissionRequest, RuntimeDecisionKind,
    RuntimeEvent, SnapshotAdapterInput, StalenessFingerprint, ToolName,
};

#[test]
fn kernel_close_case_admits_agent_done_when_evidence_complete() -> Result<(), String> {
    let snapshot = build_snapshot(complete_input()).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::CompletionRequested).map_err(format_error)?;

    assert_eq!(decision.kind, RuntimeDecisionKind::CloseCase);
    assert!(decision.completion_allowed);
    assert!(decision.admission_view.completion_allowed);
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("agent.done", snapshot.staleness_fingerprint)?,
    );
    assert!(admission.admitted);
    Ok(())
}

#[test]
fn kernel_blocks_agent_done_when_artifact_readiness_is_absent() -> Result<(), String> {
    let mut input = complete_input();
    input
        .existing_evidence
        .retain(|item| item != "artifact-readiness");
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::CompletionRequested).map_err(format_error)?;

    assert_eq!(decision.kind, RuntimeDecisionKind::BlockCompletion);
    assert!(!decision.completion_allowed);
    Ok(())
}

#[test]
fn kernel_blocks_agent_done_when_evidence_is_missing() -> Result<(), String> {
    let mut input = complete_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::CompletionRequested).map_err(format_error)?;

    assert_eq!(decision.kind, RuntimeDecisionKind::BlockCompletion);
    assert!(!decision.completion_allowed);
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("agent.done", snapshot.staleness_fingerprint)?,
    );
    assert!(!admission.admitted);
    Ok(())
}

fn complete_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-17".to_string()),
        artifact_root: Some("stories/chronos-fracture".to_string()),
        required_evidence: vec![
            "plan".to_string(),
            "observation".to_string(),
            "document-structure".to_string(),
            "artifact-readiness".to_string(),
        ],
        existing_evidence: vec![
            "plan".to_string(),
            "observation".to_string(),
            "document-structure".to_string(),
            "artifact-readiness".to_string(),
        ],
        ..SnapshotAdapterInput::default()
    }
}

fn request(tool: &str, fingerprint: StalenessFingerprint) -> Result<AdmissionRequest, String> {
    let tool = ToolName::new(tool).map_err(format_error)?;
    Ok(AdmissionRequest::new(tool, fingerprint, "action-fp"))
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
