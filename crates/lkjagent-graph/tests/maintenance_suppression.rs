use lkjagent_graph::maintenance::{no_op_suppression, MaintenanceCycleObservation};

#[test]
fn maintenance_noop_creates_suppression_key() -> Result<(), Box<dyn std::error::Error>> {
    let observation = MaintenanceCycleObservation {
        found_only_known_lessons: true,
        listed_same_queue_state: true,
        pruned_rows: 0,
        changed_paths: Vec::new(),
        new_structural_findings: Vec::new(),
        repeated_action_signatures: vec!["memory.find:old".to_string()],
    };

    let Some(record) = no_op_suppression(&observation) else {
        return Err("missing suppression".into());
    };
    assert_eq!(record.key, "maintenance:distill:no-new-evidence");
    assert_eq!(record.expires_after_cycles, 3);
    Ok(())
}

#[test]
fn maintenance_effect_does_not_create_noop_suppression() {
    let observation = MaintenanceCycleObservation {
        found_only_known_lessons: true,
        listed_same_queue_state: true,
        pruned_rows: 1,
        changed_paths: Vec::new(),
        new_structural_findings: Vec::new(),
        repeated_action_signatures: Vec::new(),
    };

    assert!(no_op_suppression(&observation).is_none());
}
