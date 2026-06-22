use lkjagent_runtime::mode::{
    admit_tool, decide, recovery_plan_for_fault, ActiveMode, FaultClass, RecoveryClass,
    RuntimeDecision, RuntimeEvent, RuntimeFault, RuntimeSnapshot,
};

#[test]
fn turn_budget_exhaustion_selects_blocked_handoff_plan() {
    let snapshot = recovery_snapshot();

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetExhausted);

    assert!(matches!(decision, RuntimeDecision::StartRecovery(_)));
    let RuntimeDecision::StartRecovery(plan) = decision else {
        return;
    };
    assert_eq!(plan.fault_class, FaultClass::Budget);
    assert_eq!(plan.recovery_class, RecoveryClass::TurnBudgetExhaustion);
    assert!(plan.partial_handoff);
    assert_eq!(plan.forced_tool, "runtime.handoff");
}

#[test]
fn verification_recovery_forced_tool_is_admitted() {
    let snapshot = recovery_snapshot();
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::VerificationMismatch);

    let admission = admit_tool(&snapshot, &plan.forced_tool);

    assert_eq!(plan.forced_tool, "verify.xtask");
    assert!(admission.admitted);
}

#[test]
fn maintenance_preemption_recovery_forced_tool_is_admitted() {
    let snapshot = recovery_snapshot();
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::MaintenanceConflict);

    let admission = admit_tool(&snapshot, &plan.forced_tool);

    assert_eq!(plan.forced_tool, "queue.list");
    assert!(admission.admitted);
}

fn recovery_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mission: ActiveMode::Recovery,
        owner_work_exists: true,
        recovery_ladder_active: true,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string()],
        missing_evidence: vec!["artifact-readiness".to_string()],
        active_artifact: Some("dictionary/bread-terms.txt".to_string()),
        last_tool_attempt: Some("fs.write".to_string()),
        repeated_action: false,
        external_owner_input_required: false,
    }
}
