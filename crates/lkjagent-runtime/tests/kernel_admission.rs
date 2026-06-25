use lkjagent_runtime::kernel::{
    admit_requested_tool, build_snapshot, reduce, AdmissionRefusalKind, AdmissionRequest,
    RuntimeEvent, RuntimeFault, SnapshotAdapterInput, StalenessFingerprint, ToolName,
};

fn owner_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-17".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/chronos-fracture".to_string()),
        artifact_cursor: Some("README.md".to_string()),
        missing_evidence: vec!["artifact-readiness".to_string()],
        ..SnapshotAdapterInput::default()
    }
}

fn request(tool: &str, fingerprint: StalenessFingerprint) -> Result<AdmissionRequest, String> {
    let tool = ToolName::new(tool).map_err(format_error)?;
    Ok(AdmissionRequest::new(tool, fingerprint, "action-fp"))
}

#[test]
fn queued_owner_work_refuses_cached_maintenance_action() -> Result<(), String> {
    let idle = SnapshotAdapterInput {
        maintenance_due: true,
        ..SnapshotAdapterInput::default()
    };
    let idle_snapshot = build_snapshot(idle).map_err(format_error)?;
    let idle_decision =
        reduce(&idle_snapshot, RuntimeEvent::MaintenanceTick).map_err(format_error)?;
    let owner_snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let admission = admit_requested_tool(
        &idle_decision.admission_view,
        request("memory.find", owner_snapshot.staleness_fingerprint)?,
    );
    assert!(!admission.admitted);
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::StaleDecision)
    );
    Ok(())
}

#[test]
fn changed_artifact_cursor_refuses_cached_batch_write() -> Result<(), String> {
    let original = build_snapshot(owner_input()).map_err(format_error)?;
    let decision = reduce(&original, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    let mut changed_input = owner_input();
    changed_input.artifact_cursor = Some("bible/timeline.md".to_string());
    let changed = build_snapshot(changed_input).map_err(format_error)?;
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("fs.batch_write", changed.staleness_fingerprint)?,
    );
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::StaleDecision)
    );
    Ok(())
}

#[test]
fn compaction_pressure_refuses_cached_content_write() -> Result<(), String> {
    let original = build_snapshot(owner_input()).map_err(format_error)?;
    let decision = reduce(&original, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    let mut changed_input = owner_input();
    changed_input.context_hard_pressure = true;
    let changed = build_snapshot(changed_input).map_err(format_error)?;
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("fs.batch_write", changed.staleness_fingerprint)?,
    );
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::StaleDecision)
    );
    Ok(())
}

#[test]
fn blocked_tool_does_not_reach_dispatch() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("memory.find", snapshot.staleness_fingerprint)?,
    );
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::BlockedTool)
    );
    assert!(admission.exact_next_action.is_some());
    Ok(())
}

#[test]
fn exhausted_fault_fingerprint_cannot_be_admitted_again() -> Result<(), String> {
    let mut input = owner_input();
    input.latest_fault = Some(RuntimeFault::Parse);
    input.retry_count = 2;
    input.prior_action_fingerprint = Some("action-fp".to_string());
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision =
        reduce(&snapshot, RuntimeEvent::ParseFault { fault_key: None }).map_err(format_error)?;

    let admission = admit_requested_tool(
        &decision.admission_view,
        request("graph.state", snapshot.staleness_fingerprint)?,
    );

    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::RepeatFingerprintExhausted)
    );
    assert!(admission.reason.contains("Parse"));
    Ok(())
}

#[test]
fn completion_request_uses_completion_block() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::CompletionRequested).map_err(format_error)?;
    let admission = admit_requested_tool(
        &decision.admission_view,
        request("agent.done", snapshot.staleness_fingerprint)?,
    );
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::CompletionBlocked)
    );
    Ok(())
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
