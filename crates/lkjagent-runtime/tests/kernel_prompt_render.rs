use lkjagent_protocol::parse_completion;
use lkjagent_runtime::kernel::{
    build_snapshot, reduce, render_prompt_frame, PromptRenderError, RuntimeDecisionId,
    RuntimeEvent, RuntimeEventId, SnapshotAdapterInput,
};
use lkjagent_tools::dispatch::validate_action;

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
fn schema_repair_batch_example_is_concrete_and_parseable() -> Result<(), String> {
    let snapshot = build_snapshot(owner_input()).map_err(format_error)?;
    let mut decision =
        reduce(&snapshot, RuntimeEvent::SchemaFault { fault_key: None }).map_err(format_error)?;
    decision.decision_id = RuntimeDecisionId::Stored(77);
    decision.event_id = RuntimeEventId(10);
    let frame = render_prompt_frame(&decision).map_err(format_error)?;
    assert!(frame.contains("path: stories/chronos-fracture/README.md"));
    assert!(!frame.contains("[{\"path\""));
    let action_text = exact_action(&frame).ok_or_else(|| "missing action".to_string())?;
    let action = parse_completion(action_text).map_err(format_error)?;
    validate_action(&action).map_err(format_error)?;
    assert_eq!(action.tool, "fs.batch_write");
    assert!(action.params.iter().any(|param| param.name == "files"));
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

fn exact_action(text: &str) -> Option<&str> {
    let start = text.find("<action>")?;
    let end = text[start..]
        .find("</action>")?
        .saturating_add(start)
        .saturating_add("</action>".len());
    text.get(start..end)
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
