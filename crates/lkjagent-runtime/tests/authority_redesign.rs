use lkjagent_runtime::mode::{
    decide_turn_authority, ActiveMode, CompletionPolicy, EndpointDecision, TurnAuthorityInput,
};

#[test]
fn owner_work_cannot_be_preempted_by_maintenance() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        pending_owner_rows: 1,
        active_owner_case: true,
        maintenance_active: true,
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::OwnerTask);
    assert_eq!(
        authority.endpoint_decision,
        EndpointDecision::DeferMaintenance
    );
}

#[test]
fn recovery_admits_artifact_escape_tools() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::Recovery);
    assert!(authority
        .effective_policy
        .allowed_tools
        .contains(&"artifact.next"));
    assert!(authority
        .effective_policy
        .allowed_tools
        .contains(&"fs.batch_write"));
}

#[test]
fn completion_gate_requires_artifact_readiness() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert!(matches!(
        authority.completion_policy,
        CompletionPolicy::OwnerTask(_)
    ));
    let CompletionPolicy::OwnerTask(gate) = authority.completion_policy else {
        return;
    };
    assert!(gate.requires_artifact_readiness);
    assert!(gate.requires_verification);
}

#[test]
fn completion_node_keeps_audit_and_repair_tools_visible() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    for tool in [
        "fs.read",
        "artifact.audit",
        "artifact.next",
        "fs.batch_write",
    ] {
        assert!(
            !authority.effective_policy.blocked_tools.contains(&tool),
            "{tool} must stay unblocked while artifact evidence is missing"
        );
    }
}

#[test]
fn hard_compaction_renders_resumable_recovery_snapshot_fields() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        compaction_required: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::Compaction);
    assert_eq!(
        authority.endpoint_decision,
        EndpointDecision::RuntimeCompact
    );
    for field in [
        "active_case",
        "missing_evidence",
        "active_artifact",
        "recovery_ladder",
        "next_valid_action",
    ] {
        assert!(authority.prompt_card.contains(field), "missing {field}");
    }
}

#[test]
fn repeated_invalid_actions_do_not_repeat_graph_state() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert_ne!(
        authority.valid_example, "<action>\n<tool>graph.state</tool>\n</action>",
        "recover-repeat needs a different exact next action"
    );
}

#[test]
fn payload_too_large_prefers_batch_planning() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert!(authority
        .effective_policy
        .preferred_next_action
        .contains("fs.batch_write"));
}
