use lkjagent_runtime::kernel::{
    build_snapshot, reduce, ActionTemplate, RuntimeEvent, RuntimeMission, SnapshotAdapterInput,
};

fn owner_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-1".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/long-novel-with-detailed-settings".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

#[test]
fn missing_plan_forces_graph_plan() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["plan".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::OwnerExecution);
    assert_eq!(next_tool(&decision)?, "graph.plan");
    assert!(decision
        .admission_view
        .admitted_tools
        .iter()
        .any(|tool| tool.as_str() == "graph.plan"));
    Ok(())
}

#[test]
fn missing_structure_forces_artifact_apply() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    assert_eq!(next_tool(&decision)?, "artifact.apply");
    Ok(())
}

#[test]
fn weak_paths_force_artifact_next() -> Result<(), String> {
    let mut input = owner_input();
    input.artifact_weak_paths = vec!["project/premise.md".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::ArtifactWeakPathFound).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::ArtifactRepair);
    assert_eq!(next_tool(&decision)?, "artifact.next");
    Ok(())
}

#[test]
fn artifact_next_candidate_forces_batch_write() -> Result<(), String> {
    let input = owner_input();
    let mut snapshot = build_snapshot(input).map_err(format_error)?;
    snapshot.artifact.weak_paths = vec!["project/premise.md".to_string()];
    snapshot.observation.latest = Some("next_decision_required=true".to_string());
    let decision = reduce(&snapshot, RuntimeEvent::ArtifactWeakPathFound).map_err(format_error)?;
    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    Ok(())
}

#[test]
fn exhausted_weak_paths_force_artifact_audit() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    assert_eq!(next_tool(&decision)?, "artifact.audit");
    Ok(())
}

#[test]
fn repeated_child_tag_schema_fault_changes_to_artifact_next() -> Result<(), String> {
    let mut input = owner_input();
    input.retry_count = 1;
    input.parameter_shape_fingerprint = Some("child-file-tags".to_string());
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision =
        reduce(&snapshot, RuntimeEvent::SchemaFault { fault_key: None }).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::SchemaRepair);
    assert_eq!(next_tool(&decision)?, "artifact.next");
    Ok(())
}

#[test]
fn provider_anomaly_selects_owner_recovery() -> Result<(), String> {
    let mut input = owner_input();
    input.provider_anomaly_class = Some("reasoning_only_response".to_string());
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let decision = reduce(
        &snapshot,
        RuntimeEvent::ProviderAnomaly {
            class: "reasoning_only_response".to_string(),
        },
    )
    .map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::OwnerRecovery);
    Ok(())
}

fn next_tool(decision: &lkjagent_runtime::kernel::RuntimeDecision) -> Result<&str, String> {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => Ok(tool.as_str()),
        _ => Err("expected exact next tool".to_string()),
    }
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
