use std::collections::BTreeSet;

use lkjagent_runtime::mode::{
    recovery_plan_for_fault, ActiveMode, FaultClass, RecoveryClass, RuntimeFault, RuntimeSnapshot,
};

#[test]
fn recovery_fault_table_names_escape_route_for_each_class() {
    use FaultClass as Fc;
    use RecoveryClass as Rc;
    use RuntimeFault::*;

    let snapshot = recovery_snapshot();
    let cases = [
        (Parse, Fc::Parse, Rc::ParseFault, "graph.recover"),
        (Parameter, Fc::Parameter, Rc::ParameterFault, "fs.write"),
        (Schema, Fc::Parameter, Rc::SchemaFault, "fs.write"),
        (
            PolicyContradiction,
            Fc::Tool,
            Rc::ToolAdmissionContradiction,
            "graph.recover",
        ),
        (
            ToolRuntime,
            Fc::Tool,
            Rc::EndpointFault,
            "workspace.summary",
        ),
        (Repeat, Fc::Repeat, Rc::RepeatActionFault, "graph.recover"),
        (
            EndpointFault,
            Fc::Endpoint,
            Rc::EndpointFault,
            "workspace.summary",
        ),
        (
            TurnBudgetExhausted,
            Fc::Budget,
            Rc::TurnBudgetExhaustion,
            "runtime.handoff",
        ),
        (
            ContextInvalid,
            Fc::Context,
            Rc::EndpointFault,
            "workspace.summary",
        ),
        (
            MaintenanceConflict,
            Fc::Context,
            Rc::MaintenancePreemption,
            "queue.list",
        ),
        (
            VerificationMismatch,
            Fc::Verification,
            Rc::VerificationFailure,
            "verify.xtask",
        ),
        (
            CompactionPressure,
            Fc::Compaction,
            Rc::CompactionResumeGap,
            "runtime.compact",
        ),
        (
            CompactionResumeGap,
            Fc::Compaction,
            Rc::CompactionResumeGap,
            "runtime.compact",
        ),
        (
            PayloadTooLarge,
            Fc::Payload,
            Rc::PayloadOverflow,
            "fs.batch_write",
        ),
        (
            ArtifactAuditFailure,
            Fc::Completion,
            Rc::ArtifactAuditFailure,
            "artifact.next",
        ),
        (
            WeakArtifactContent,
            Fc::Completion,
            Rc::WeakArtifactContent,
            "artifact.next",
        ),
        (
            FalseCompletion,
            Fc::Completion,
            Rc::FalseCompletion,
            "artifact.audit",
        ),
        (
            CompletionRefused,
            Fc::Completion,
            Rc::FalseCompletion,
            "artifact.audit",
        ),
    ];

    let mut covered = BTreeSet::new();
    for (fault, fault_class, recovery_class, tool) in cases {
        let plan = recovery_plan_for_fault(&snapshot, fault);
        assert_eq!(plan.fault_class, fault_class);
        assert_eq!(plan.recovery_class, recovery_class);
        assert_eq!(plan.forced_tool, tool);
        assert_eq!(plan.recovery_route, route_for(fault_class));
        assert!(!plan.escalation_route.is_empty());
        assert!(!plan.blocked_handoff_behavior.is_empty());
        assert!(!plan.exact_valid_example.is_empty());
        assert!(!plan.forced_next_action.is_empty());
        assert!(!plan.fallback_action.is_empty());
        assert!(!plan.allowed_observation_tools.is_empty());
        assert!(!plan.allowed_repair_tools.is_empty());
        covered.insert(plan.fault_class);
    }
    assert_eq!(covered, all_fault_classes());
}

fn route_for(class: FaultClass) -> &'static str {
    match class {
        FaultClass::Parse => "recover-parse",
        FaultClass::Parameter => "recover-params",
        FaultClass::Tool => "recover-tool",
        FaultClass::Repeat => "recover-repeat",
        FaultClass::Endpoint => "recover-endpoint",
        FaultClass::Budget => "recover-budget",
        FaultClass::Context => "recover-context",
        FaultClass::Verification => "recover-verification",
        FaultClass::Compaction => "recover-compaction",
        FaultClass::Payload => "recover-by-bounded-write",
        FaultClass::Completion => "recover-completion",
    }
}

fn all_fault_classes() -> BTreeSet<FaultClass> {
    [
        FaultClass::Parse,
        FaultClass::Parameter,
        FaultClass::Tool,
        FaultClass::Repeat,
        FaultClass::Endpoint,
        FaultClass::Budget,
        FaultClass::Context,
        FaultClass::Verification,
        FaultClass::Compaction,
        FaultClass::Payload,
        FaultClass::Completion,
    ]
    .into_iter()
    .collect()
}

fn recovery_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: ActiveMode::Recovery,
        case_id: None,
        graph_node: None,
        graph_phase: None,
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
