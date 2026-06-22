use lkjagent_runtime::mode::{
    admit_tool, decide, ActiveMode, RecoveryClass, RuntimeDecision, RuntimeEvent, RuntimeFault,
    RuntimeSnapshot,
};
use lkjagent_tools::dispatch::registry_valid_example;

#[test]
fn admit_tool_blocks_done_when_content_evidence_is_missing() {
    let snapshot = owner_snapshot(&["artifact-readiness", "verification"]);

    let admission = admit_tool(&snapshot, "agent.done");

    assert!(!admission.admitted);
    assert!(admission.reason.contains("completion missing"));
    assert_eq!(
        admission.exact_valid_example.as_deref(),
        registry_valid_example("fs.read").as_deref()
    );
    assert!(admission
        .next_valid_tools
        .contains(&"artifact.next".to_string()));
    assert!(admission
        .next_valid_tools
        .contains(&"fs.batch_write".to_string()));
}

#[test]
fn reducer_blocks_completion_with_explainable_admission() {
    let snapshot = owner_snapshot(&["artifact-readiness"]);

    let decision = decide(&snapshot, RuntimeEvent::CompletionRequested);

    assert!(matches!(decision, RuntimeDecision::BlockCompletion(_)));
    let RuntimeDecision::BlockCompletion(admission) = decision else {
        return;
    };
    assert_eq!(admission.active_mission, ActiveMode::OwnerTask);
    assert_eq!(admission.missing_evidence, vec!["artifact-readiness"]);
    assert!(admission.exact_valid_example.is_some());
}

#[test]
fn payload_too_large_routes_to_batch_write_recovery() {
    let snapshot = recovery_snapshot();

    let decision = decide(
        &snapshot,
        RuntimeEvent::ToolFailed {
            fault: RuntimeFault::PayloadTooLarge,
        },
    );

    assert!(matches!(decision, RuntimeDecision::ContinueRecovery { .. }));
    let RuntimeDecision::ContinueRecovery { plan, admission } = decision else {
        return;
    };
    assert_eq!(plan.recovery_class, RecoveryClass::PayloadOverflow);
    assert!(admission
        .next_valid_tools
        .contains(&"fs.batch_write".to_string()));
    assert!(admission.admitted);
    assert_eq!(
        admission.exact_valid_example.as_deref(),
        registry_valid_example("fs.batch_write").as_deref()
    );
}

#[test]
fn recovery_keeps_artifact_escape_tools_admitted() {
    let snapshot = recovery_snapshot();
    let tools = admit_tool(&snapshot, "artifact.next").next_valid_tools;

    for tool in [
        "artifact.next",
        "artifact.audit",
        "doc.audit",
        "doc.scaffold",
        "fs.read",
        "fs.tree",
        "fs.write",
        "fs.batch_write",
    ] {
        assert!(
            tools.iter().any(|candidate| candidate == tool),
            "missing {tool}"
        );
    }
}

#[test]
fn repeated_action_is_not_executed_again() {
    let mut snapshot = recovery_snapshot();
    snapshot.last_tool_attempt = Some("graph.recover".to_string());
    snapshot.repeated_action = true;

    let decision = decide(
        &snapshot,
        RuntimeEvent::EndpointActionParsed {
            tool: "graph.recover".to_string(),
        },
    );

    assert!(matches!(decision, RuntimeDecision::ContinueRecovery { .. }));
    let RuntimeDecision::ContinueRecovery { plan, admission } = decision else {
        return;
    };
    assert_eq!(plan.recovery_class, RecoveryClass::RepeatActionFault);
    assert!(!admission.admitted);
    assert!(admission.reason.contains("repeat action"));
    assert_ne!(
        admission.exact_valid_example.as_deref(),
        registry_valid_example("graph.recover").as_deref()
    );
}

#[test]
fn compaction_pressure_preempts_recovery_as_runtime_action() {
    let mut snapshot = recovery_snapshot();
    snapshot.context_pressure_active = true;

    let decision = decide(
        &snapshot,
        RuntimeEvent::EndpointActionParsed {
            tool: "graph.recover".to_string(),
        },
    );

    assert_eq!(decision, RuntimeDecision::StartCompaction);
}

#[test]
fn maintenance_tick_yields_when_owner_work_exists() {
    let snapshot = RuntimeSnapshot {
        active_mission: ActiveMode::OwnerTask,
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: true,
        required_evidence: Vec::new(),
        missing_evidence: Vec::new(),
        active_artifact: None,
        last_tool_attempt: None,
        repeated_action: false,
        external_owner_input_required: false,
    };

    assert_eq!(
        decide(&snapshot, RuntimeEvent::MaintenanceTick),
        RuntimeDecision::AskEndpoint
    );
}

fn owner_snapshot(missing: &[&str]) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mission: ActiveMode::OwnerTask,
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string(), "verification".to_string()],
        missing_evidence: missing.iter().map(|value| (*value).to_string()).collect(),
        active_artifact: Some("dictionary/bread-terms.txt".to_string()),
        last_tool_attempt: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
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
