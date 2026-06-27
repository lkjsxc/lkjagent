use lkjagent_runtime::kernel::{
    admit_requested_tool, build_snapshot, AdmissionRefusalKind, AdmissionRequest,
    RuntimeDecisionKind, RuntimeEvent, RuntimeMission, SnapshotAdapterInput, ToolName,
};

#[test]
fn case_closed_event_uses_completion_gate() -> Result<(), String> {
    let snapshot = build_snapshot(complete_input()).map_err(format_error)?;
    let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::CaseClosed)
        .map_err(format_error)?;

    assert_eq!(decision.mission, RuntimeMission::OwnerCompletion);
    assert_eq!(decision.kind, RuntimeDecisionKind::CloseCase);
    assert!(decision.completion_allowed);
    Ok(())
}

#[test]
fn close_path_refuses_with_artifact_weak_paths() -> Result<(), String> {
    let mut input = complete_input();
    input.artifact_weak_paths = vec!["project/premise.md".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::CaseClosed)
        .map_err(format_error)?;

    assert_eq!(decision.mission, RuntimeMission::ArtifactRepair);
    let admission = admit_requested_tool(
        &decision.admission_view,
        AdmissionRequest::new(
            ToolName::new("agent.done").map_err(format_error)?,
            snapshot.staleness_fingerprint,
            "done-fp",
        ),
    );
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::CompletionBlocked)
    );
    Ok(())
}

fn complete_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-17".to_string()),
        queue_head: Some("queue-17".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/chronos-fracture".to_string()),
        existing_evidence: vec![
            "plan".to_string(),
            "observation".to_string(),
            "document-structure".to_string(),
            "artifact-readiness".to_string(),
        ],
        ..SnapshotAdapterInput::default()
    }
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
