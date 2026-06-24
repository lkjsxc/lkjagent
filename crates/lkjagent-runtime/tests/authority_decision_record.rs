use lkjagent_runtime::mode::{
    decide_record, ActiveMode, DecisionKind, RuntimeEvent, RuntimeMission, RuntimeSnapshot,
};

#[test]
fn decision_record_carries_mission_fingerprint_and_completion_refusal() {
    let snapshot = owner_snapshot(&["artifact-readiness"]);

    let record = decide_record(&snapshot, RuntimeEvent::CompletionRequested);

    assert_eq!(record.kind, DecisionKind::BlockCompletion);
    assert_eq!(record.mission, RuntimeMission::OwnerExecution);
    assert_eq!(record.active_mode, ActiveMode::OwnerTask);
    assert_eq!(record.event_kind, "completion_requested");
    assert_eq!(record.case_id, "42");
    assert_eq!(record.state_node, "audit");
    assert!(!record.case_id.contains("case:unknown"));
    assert!(record.decision_id.starts_with("decision-"));
    assert!(record.event_id.starts_with("event-"));
    assert_eq!(record.missing_evidence, vec!["artifact-readiness"]);
    assert!(!record.completion_allowed);
    assert!(record.completion_refusal.is_some());
    assert!(record.blocked_tools.contains(&"agent.done".to_string()));
    assert!(record.admitted_tools.contains(&"artifact.next".to_string()));
    assert!(!record.authority_fingerprint.0.is_empty());
}

fn owner_snapshot(missing: &[&str]) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: ActiveMode::OwnerTask,
        case_id: Some("42".to_string()),
        graph_node: Some("audit".to_string()),
        graph_phase: Some("verification".to_string()),
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string(), "verification".to_string()],
        missing_evidence: missing.iter().map(|value| (*value).to_string()).collect(),
        active_artifact: Some("dictionary/bread-terms.txt".to_string()),
        last_tool_attempt: None,
        latest_fault: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}
