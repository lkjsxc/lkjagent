use serde::{Deserialize, Serialize};

use crate::runtime_state_edge::{EdgeEvidenceRef, StateEdge, StateEdgeRelation, StateRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Proposed,
    Admitted,
    Ready,
    Active,
    WaitingOwner,
    WaitingExternal,
    Blocked,
    Recovering,
    Verifying,
    Succeeded,
    Failed,
    Superseded,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    Task,
    Decision,
    Artifact,
    WorkspaceObject,
    Resource,
    AgentRun,
    Observation,
    Policy,
    Plan,
    Experiment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorKind {
    Owner,
    Model,
    Scheduler,
    Tool,
    Harness,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTransition {
    pub id: String,
    pub node_ref: StateRef,
    pub node_kind: NodeKind,
    pub previous_state: NodeState,
    pub next_state: NodeState,
    pub actor_kind: ActorKind,
    pub actor_id: Option<String>,
    pub reason_code: String,
    pub summary: String,
    pub evidence_refs: Vec<EdgeEvidenceRef>,
    pub context_frame_fingerprint: Option<String>,
    pub tool_call_id: Option<String>,
    pub retry_count: u32,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionGuard {
    pub allowed: bool,
    pub reason: String,
}

impl RuntimeTransition {
    pub fn new(
        id: impl Into<String>,
        node_ref: StateRef,
        node_kind: NodeKind,
        previous_state: NodeState,
        next_state: NodeState,
        actor_kind: ActorKind,
    ) -> Self {
        Self {
            id: id.into(),
            node_ref,
            node_kind,
            previous_state,
            next_state,
            actor_kind,
            actor_id: None,
            reason_code: String::new(),
            summary: String::new(),
            evidence_refs: Vec::new(),
            context_frame_fingerprint: None,
            tool_call_id: None,
            retry_count: 0,
            correlation_id: String::new(),
        }
    }

    pub fn with_evidence(mut self, evidence_refs: Vec<EdgeEvidenceRef>) -> Self {
        self.evidence_refs = evidence_refs;
        self
    }
}

pub fn validate_transition(
    transition: &RuntimeTransition,
    active_edges: &[StateEdge],
) -> TransitionGuard {
    if is_terminal(transition.previous_state) && transition.previous_state != transition.next_state
    {
        return reject("terminal state cannot transition");
    }
    if !is_legal_state_step(transition.previous_state, transition.next_state) {
        return reject("state step is not legal");
    }
    if requires_evidence(transition.next_state) && transition.evidence_refs.is_empty() {
        return reject("verification evidence is required");
    }
    if wants_progress(transition.next_state) {
        if let Some(edge) = blocking_edge(&transition.node_ref, active_edges) {
            return reject(format!("blocked by edge {}", edge.id));
        }
    }
    allow("transition accepted")
}

fn allow(reason: impl Into<String>) -> TransitionGuard {
    TransitionGuard {
        allowed: true,
        reason: reason.into(),
    }
}

fn reject(reason: impl Into<String>) -> TransitionGuard {
    TransitionGuard {
        allowed: false,
        reason: reason.into(),
    }
}

#[rustfmt::skip]
fn is_legal_state_step(previous: NodeState, next: NodeState) -> bool {
    previous == next || matches!((previous, next),
        (NodeState::Proposed, NodeState::Admitted | NodeState::Archived)
        | (NodeState::Admitted, NodeState::Ready | NodeState::Blocked | NodeState::Archived)
        | (NodeState::Ready, NodeState::Active | NodeState::Blocked | NodeState::WaitingExternal)
        | (NodeState::Active, NodeState::WaitingOwner | NodeState::WaitingExternal)
        | (NodeState::Active, NodeState::Blocked | NodeState::Recovering | NodeState::Verifying)
        | (NodeState::WaitingOwner | NodeState::WaitingExternal, NodeState::Ready)
        | (NodeState::Blocked, NodeState::Ready | NodeState::Recovering | NodeState::Failed)
        | (NodeState::Recovering, NodeState::Ready | NodeState::Active | NodeState::Failed)
        | (NodeState::Verifying, NodeState::Succeeded | NodeState::Failed | NodeState::Recovering)
        | (_, NodeState::Superseded))
}

fn requires_evidence(next: NodeState) -> bool {
    matches!(
        next,
        NodeState::Succeeded | NodeState::Failed | NodeState::Superseded
    )
}

fn wants_progress(next: NodeState) -> bool {
    matches!(
        next,
        NodeState::Ready | NodeState::Active | NodeState::Verifying
    )
}

fn is_terminal(state: NodeState) -> bool {
    matches!(
        state,
        NodeState::Succeeded | NodeState::Failed | NodeState::Superseded | NodeState::Archived
    )
}

fn blocking_edge<'a>(node_ref: &StateRef, active_edges: &'a [StateEdge]) -> Option<&'a StateEdge> {
    active_edges.iter().find(|edge| {
        let relation = &edge.relation;
        (relation == &StateEdgeRelation::blocks() && edge.to_ref == *node_ref)
            || (relation == &StateEdgeRelation::depends_on() && edge.from_ref == *node_ref)
    })
}
