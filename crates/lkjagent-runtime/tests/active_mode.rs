use lkjagent_runtime::mode::{
    policy_for_mode, render_mode_policy, select_active_mode, ActiveMode, ActiveModeInput,
};

#[test]
fn owner_queue_preempts_maintenance_before_endpoint_turn() {
    let mode = select_active_mode(ActiveModeInput {
        pending_owner_rows: 1,
        maintenance_active: true,
        maintenance_due: true,
        ..ActiveModeInput::default()
    });

    assert_eq!(mode, ActiveMode::OwnerTask);
}

#[test]
fn active_owner_case_blocks_new_maintenance_cycle() {
    let mode = select_active_mode(ActiveModeInput {
        active_owner_case: true,
        maintenance_due: true,
        ..ActiveModeInput::default()
    });

    assert_eq!(mode, ActiveMode::OwnerTask);
}

#[test]
fn recovery_owner_case_blocks_maintenance_cycle() {
    let mode = select_active_mode(ActiveModeInput {
        recoverable_owner_case: true,
        maintenance_due: true,
        ..ActiveModeInput::default()
    });

    assert_eq!(mode, ActiveMode::Recovery);
}

#[test]
fn closed_case_allows_maintenance_only_after_cooldown() {
    let idle = select_active_mode(ActiveModeInput::default());
    let due = select_active_mode(ActiveModeInput {
        maintenance_due: true,
        ..ActiveModeInput::default()
    });

    assert_eq!(idle, ActiveMode::ClosedIdle);
    assert_eq!(due, ActiveMode::Maintenance);
}

#[test]
fn maintenance_mode_does_not_render_graph_policy_refusals() {
    let rendered = render_mode_policy(&policy_for_mode(ActiveMode::Maintenance));

    assert!(rendered.contains("policy_layers=maintenance"));
    assert!(!rendered.contains(
        "allowed_tools=memory.find,memory.prune,memory.save,queue.list,agent.done,agent.ask"
    ));
    assert!(rendered.contains("blocked_tools="));
    assert!(rendered.contains("agent.ask"));
    assert!(!rendered.contains("graph policy refused"));
    assert!(!rendered.contains("policy_layers=graph,maintenance"));
}

#[test]
fn compaction_mode_does_not_render_graph_policy_refusals() {
    let rendered = render_mode_policy(&policy_for_mode(ActiveMode::Compaction));

    assert!(rendered.contains("policy_layers=compaction"));
    assert!(rendered.contains("runtime-owned compaction snapshot"));
    assert!(!rendered.contains("graph policy refused"));
}

#[test]
fn maintenance_policy_and_graph_policy_never_render_together() {
    for mode in [
        ActiveMode::OwnerTask,
        ActiveMode::Recovery,
        ActiveMode::Maintenance,
        ActiveMode::Compaction,
        ActiveMode::ClosedIdle,
    ] {
        let rendered = render_mode_policy(&policy_for_mode(mode));
        assert!(!rendered.contains("policy_layers=graph,maintenance"));
        assert!(!rendered.contains("policy_layers=graph,compaction"));
    }
}
