use lkjagent_runtime::mode::{
    decide, ActiveMode, RecoveryClass, RuntimeDecision, RuntimeEvent, RuntimeSnapshot,
};

#[test]
fn checkpoint_with_actionable_work_continues_owner_execution() {
    let snapshot = owner_snapshot(vec!["observation"]);

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetCheckpoint);

    assert_eq!(decision, RuntimeDecision::AskEndpoint);
}

#[test]
fn checkpoint_with_repeated_fault_enters_recovery() {
    let mut snapshot = owner_snapshot(vec!["artifact-readiness"]);
    snapshot.active_mission = ActiveMode::Recovery;
    snapshot.recovery_ladder_active = true;
    snapshot.repeated_action = true;
    snapshot.last_tool_attempt = Some("graph.next".to_string());

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetCheckpoint);

    assert!(matches!(decision, RuntimeDecision::StartRecovery(_)));
    let RuntimeDecision::StartRecovery(plan) = decision else {
        return;
    };
    assert_eq!(plan.recovery_class, RecoveryClass::RepeatActionFault);
    assert_ne!(plan.forced_tool, "graph.next");
}

#[test]
fn checkpoint_does_not_start_maintenance_while_owner_work_exists() {
    let snapshot = owner_snapshot(vec!["verification"]);

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetCheckpoint);

    assert_ne!(decision, RuntimeDecision::StartMaintenance);
}

fn owner_snapshot(missing: Vec<&str>) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mission: ActiveMode::OwnerTask,
        case_id: None,
        graph_node: None,
        graph_phase: None,
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: true,
        required_evidence: vec!["observation".to_string(), "verification".to_string()],
        missing_evidence: missing.into_iter().map(str::to_string).collect(),
        active_artifact: Some("cookbook/current".to_string()),
        last_tool_attempt: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}
