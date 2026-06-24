use lkjagent_runtime::mode::{
    admit_tool, decide_completion, ActiveMode, CompletionKind, RuntimeSnapshot,
};

#[test]
fn owner_completion_lists_failed_gates_and_next_repair_action() {
    let snapshot = owner_snapshot(&["artifact-readiness", "verification"]);

    let decision = decide_completion(&snapshot);
    let admission = admit_tool(&snapshot, "agent.done");

    assert!(!decision.allowed);
    assert_eq!(decision.completion_kind, CompletionKind::OwnerTask);
    assert_eq!(decision.missing_evidence, admission.missing_evidence);
    assert!(decision
        .failed_gates
        .contains(&"artifact-readiness".to_string()));
    assert_eq!(decision.next_executable_action, "verify.xtask");
    assert!(decision.valid_example.contains("verify.xtask"));
}

#[test]
fn owner_completion_allows_done_only_when_gates_are_clear() {
    let snapshot = owner_snapshot(&[]);

    let decision = decide_completion(&snapshot);
    let admission = admit_tool(&snapshot, "agent.done");

    assert!(decision.allowed);
    assert!(admission.admitted);
    assert_eq!(decision.next_executable_action, "agent.done");
    assert_eq!(decision.status_text, "completion admitted");
}

#[test]
fn maintenance_completion_is_separate_from_owner_completion() {
    let mut snapshot = owner_snapshot(&[]);
    snapshot.active_mode = ActiveMode::Maintenance;
    snapshot.owner_work_exists = false;

    let decision = decide_completion(&snapshot);

    assert!(decision.allowed);
    assert_eq!(decision.completion_kind, CompletionKind::Maintenance);
}

#[test]
fn compaction_blocks_model_completion() {
    let mut snapshot = owner_snapshot(&[]);
    snapshot.active_mode = ActiveMode::Compaction;
    snapshot.context_pressure_active = true;

    let decision = decide_completion(&snapshot);

    assert!(!decision.allowed);
    assert_eq!(decision.completion_kind, CompletionKind::RuntimeOnly);
    assert_eq!(decision.next_executable_action, "runtime.compact");
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
            "artifact-readiness".to_string(),
            "verification".to_string(),
        ],
        missing_evidence: missing.iter().map(|value| (*value).to_string()).collect(),
        active_artifact: Some("cookbook/current".to_string()),
        last_tool_attempt: None,
        latest_fault: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}
