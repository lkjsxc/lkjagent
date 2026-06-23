use lkjagent_runtime::mode::{
    decide_turn_authority, policy_for_mode, ActiveMode, TurnAuthorityInput,
};

#[test]
fn preferred_action_never_points_at_blocked_tool() {
    for mode in [
        ActiveMode::OwnerTask,
        ActiveMode::Recovery,
        ActiveMode::Maintenance,
        ActiveMode::Compaction,
        ActiveMode::ClosedIdle,
    ] {
        let policy = policy_for_mode(mode);
        assert_eq!(policy.blocked_preferred_tool(), None, "mode={mode:?}");
    }
}

#[test]
fn tool_requiring_prompt_never_has_empty_tool_surface() {
    let owner = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        ..TurnAuthorityInput::default()
    });

    assert_ne!(owner.effective_policy.allowed_tools, Vec::<&str>::new());
    assert!(!owner.prompt_card.contains("allowed_tools=none"));
    assert!(owner.prompt_card.contains("allowed_tools=fs.read"));
    assert!(owner.prompt_card.contains("graph.state"));
}

#[test]
fn valid_example_never_renders_blocked_tool() {
    let cases = [
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
    ];
    for input in cases {
        let authority = decide_turn_authority(input);
        for blocked in &authority.effective_policy.blocked_tools {
            assert!(!authority
                .valid_example
                .contains(&format!("<tool>{blocked}</tool>")));
        }
    }
}
