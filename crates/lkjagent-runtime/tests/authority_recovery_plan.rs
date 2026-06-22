use lkjagent_runtime::mode::{
    decide, recovery_plan_for_fault, ActiveMode, RecoveryClass, RuntimeDecision, RuntimeEvent,
    RuntimeFault, RuntimeSnapshot,
};

#[test]
fn recovery_fault_table_names_escape_route_for_each_class() {
    let snapshot = recovery_snapshot();
    let cases = [
        (
            RuntimeFault::Parse,
            RecoveryClass::ParseFault,
            "graph.recover",
            false,
        ),
        (
            RuntimeFault::Parameter,
            RecoveryClass::ParameterFault,
            "fs.write",
            false,
        ),
        (
            RuntimeFault::Schema,
            RecoveryClass::SchemaFault,
            "fs.write",
            false,
        ),
        (
            RuntimeFault::PolicyContradiction,
            RecoveryClass::ToolAdmissionContradiction,
            "graph.recover",
            false,
        ),
        (
            RuntimeFault::Repeat,
            RecoveryClass::RepeatActionFault,
            "graph.recover",
            false,
        ),
        (
            RuntimeFault::PayloadTooLarge,
            RecoveryClass::PayloadOverflow,
            "fs.batch_write",
            false,
        ),
        (
            RuntimeFault::ArtifactAuditFailure,
            RecoveryClass::ArtifactAuditFailure,
            "artifact.next",
            false,
        ),
        (
            RuntimeFault::WeakArtifactContent,
            RecoveryClass::WeakArtifactContent,
            "artifact.next",
            false,
        ),
        (
            RuntimeFault::FalseCompletion,
            RecoveryClass::FalseCompletion,
            "artifact.audit",
            false,
        ),
        (
            RuntimeFault::VerificationMismatch,
            RecoveryClass::VerificationFailure,
            "verify.xtask",
            false,
        ),
        (
            RuntimeFault::CompactionResumeGap,
            RecoveryClass::CompactionResumeGap,
            "runtime.compact",
            true,
        ),
        (
            RuntimeFault::MaintenanceConflict,
            RecoveryClass::MaintenancePreemption,
            "queue.list",
            false,
        ),
        (
            RuntimeFault::EndpointFault,
            RecoveryClass::EndpointFault,
            "workspace.summary",
            true,
        ),
        (
            RuntimeFault::TurnBudgetExhausted,
            RecoveryClass::TurnBudgetExhaustion,
            "runtime.handoff",
            true,
        ),
    ];

    for (fault, class, tool, partial_handoff) in cases {
        let plan = recovery_plan_for_fault(&snapshot, fault);
        assert_eq!(plan.recovery_class, class);
        assert_eq!(plan.forced_tool, tool);
        assert_eq!(plan.partial_handoff, partial_handoff);
        assert!(!plan.exact_valid_example.is_empty());
        assert!(!plan.forced_next_action.is_empty());
        assert!(!plan.fallback_action.is_empty());
        assert!(!plan.allowed_observation_tools.is_empty());
        assert!(!plan.allowed_repair_tools.is_empty());
    }
}

#[test]
fn turn_budget_exhaustion_selects_blocked_handoff_plan() {
    let snapshot = recovery_snapshot();

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetExhausted);

    assert!(matches!(decision, RuntimeDecision::StartRecovery(_)));
    let RuntimeDecision::StartRecovery(plan) = decision else {
        return;
    };
    assert_eq!(plan.recovery_class, RecoveryClass::TurnBudgetExhaustion);
    assert!(plan.partial_handoff);
    assert_eq!(plan.forced_tool, "runtime.handoff");
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
