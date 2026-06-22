use lkjagent_runtime::mode::{admit_tool, ActiveMode, RuntimeSnapshot};

#[test]
fn owner_question_is_blocked_without_external_input_gate() {
    let snapshot = snapshot(false);

    let admission = admit_tool(&snapshot, "agent.ask");

    assert!(!admission.admitted);
    assert!(admission.reason.contains("external missing input"));
    assert!(!admission
        .next_valid_tools
        .contains(&"agent.ask".to_string()));
}

#[test]
fn owner_question_is_admitted_only_with_external_input_gate() {
    let snapshot = snapshot(true);

    let admission = admit_tool(&snapshot, "agent.ask");

    assert!(admission.admitted);
    assert!(admission
        .next_valid_tools
        .contains(&"agent.ask".to_string()));
}

fn snapshot(external_owner_input_required: bool) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mission: ActiveMode::OwnerTask,
        owner_work_exists: true,
        recovery_ladder_active: false,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["owner-preference".to_string()],
        missing_evidence: vec!["owner-preference".to_string()],
        active_artifact: Some("cookbook".to_string()),
        last_tool_attempt: None,
        repeated_action: false,
        external_owner_input_required,
    }
}
