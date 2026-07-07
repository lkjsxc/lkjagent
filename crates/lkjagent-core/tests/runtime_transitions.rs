use lkjagent_core::runtime_state_edge::{EdgeEvidenceRef, StateEdge, StateEdgeRelation, StateRef};
use lkjagent_core::runtime_transition::{
    validate_transition, ActorKind, NodeKind, NodeState, RuntimeTransition,
};

#[test]
fn accepts_normal_progression_with_verification_evidence() {
    let transition = base(NodeState::Verifying, NodeState::Succeeded)
        .with_evidence(vec![EdgeEvidenceRef::new("check", "check-1", "fp")]);

    let guard = validate_transition(&transition, &[]);

    assert!(guard.allowed, "{}", guard.reason);
}

#[test]
fn rejects_success_without_fresh_evidence() {
    let transition = base(NodeState::Verifying, NodeState::Succeeded);

    let guard = validate_transition(&transition, &[]);

    assert!(!guard.allowed);
    assert_eq!(guard.reason, "verification evidence is required");
}

#[test]
fn active_blocker_edge_prevents_ready_or_active_progress() {
    let transition = base(NodeState::Admitted, NodeState::Ready);
    let blocker = StateEdge::active(
        "edge-1",
        "case-1",
        StateRef::new("policy", "owner-approval"),
        transition.node_ref.clone(),
        StateEdgeRelation::blocks(),
        "event-1",
    );

    let guard = validate_transition(&transition, &[blocker]);

    assert!(!guard.allowed);
    assert_eq!(guard.reason, "blocked by edge edge-1");
}

#[test]
fn dependency_edge_prevents_dependent_progress() {
    let transition = base(NodeState::Ready, NodeState::Active);
    let dependency = StateEdge::active(
        "edge-dep",
        "case-1",
        transition.node_ref.clone(),
        StateRef::new("artifact", "input-report"),
        StateEdgeRelation::depends_on(),
        "event-1",
    );

    let guard = validate_transition(&transition, &[dependency]);

    assert!(!guard.allowed);
    assert_eq!(guard.reason, "blocked by edge edge-dep");
}

#[test]
fn recovery_is_legal_but_terminal_states_do_not_reopen() {
    let recovery = base(NodeState::Active, NodeState::Recovering);
    assert!(validate_transition(&recovery, &[]).allowed);

    let reopened = base(NodeState::Succeeded, NodeState::Active);
    let guard = validate_transition(&reopened, &[]);

    assert!(!guard.allowed);
    assert_eq!(guard.reason, "terminal state cannot transition");
}

fn base(previous_state: NodeState, next_state: NodeState) -> RuntimeTransition {
    RuntimeTransition::new(
        "transition-1",
        StateRef::new("task", "task-1"),
        NodeKind::Task,
        previous_state,
        next_state,
        ActorKind::Harness,
    )
}
