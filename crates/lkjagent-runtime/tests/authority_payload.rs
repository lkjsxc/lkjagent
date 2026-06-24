use lkjagent_runtime::mode::{admit_tool, ActiveMode, RuntimeFault, RuntimeSnapshot};

#[test]
fn payload_fault_blocks_raw_write_retry_and_keeps_batch_route() {
    let mut snapshot = recovery_snapshot();
    snapshot.latest_fault = Some(RuntimeFault::PayloadTooLarge);

    let write = admit_tool(&snapshot, "fs.write");
    let batch = admit_tool(&snapshot, "fs.batch_write");

    assert!(!write.admitted);
    assert!(batch.admitted);
    assert!(write
        .next_valid_tools
        .contains(&"artifact.next".to_string()));
    assert!(write
        .next_valid_tools
        .contains(&"fs.batch_write".to_string()));
    assert!(!write.next_valid_tools.contains(&"fs.write".to_string()));
}

fn recovery_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: ActiveMode::Recovery,
        case_id: Some("1".to_string()),
        graph_node: Some("recover-by-bounded-write".to_string()),
        graph_phase: Some("recovery".to_string()),
        owner_work_exists: true,
        recovery_ladder_active: true,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string()],
        missing_evidence: vec!["artifact-readiness".to_string()],
        active_artifact: Some("stories/sf".to_string()),
        last_tool_attempt: Some("fs.write".to_string()),
        latest_fault: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}
