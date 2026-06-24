use lkjagent_runtime::mode::{
    admit_tool, decide, ActiveMode, RuntimeDecision, RuntimeEvent, RuntimeSnapshot,
};
use lkjagent_tools::dispatch::registry_valid_example;

#[test]
fn reducer_executes_done_only_after_required_evidence_is_present() {
    let snapshot = owner_snapshot(&[]);

    let decision = decide(
        &snapshot,
        RuntimeEvent::EndpointActionParsed {
            tool: "agent.done".to_string(),
        },
    );

    assert!(matches!(decision, RuntimeDecision::ExecuteTool(_)));
    let RuntimeDecision::ExecuteTool(admission) = decision else {
        return;
    };
    assert!(admission.admitted);
    assert_eq!(
        admission.exact_valid_example.as_deref(),
        registry_valid_example("agent.done").as_deref()
    );
}

#[test]
fn reducer_closes_completion_request_only_after_required_evidence_is_present() {
    let snapshot = owner_snapshot(&[]);

    let decision = decide(&snapshot, RuntimeEvent::CompletionRequested);

    assert_eq!(decision, RuntimeDecision::CloseCase);
}

#[test]
fn reducer_refusal_keeps_audit_and_repair_tools_admitted() {
    let snapshot = owner_snapshot(&["artifact-readiness", "verification"]);

    let admission = admit_tool(&snapshot, "agent.done");

    assert!(!admission.admitted);
    for tool in [
        "artifact.audit",
        "artifact.next",
        "doc.audit",
        "fs.batch_write",
    ] {
        assert!(
            admission
                .next_valid_tools
                .iter()
                .any(|candidate| candidate == tool),
            "missing {tool}"
        );
    }
}

#[test]
fn reducer_refuses_done_after_planning_only() {
    let snapshot = owner_snapshot(&["observation", "verification"]);

    let decision = decide(
        &snapshot,
        RuntimeEvent::EndpointActionParsed {
            tool: "agent.done".to_string(),
        },
    );

    assert!(matches!(decision, RuntimeDecision::BlockCompletion(_)));
    let RuntimeDecision::BlockCompletion(admission) = decision else {
        return;
    };
    assert!(!admission.admitted);
    assert!(admission
        .missing_evidence
        .contains(&"observation".to_string()));
    assert!(admission.exact_valid_example.is_some());
}

#[test]
fn maintenance_done_stays_admitted_when_owner_work_is_absent() {
    let mut snapshot = owner_snapshot(&[]);
    snapshot.active_mode = ActiveMode::Maintenance;
    snapshot.owner_work_exists = false;

    let decision = decide(
        &snapshot,
        RuntimeEvent::EndpointActionParsed {
            tool: "agent.done".to_string(),
        },
    );

    assert!(matches!(decision, RuntimeDecision::ExecuteTool(_)));
}

fn owner_snapshot(missing: &[&str]) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: ActiveMode::OwnerTask,
        case_id: None,
        graph_node: None,
        graph_phase: None,
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec![
            "plan".to_string(),
            "observation".to_string(),
            "verification".to_string(),
        ],
        missing_evidence: missing.iter().map(|value| (*value).to_string()).collect(),
        active_artifact: Some("story/current".to_string()),
        last_tool_attempt: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}
