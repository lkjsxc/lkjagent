use lkjagent_runtime::kernel::{
    admit_requested_tool, build_snapshot, AdmissionRefusalKind, AdmissionRequest, RuntimeEvent,
    SnapshotAdapterInput, ToolName,
};
use lkjagent_runtime::mode::{
    decide_turn_authority, policy_for_mode, ActiveMode, TurnAuthorityInput,
};

#[test]
fn preferred_action_never_points_at_blocked_tool() {
    for mode in [
        ActiveMode::OwnerTask,
        ActiveMode::Recovery,
        ActiveMode::Maintenance,
        ActiveMode::Compaction,
        ActiveMode::ClosedIdle,
    ] {
        let policy = policy_for_mode(mode);
        assert_eq!(policy.blocked_preferred_tool(), None, "mode={mode:?}");
    }
}

#[test]
fn tool_requiring_prompt_never_has_empty_tool_surface() {
    let owner = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert_ne!(owner.effective_policy.allowed_tools, Vec::<&str>::new());
    assert!(!owner.prompt_card.contains("allowed_tools=none"));
    assert!(owner.prompt_card.contains("allowed_tools=fs.read"));
    assert!(owner.prompt_card.contains("artifact.apply"));
    assert!(owner.prompt_card.contains("graph.state"));
}

#[test]
fn kernel_owner_execution_does_not_admit_broad_log_surface() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-1".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/long-novel".to_string()),
        ..SnapshotAdapterInput::default()
    })
    .map_err(format_error)?;
    let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::OwnerMessageReceived)
        .map_err(format_error)?;
    let tools: Vec<&str> = decision
        .admission_view
        .admitted_tools
        .iter()
        .map(|tool| tool.as_str())
        .collect();
    assert!(!tools.contains(&"workspace.summary"));
    assert!(!tools.contains(&"graph.transition"));
    assert!(!tools.contains(&"graph.evidence"));
    Ok(())
}

#[test]
fn kernel_maintenance_excludes_owner_close_tools() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 2,
        maintenance_due: true,
        ..SnapshotAdapterInput::default()
    })
    .map_err(format_error)?;
    let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::MaintenanceTick)
        .map_err(format_error)?;
    let request = AdmissionRequest::new(
        ToolName::new("agent.done").map_err(format_error)?,
        snapshot.staleness_fingerprint,
        "done-fp",
    );
    let admission = admit_requested_tool(&decision.admission_view, request);
    assert_eq!(
        admission.refusal_kind,
        Some(AdmissionRefusalKind::BlockedTool)
    );
    Ok(())
}

#[test]
fn admission_refuses_non_current_decision_or_prompt_ids() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 3,
        case_id: Some("case-3".to_string()),
        queue_head: Some("queue-3".to_string()),
        pending_owner_count: 1,
        ..SnapshotAdapterInput::default()
    })
    .map_err(format_error)?;
    let mut decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::QueueChanged)
        .map_err(format_error)?;
    decision.admission_view = decision
        .admission_view
        .with_current_ids("decision-3", "frame-3");
    let stale_decision = AdmissionRequest::new(
        ToolName::new("artifact.apply").map_err(format_error)?,
        snapshot.staleness_fingerprint.clone(),
        "action-fp",
    )
    .with_current_ids("old-decision", "frame-3");
    assert_eq!(
        admit_requested_tool(&decision.admission_view, stale_decision).refusal_kind,
        Some(AdmissionRefusalKind::DecisionNotCurrent)
    );
    let stale_prompt = AdmissionRequest::new(
        ToolName::new("artifact.apply").map_err(format_error)?,
        snapshot.staleness_fingerprint,
        "action-fp",
    )
    .with_current_ids("decision-3", "old-frame");
    assert_eq!(
        admit_requested_tool(&decision.admission_view, stale_prompt).refusal_kind,
        Some(AdmissionRefusalKind::PromptFrameNotCurrent)
    );
    Ok(())
}

#[test]
fn valid_example_never_renders_blocked_tool() {
    let cases = [
        TurnAuthorityInput {
            recoverable_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            maintenance_due: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            compaction_required: true,
            ..TurnAuthorityInput::default()
        },
    ];
    for input in cases {
        let authority = decide_turn_authority(input);
        for blocked in &authority.effective_policy.blocked_tools {
            assert!(!authority
                .valid_example
                .contains(&format!("<tool>{blocked}</tool>")));
        }
    }
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
