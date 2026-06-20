use crate::guards::evaluate_guard;
use crate::model::{EdgeKind, GraphDefinition, GraphNodeId};
use crate::state::{TaskGraphState, TransitionDecision};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionLegality {
    Legal,
    Blocked,
    Illegal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionQuality {
    pub legality: TransitionLegality,
    pub reason: String,
    pub evidence_delta: i16,
    pub context_delta: i16,
    pub risk_delta: i16,
    pub repetition_penalty: i16,
    pub expected_next_observation: Option<String>,
}

pub fn legal_targets(graph: GraphDefinition, from: GraphNodeId) -> Vec<GraphNodeId> {
    let mut edges = graph
        .edges
        .iter()
        .filter(|edge| edge.from == from)
        .collect::<Vec<_>>();
    edges.sort_by_key(|edge| (edge.policy.priority, edge.id));
    edges.into_iter().map(|edge| edge.to).collect()
}

pub fn admitted_targets(graph: &GraphDefinition, state: &TaskGraphState) -> Vec<GraphNodeId> {
    let mut edges = graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .filter(|edge| {
            edge.guards
                .iter()
                .all(|guard| evaluate_guard(*guard, graph, state).is_ok())
        })
        .collect::<Vec<_>>();
    edges.sort_by_key(|edge| (edge.policy.priority, edge.id));
    edges.into_iter().map(|edge| edge.to).collect()
}

pub fn admit_transition(
    graph: GraphDefinition,
    state: &TaskGraphState,
    target: GraphNodeId,
) -> TransitionDecision {
    let Some(edge) = graph
        .edges
        .iter()
        .find(|edge| edge.from == state.active_node && edge.to == target)
    else {
        return TransitionDecision::Refuse {
            reason: format!("illegal transition {} -> {}", state.active_node.0, target.0),
        };
    };
    let missing = edge
        .guards
        .iter()
        .filter_map(|guard| evaluate_guard(*guard, &graph, state).err())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        if target == GraphNodeId("recover") {
            return TransitionDecision::Recover {
                reason: missing.join(", "),
                target,
            };
        }
        return TransitionDecision::Defer { missing };
    }
    TransitionDecision::Admit { target }
}

pub fn transition_quality(
    graph: &GraphDefinition,
    state: &TaskGraphState,
    target: GraphNodeId,
) -> TransitionQuality {
    let Some(edge) = graph
        .edges
        .iter()
        .find(|edge| edge.from == state.active_node && edge.to == target)
    else {
        return TransitionQuality {
            legality: TransitionLegality::Illegal,
            reason: format!("illegal transition {} -> {}", state.active_node.0, target.0),
            evidence_delta: 0,
            context_delta: 0,
            risk_delta: 15,
            repetition_penalty: 0,
            expected_next_observation: None,
        };
    };
    let missing = edge
        .guards
        .iter()
        .filter_map(|guard| evaluate_guard(*guard, graph, state).err())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return TransitionQuality {
            legality: TransitionLegality::Blocked,
            reason: missing.join(", "),
            evidence_delta: 0,
            context_delta: 0,
            risk_delta: 8,
            repetition_penalty: repetition_penalty(state, target),
            expected_next_observation: Some(format!("resolve {}", missing.join("+"))),
        };
    }
    TransitionQuality {
        legality: TransitionLegality::Legal,
        reason: edge.rationale.to_string(),
        evidence_delta: evidence_delta(edge.kind, target),
        context_delta: context_delta(edge.kind),
        risk_delta: risk_delta(edge.kind),
        repetition_penalty: repetition_penalty(state, target),
        expected_next_observation: Some(next_observation(edge.kind, target).to_string()),
    }
}

fn evidence_delta(kind: EdgeKind, target: GraphNodeId) -> i16 {
    match (kind, target.0) {
        (EdgeKind::Verify | EdgeKind::Complete, _) => 18,
        (_, "integrate-evidence" | "observe" | "observe-result") => 14,
        (EdgeKind::Execute, _) => 8,
        (EdgeKind::Plan, _) => 6,
        _ => 2,
    }
}

fn context_delta(kind: EdgeKind) -> i16 {
    match kind {
        EdgeKind::SelectContext | EdgeKind::Compact => 12,
        EdgeKind::Plan => 5,
        _ => 0,
    }
}

fn risk_delta(kind: EdgeKind) -> i16 {
    match kind {
        EdgeKind::Recover | EdgeKind::Compact => -10,
        EdgeKind::Complete => -6,
        EdgeKind::Execute => 4,
        _ => 0,
    }
}

fn repetition_penalty(state: &TaskGraphState, target: GraphNodeId) -> i16 {
    state
        .transitions
        .last()
        .filter(|record| record.to == target)
        .map_or(0, |_| 12)
}

fn next_observation(kind: EdgeKind, target: GraphNodeId) -> &'static str {
    match kind {
        EdgeKind::Plan => "plan quality improves",
        EdgeKind::SelectContext => "context evidence selected",
        EdgeKind::Execute => "typed action output recorded",
        EdgeKind::Verify => "verification evidence recorded",
        EdgeKind::Recover => "fault route narrows",
        EdgeKind::Compact => "context pressure drops",
        EdgeKind::Complete => "completion evidence closes gaps",
        EdgeKind::Start | EdgeKind::Maintain => target.0,
    }
}
