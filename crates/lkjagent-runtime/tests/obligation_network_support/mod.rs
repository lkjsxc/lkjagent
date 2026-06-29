use lkjagent_runtime::kernel::{
    build_snapshot, reduce, ActionTemplate, FaultClass, FaultKey, RuntimeDecision, RuntimeEvent,
    SnapshotAdapterInput,
};

pub fn owner_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-1".to_string()),
        queue_head: Some("queue-1".to_string()),
        pending_owner_count: 1,
        artifact_root: Some("stories/novel".to_string()),
        artifact_kind: Some("story".to_string()),
        owner_objective: Some("Create a long novel with structured settings".to_string()),
        ..SnapshotAdapterInput::default()
    }
}

pub fn decision(
    input: SnapshotAdapterInput,
    event: RuntimeEvent,
) -> Result<RuntimeDecision, String> {
    let snapshot = build_snapshot(input).map_err(format_error)?;
    reduce(&snapshot, event).map_err(format_error)
}

pub fn next_tool(decision: &RuntimeDecision) -> Result<&str, String> {
    match decision.forced_next_action.as_ref() {
        Some(ActionTemplate::ExactTool { tool, .. }) => Ok(tool.as_str()),
        _ => Err("expected exact tool".to_string()),
    }
}

pub fn identity_paths(decision: &RuntimeDecision) -> Result<Vec<String>, String> {
    decision
        .content_write_contract
        .as_ref()
        .map(|contract| contract.paths.clone())
        .ok_or_else(|| "missing write contract".to_string())
}

pub fn story_identity_paths(root: &str) -> Vec<String> {
    [
        "catalog.toml",
        "README.md",
        "objective.md",
        "setting-overview.md",
        "cast.md",
    ]
    .into_iter()
    .map(|path| format!("{root}/{path}"))
    .collect()
}

pub fn blocked(decision: &RuntimeDecision, name: &str) -> bool {
    decision
        .admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == name)
}

pub fn admitted(decision: &RuntimeDecision, name: &str) -> bool {
    decision
        .admission_view
        .admitted_tools
        .iter()
        .any(|tool| tool.as_str() == name)
}

pub fn repeat_event() -> RuntimeEvent {
    RuntimeEvent::RepeatActionDetected {
        fault_key: FaultKey::new(FaultClass::Repeat).with_tool("doc.audit"),
    }
}

pub fn missing_root_observation(root: &str) -> String {
    format!(
        "document audit failed\nroot={root}\ntopology=failed\ncontent_readiness=not-requested\nfailed=1\nfailures:\n- missing_root: {root}\nnext_action=artifact.next or fs.batch_write exact failed topology"
    )
}

pub fn artifact_next_missing_root(root: &str) -> String {
    format!(
        "artifact_next_result=root_missing\nroot={root}\nkind=story\nmissing=root\nruntime_event=ArtifactRootMissing\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\ntool=fs.batch_write\nroot={root}\nkind=story\npaths:\n- {root}/catalog.toml\n- {root}/README.md\n- {root}/objective.md\n- {root}/setting-overview.md\n- {root}/cast.md"
    )
}

pub fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
