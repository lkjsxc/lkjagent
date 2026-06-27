use lkjagent_runtime::kernel::{
    build_snapshot, ActiveMode, RuntimeFault, SnapshotAdapterError, SnapshotAdapterInput,
};

fn owner_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-17".to_string()),
        graph_node: Some("document".to_string()),
        graph_phase: Some("execution".to_string()),
        owner_objective: Some("finish Chronos story bible".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        missing_evidence: vec!["artifact-readiness".to_string()],
        artifact_id: Some("artifact-17".to_string()),
        artifact_root: Some("stories/chronos-fracture".to_string()),
        artifact_cursor: Some("README.md".to_string()),
        prompt_frame_fingerprint: Some("prompt-1".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

#[test]
fn owner_queue_changes_staleness_fingerprint() -> Result<(), String> {
    let first = build_snapshot(owner_input()).map_err(format_error)?;
    let mut changed = owner_input();
    changed.queue_head = Some("queue-2".to_string());
    let second = build_snapshot(changed).map_err(format_error)?;
    assert_ne!(first.staleness_fingerprint, second.staleness_fingerprint);
    Ok(())
}

#[test]
fn active_mode_hint_changes_staleness_fingerprint() -> Result<(), String> {
    let first = build_snapshot(owner_input()).map_err(format_error)?;
    let mut changed = owner_input();
    changed.active_mode_hint = Some("Recovery".to_string());
    let second = build_snapshot(changed).map_err(format_error)?;
    assert_ne!(first.staleness_fingerprint, second.staleness_fingerprint);
    Ok(())
}

#[test]
fn artifact_cursor_changes_staleness_fingerprint() -> Result<(), String> {
    let first = build_snapshot(owner_input()).map_err(format_error)?;
    let mut changed = owner_input();
    changed.artifact_cursor = Some("bible/era.md".to_string());
    let second = build_snapshot(changed).map_err(format_error)?;
    assert_ne!(first.staleness_fingerprint, second.staleness_fingerprint);
    Ok(())
}

#[test]
fn compaction_pressure_changes_staleness_fingerprint() -> Result<(), String> {
    let first = build_snapshot(owner_input()).map_err(format_error)?;
    let mut changed = owner_input();
    changed.context_hard_pressure = true;
    let second = build_snapshot(changed).map_err(format_error)?;
    assert_ne!(first.staleness_fingerprint, second.staleness_fingerprint);
    assert_eq!(second.active_mode, ActiveMode::Compaction);
    Ok(())
}

#[test]
fn maintenance_due_is_ignored_when_owner_work_exists() -> Result<(), String> {
    let mut first_input = owner_input();
    first_input.maintenance_due = false;
    let first = build_snapshot(first_input).map_err(format_error)?;
    let mut second_input = owner_input();
    second_input.maintenance_due = true;
    let second = build_snapshot(second_input).map_err(format_error)?;
    assert_eq!(first.staleness_fingerprint, second.staleness_fingerprint);
    assert!(!second.maintenance.due);
    Ok(())
}

#[test]
fn active_owner_work_rejects_synthetic_case_id() {
    let mut missing = owner_input();
    missing.case_id = None;
    assert!(build_snapshot(missing).is_ok());
    let mut synthetic = owner_input();
    synthetic.case_id = Some("case:unknown".to_string());
    assert_eq!(
        build_snapshot(synthetic),
        Err(SnapshotAdapterError::SyntheticCaseId(
            "case:unknown".to_string()
        ))
    );
}

#[test]
fn latest_fault_selects_recovery_mode() -> Result<(), String> {
    let mut input = owner_input();
    input.latest_fault = Some(RuntimeFault::Parse);
    let snapshot = build_snapshot(input).map_err(format_error)?;
    assert_eq!(snapshot.active_mode, ActiveMode::Recovery);
    Ok(())
}

#[test]
fn provider_anomaly_and_observation_change_staleness() -> Result<(), String> {
    let first = build_snapshot(owner_input()).map_err(format_error)?;
    let mut changed = owner_input();
    changed.provider_anomaly_class = Some("reasoning_only_response".to_string());
    changed.provider_retry_count = 1;
    changed.latest_observation = Some("schema fault".to_string());
    changed.latest_successful_observation = Some("artifact scaffolded".to_string());
    let second = build_snapshot(changed).map_err(format_error)?;
    assert_ne!(first.staleness_fingerprint, second.staleness_fingerprint);
    assert_eq!(
        second.provider.anomaly_class.as_deref(),
        Some("reasoning_only_response")
    );
    assert_eq!(second.observation.latest.as_deref(), Some("schema fault"));
    assert_eq!(
        second.observation.latest_successful.as_deref(),
        Some("artifact scaffolded")
    );
    Ok(())
}

fn format_error(error: SnapshotAdapterError) -> String {
    format!("{error:?}")
}
