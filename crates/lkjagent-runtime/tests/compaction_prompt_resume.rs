use lkjagent_runtime::kernel::{
    build_snapshot, reduce, ActionTemplate, RuntimeDecisionKind, RuntimeEffectCommand,
    RuntimeEvent, SnapshotAdapterInput,
};

#[test]
fn hard_compaction_resumes_same_next_action_after_post_snapshot() -> Result<(), String> {
    let before = build_snapshot(owner_repair_input(false)).map_err(format_error)?;
    let before_decision =
        reduce(&before, RuntimeEvent::ArtifactWeakPathFound).map_err(format_error)?;
    assert_eq!(next_tool(&before_decision)?, "artifact.next");

    let pressure = build_snapshot(owner_repair_input(true)).map_err(format_error)?;
    let pressure_decision =
        reduce(&pressure, RuntimeEvent::ContextPressureDetected).map_err(format_error)?;
    assert_eq!(pressure_decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert_eq!(
        pressure_decision.runtime_effect,
        Some(RuntimeEffectCommand::CompactNow)
    );

    let after = build_snapshot(owner_repair_input(false)).map_err(format_error)?;
    let after_decision = reduce(&after, RuntimeEvent::CompactionCompleted).map_err(format_error)?;
    assert_eq!(next_tool(&after_decision)?, next_tool(&before_decision)?);
    Ok(())
}

fn owner_repair_input(hard_pressure: bool) -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: if hard_pressure { 2 } else { 1 },
        case_id: Some("case-9".to_string()),
        queue_head: Some("queue-9".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/novel".to_string()),
        artifact_cursor: Some("project/premise.md".to_string()),
        artifact_weak_paths: vec!["project/premise.md".to_string()],
        latest_observation: Some("doc.audit found weak paths".to_string()),
        context_hard_pressure: hard_pressure,
        prompt_frame_fingerprint: Some("prompt-frame-9".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

fn next_tool(decision: &lkjagent_runtime::kernel::RuntimeDecision) -> Result<&str, String> {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => Ok(tool.as_str()),
        _ => Err("missing exact next tool".to_string()),
    }
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
