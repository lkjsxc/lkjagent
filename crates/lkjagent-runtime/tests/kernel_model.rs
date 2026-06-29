use lkjagent_runtime::kernel::{
    reduce, reduce_with_event_id, select_mission, ArtifactFacts, AuthorityFingerprint, CaseFacts,
    ContextFacts, EvidenceFacts, GraphFacts, MaintenanceFacts, ProviderFacts, QueueFacts,
    RuntimeDecision, RuntimeDecisionId, RuntimeDecisionInput, RuntimeDecisionKind, RuntimeEvent,
    RuntimeEventId, RuntimeEventKind, RuntimeMission, RuntimeSnapshot, RuntimeSnapshotId,
    RuntimeSnapshotInput, StalenessFingerprint, ToolAdmissionView,
};

fn snapshot() -> Result<RuntimeSnapshot, String> {
    let authority_fingerprint = AuthorityFingerprint::new("authority:1").map_err(format_error)?;
    let staleness_fingerprint = StalenessFingerprint::new("stale:1").map_err(format_error)?;
    Ok(RuntimeSnapshot::new(RuntimeSnapshotInput {
        snapshot_id: RuntimeSnapshotId(1),
        case: CaseFacts::default(),
        graph: GraphFacts::default(),
        queue: QueueFacts::default(),
        evidence: EvidenceFacts::default(),
        artifact: ArtifactFacts::default(),
        context: ContextFacts::default(),
        maintenance: MaintenanceFacts::default(),
        provider: ProviderFacts::default(),
        authority_fingerprint,
        staleness_fingerprint,
    }))
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

fn tool_names(decision: &RuntimeDecision) -> Vec<&str> {
    decision
        .admission_view
        .admitted_tools
        .iter()
        .map(|tool| tool.as_str())
        .collect()
}

#[test]
fn mission_priority_table_matches_contract() {
    let names: Vec<&str> = RuntimeMission::PRIORITY
        .iter()
        .map(|mission| mission.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            "hard_runtime_compaction",
            "owner_recovery",
            "schema_repair",
            "artifact_repair",
            "verification_repair",
            "owner_execution",
            "owner_verification",
            "owner_completion",
            "idle_maintenance",
            "closed_idle",
        ]
    );
}

#[test]
fn event_kind_strings_are_canonical_snake_case() {
    assert_eq!(RuntimeEventKind::CaseResumed.as_str(), "case_resumed");
    assert_eq!(
        RuntimeEventKind::TurnBudgetExhausted.as_str(),
        "turn_budget_exhausted"
    );
    assert_eq!(
        RuntimeEventKind::ContextPressureDetected.as_str(),
        "context_pressure_detected"
    );
}

#[test]
fn context_pressure_outranks_owner_execution() -> Result<(), String> {
    let mut state = snapshot()?;
    state.context.hard_pressure = true;
    state.queue.pending_owner_count = 1;
    let event = RuntimeEvent::OwnerMessageReceived;
    assert_eq!(
        select_mission(&state, &event),
        RuntimeMission::HardRuntimeCompaction
    );
    let decision = reduce(&state, event).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::HardRuntimeCompaction);
    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert!(decision.admission_view.admitted_tools.is_empty());
    assert!(decision.runtime_effect.is_some());
    Ok(())
}

#[test]
fn owner_work_outranks_maintenance() -> Result<(), String> {
    let mut state = snapshot()?;
    state.queue.pending_owner_count = 1;
    state.maintenance.due = true;
    let decision = reduce(&state, RuntimeEvent::MaintenanceTick).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::OwnerExecution);
    assert!(tool_names(&decision).contains(&"artifact.next"));
    assert!(decision
        .admission_view
        .blocked_tools
        .iter()
        .any(|tool| tool.as_str() == "memory.find"));
    Ok(())
}

#[test]
fn schema_repair_admits_batch_and_escape_tools() -> Result<(), String> {
    let mut state = snapshot()?;
    state.artifact.root = Some("stories/chronos-fracture".to_string());
    let decision =
        reduce(&state, RuntimeEvent::SchemaFault { fault_key: None }).map_err(format_error)?;
    let tools = tool_names(&decision);
    assert_eq!(decision.mission, RuntimeMission::SchemaRepair);
    assert!(tools.contains(&"fs.batch_write"));
    Ok(())
}

#[test]
fn model_call_decision_rejects_empty_admitted_tools() -> Result<(), String> {
    let state = snapshot()?;
    let admission = ToolAdmissionView::new(
        RuntimeMission::OwnerExecution.active_mode(),
        Vec::new(),
        Vec::new(),
        state.staleness_fingerprint.clone(),
    );
    let input = RuntimeDecisionInput {
        decision_id: RuntimeDecisionId::Pending,
        snapshot_id: RuntimeSnapshotId(1),
        event_id: RuntimeEventId(1),
        mission: RuntimeMission::OwnerExecution,
        kind: RuntimeDecisionKind::ModelCall,
        admission_view: admission,
        authority_fingerprint: state.authority_fingerprint.clone(),
        staleness_fingerprint: state.staleness_fingerprint.clone(),
    };
    assert!(RuntimeDecision::new(input).is_err());
    Ok(())
}

#[test]
fn closed_idle_allows_empty_tools_only_as_runtime_effect() -> Result<(), String> {
    let state = snapshot()?;
    let decision = reduce(&state, RuntimeEvent::MaintenanceTick).map_err(format_error)?;
    assert_eq!(decision.mission, RuntimeMission::ClosedIdle);
    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert!(decision.admission_view.admitted_tools.is_empty());
    assert!(decision.runtime_effect.is_some());
    Ok(())
}

#[test]
fn completion_with_missing_artifact_readiness_is_blocked() -> Result<(), String> {
    let mut state = snapshot()?;
    state.case.case_id = Some("case-1".to_string());
    state.evidence.missing = vec!["artifact-readiness".to_string()];
    let decision = reduce(&state, RuntimeEvent::CompletionRequested).map_err(format_error)?;
    let tools = tool_names(&decision);
    assert_eq!(decision.mission, RuntimeMission::OwnerCompletion);
    assert_eq!(decision.kind, RuntimeDecisionKind::RuntimeEffect);
    assert_eq!(tools, vec!["artifact.plan"]);
    assert!(!tools.contains(&"agent.done"));
    Ok(())
}

#[test]
fn one_event_emits_one_decision_with_event_and_snapshot_ids() -> Result<(), String> {
    let mut state = snapshot()?;
    state.queue.pending_owner_count = 1;
    let decision = reduce_with_event_id(&state, RuntimeEventId(8), RuntimeEvent::QueueChanged)
        .map_err(format_error)?;
    assert_eq!(decision.snapshot_id, RuntimeSnapshotId(1));
    assert_eq!(decision.event_id, RuntimeEventId(8));
    assert_eq!(decision.decision_id, RuntimeDecisionId::Pending);
    Ok(())
}

#[test]
fn event_catalog_contains_expected_closed_size() {
    assert_eq!(lkjagent_runtime::kernel::RuntimeEventKind::ALL.len(), 56);
}
