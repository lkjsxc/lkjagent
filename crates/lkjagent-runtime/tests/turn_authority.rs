use lkjagent_protocol::parse_completion;
use lkjagent_runtime::mode::{
    decide_turn_authority, render_turn_authority, ActiveMode, CompletionPolicy, EndpointDecision,
    TurnAuthorityInput,
};

#[test]
fn pending_owner_row_beats_active_maintenance() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        pending_owner_rows: 1,
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
fn pending_owner_row_beats_due_maintenance() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        pending_owner_rows: 1,
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::OwnerTask);
    assert_eq!(authority.endpoint_decision, EndpointDecision::DeliverOwner);
}

#[test]
fn recoverable_owner_case_beats_maintenance() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::Recovery);
    assert_eq!(authority.endpoint_decision, EndpointDecision::CallModel);
    assert!(authority.effective_policy.graph_policy_applies);
}

#[test]
fn active_owner_case_beats_maintenance() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.mode, ActiveMode::OwnerTask);
    assert_eq!(authority.endpoint_decision, EndpointDecision::CallModel);
}

#[test]
fn compaction_waits_for_owner_work_to_clear() {
    let owner = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        compaction_required: true,
        ..TurnAuthorityInput::default()
    });
    let compaction = decide_turn_authority(TurnAuthorityInput {
        compaction_required: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(owner.mode, ActiveMode::OwnerTask);
    assert_eq!(compaction.mode, ActiveMode::Compaction);
    assert_eq!(
        compaction.endpoint_decision,
        EndpointDecision::RuntimeCompact
    );
}

#[test]
fn maintenance_only_runs_without_owner_work() {
    let maintenance = decide_turn_authority(TurnAuthorityInput {
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });
    let recovery = decide_turn_authority(TurnAuthorityInput {
        recoverable_owner_case: true,
        maintenance_due: true,
        ..TurnAuthorityInput::default()
    });

    assert_eq!(maintenance.mode, ActiveMode::Maintenance);
    assert_eq!(maintenance.endpoint_decision, EndpointDecision::CallModel);
    assert!(!maintenance.effective_policy.graph_policy_applies);
    assert_eq!(recovery.mode, ActiveMode::Recovery);
}

#[test]
fn closed_idle_has_no_endpoint_action() {
    let authority = decide_turn_authority(TurnAuthorityInput::default());

    assert_eq!(authority.mode, ActiveMode::ClosedIdle);
    assert_eq!(authority.endpoint_decision, EndpointDecision::ClosedIdle);
    assert!(matches!(
        authority.completion_policy,
        CompletionPolicy::ClosedIdle
    ));
}

#[test]
fn compaction_blocks_model_authored_memory_save() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        compaction_required: true,
        ..TurnAuthorityInput::default()
    });

    assert!(authority
        .effective_policy
        .blocked_tools
        .contains(&"memory.save"));
    assert!(matches!(
        authority.completion_policy,
        CompletionPolicy::Compaction(_)
    ));
    assert!(!authority.prompt_card.contains("<tool>memory.save</tool>"));
}

#[test]
fn policy_layers_are_exclusive() {
    for input in [
        TurnAuthorityInput {
            active_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            recoverable_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            maintenance_due: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            compaction_required: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput::default(),
    ] {
        let card = decide_turn_authority(input).prompt_card;
        assert!(!card.contains("policy_layers=graph,maintenance"));
        assert!(!card.contains("policy_layers=graph,compaction"));
    }
}

#[test]
fn rendered_model_examples_parse_for_call_model_modes() {
    for input in [
        TurnAuthorityInput {
            active_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            recoverable_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            maintenance_due: true,
            ..TurnAuthorityInput::default()
        },
    ] {
        let authority = decide_turn_authority(input);
        assert_eq!(authority.endpoint_decision, EndpointDecision::CallModel);
        assert!(parse_completion(&authority.valid_example).is_ok());
    }
}

#[test]
fn authority_card_renders_once_and_matches_dispatch_mode() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        ..TurnAuthorityInput::default()
    });
    let card = render_turn_authority(&authority);

    assert_eq!(card.matches("Active Mode:").count(), 1);
    assert!(card.contains("mode=OwnerTask"));
    assert!(authority.dispatch_card.contains("active_mode=OwnerTask"));
}
