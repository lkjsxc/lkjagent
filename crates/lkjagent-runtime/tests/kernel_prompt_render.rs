use lkjagent_runtime::kernel::{
    build_snapshot, reduce, render_prompt_frame, PromptRenderError, RuntimeDecisionId,
    RuntimeEvent, RuntimeEventId, SnapshotAdapterInput,
};

fn owner_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 7,
        case_id: Some("case-chronos".to_string()),
        graph_node: Some("document".to_string()),
        graph_phase: Some("execution".to_string()),
        owner_objective: Some("create story bible".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        missing_evidence: vec!["artifact-readiness".to_string()],
        artifact_root: Some("stories/chronos-fracture".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

#[test]
fn prompt_frame_requires_persisted_decision_id() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let decision = reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    assert_eq!(
        render_prompt_frame(&decision),
        Err(PromptRenderError::DecisionNotPersisted)
    );
    Ok(())
}

#[test]
fn prompt_frame_requires_persisted_event_id() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let mut decision =
        reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    decision.decision_id = RuntimeDecisionId::Stored(42);
    assert_eq!(
        render_prompt_frame(&decision),
        Err(PromptRenderError::EventNotPersisted)
    );
    Ok(())
}

#[test]
fn owner_prompt_cites_decision_id_and_fingerprints() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let mut decision =
        reduce(&snapshot, RuntimeEvent::OwnerMessageReceived).map_err(format_error)?;
    decision.decision_id = RuntimeDecisionId::Stored(42);
    decision.event_id = RuntimeEventId(9);
    let frame = render_prompt_frame(&decision).map_err(format_error)?;
    assert!(frame.contains("<runtime-card>"));
    assert!(frame.contains("<decision>42</decision>"));
    assert!(frame.contains("<event>9</event>"));
    assert!(frame.contains("<authority>authority:"));
    assert!(frame.contains("<staleness>stale:"));
    assert!(frame.contains("<budget>512 output tokens</budget>"));
    assert!(frame.contains("<must-use>artifact.audit</must-use>"));
    assert!(!frame.contains("admitted_tools="));
    Ok(())
}

#[test]
fn schema_repair_renders_write_contract_not_prefilled_batch() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let mut decision =
        reduce(&snapshot, RuntimeEvent::SchemaFault { fault_key: None }).map_err(format_error)?;
    decision.decision_id = RuntimeDecisionId::Stored(77);
    decision.event_id = RuntimeEventId(10);
    let frame = render_prompt_frame(&decision).map_err(format_error)?;
    assert!(frame.contains("<write-contract>"));
    assert!(frame.contains("<tool>fs.batch_write</tool>"));
    assert!(frame.contains("<root>stories/chronos-fracture</root>"));
    assert!(frame.contains("author exactly one fs.batch_write action"));
    assert!(!frame.contains("[{\"path\""));
    assert!(!frame.contains("path: stories/chronos-fracture/README.md"));
    Ok(())
}

#[test]
fn runtime_effect_decision_produces_no_prompt() -> Result<(), String> {
    let snapshot = build_snapshot(SnapshotAdapterInput::default()).map_err(format_error)?;
    let mut decision = reduce(&snapshot, RuntimeEvent::MaintenanceTick).map_err(format_error)?;
    decision.decision_id = RuntimeDecisionId::Stored(88);
    decision.event_id = RuntimeEventId(11);
    assert_eq!(
        render_prompt_frame(&decision),
        Err(PromptRenderError::RuntimeEffectHasNoPrompt)
    );
    Ok(())
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
