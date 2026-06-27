use lkjagent_runtime::kernel::{
    build_snapshot, reduce, RuntimeDecisionKind, RuntimeEvent, RuntimeMission, SnapshotAdapterInput,
};

#[test]
fn empty_maintenance_noop_returns_to_closed_idle() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 1,
        maintenance_active: true,
        maintenance_cooldown: true,
        ..SnapshotAdapterInput::default()
    })
    .map_err(format_error)?;
    let decision =
        reduce(&snapshot, RuntimeEvent::MaintenanceNoopCooldownRecorded).map_err(format_error)?;

    assert_eq!(decision.mission, RuntimeMission::ClosedIdle);
    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert!(decision.runtime_effect.is_some());
    assert!(decision.admission_view.admitted_tools.is_empty());
    Ok(())
}

#[test]
fn owner_queue_preempts_maintenance_before_model_call() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 2,
        case_id: Some("case-2".to_string()),
        queue_head: Some("queue-2".to_string()),
        pending_owner_count: 1,
        maintenance_due: true,
        ..SnapshotAdapterInput::default()
    })
    .map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::MaintenanceDue).map_err(format_error)?;

    assert_eq!(decision.mission, RuntimeMission::OwnerExecution);
    assert!(decision
        .admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == "memory.find"));
    Ok(())
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
