use lkjagent_runtime::mode::{
    recovery_plan_for_fault, ActiveMode, RecoveryClass, RuntimeFault, RuntimeSnapshot,
};

#[test]
fn repeated_batch_schema_fault_changes_to_artifact_next() {
    let snapshot = recovery_snapshot("fs.batch_write", true);
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::Schema);

    assert_eq!(plan.recovery_class, RecoveryClass::SchemaFault);
    assert_eq!(plan.forced_tool, "artifact.next");
    assert!(plan
        .allowed_repair_tools
        .iter()
        .any(|tool| tool == "fs.batch_write"));
    assert!(plan
        .allowed_repair_tools
        .iter()
        .any(|tool| tool == "artifact.next"));
}

#[test]
fn first_batch_schema_fault_keeps_exact_schema_example() {
    let snapshot = recovery_snapshot("fs.batch_write", false);
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::Schema);

    assert_eq!(plan.forced_tool, "fs.batch_write");
    assert!(plan
        .exact_valid_example
        .contains("<tool>fs.batch_write</tool>"));
}

#[test]
fn repeated_graph_recover_changes_action_class() {
    let snapshot = recovery_snapshot("graph.recover", true);
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::Repeat);

    assert_ne!(plan.forced_tool, "graph.recover");
    assert_eq!(plan.recovery_class, RecoveryClass::RepeatActionFault);
    assert_eq!(plan.retry_budget, 0);
}

#[test]
fn oversized_batch_with_artifact_routes_to_artifact_next() {
    let snapshot = recovery_snapshot("fs.batch_write", false);
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::PayloadTooLarge);

    assert_eq!(plan.recovery_class, RecoveryClass::PayloadOverflow);
    assert_eq!(plan.forced_tool, "artifact.next");
    assert!(plan
        .exact_valid_example
        .contains("<tool>artifact.next</tool>"));
    assert!(plan
        .exact_valid_example
        .contains("<root>stories/chronos-fracture</root>"));
}

#[test]
fn oversized_batch_without_artifact_keeps_bounded_batch_route() {
    let mut snapshot = recovery_snapshot("fs.batch_write", false);
    snapshot.active_artifact = None;
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::PayloadTooLarge);

    assert_eq!(plan.forced_tool, "fs.batch_write");
    assert!(plan
        .exact_valid_example
        .contains("<tool>fs.batch_write</tool>"));
}

fn recovery_snapshot(last_tool: &str, repeated_action: bool) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: ActiveMode::Recovery,
        case_id: Some("case-17".to_string()),
        graph_node: Some("document".to_string()),
        graph_phase: Some("recovery".to_string()),
        owner_work_exists: true,
        recovery_ladder_active: true,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string()],
        missing_evidence: vec!["artifact-readiness".to_string()],
        active_artifact: Some("stories/chronos-fracture".to_string()),
        last_tool_attempt: Some(last_tool.to_string()),
        latest_fault: Some(RuntimeFault::Schema),
        repeated_action,
        external_owner_input_required: false,
    }
}
