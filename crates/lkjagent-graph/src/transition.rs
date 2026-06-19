use crate::completion::missing_requirements;
use crate::model::{GraphDefinition, GraphNodeId};
use crate::state::{TaskGraphState, TransitionDecision};

pub fn legal_targets(graph: GraphDefinition, from: GraphNodeId) -> Vec<GraphNodeId> {
    graph
        .edges
        .iter()
        .filter(|edge| edge.from == from)
        .map(|edge| edge.to)
        .collect()
}

pub fn admit_transition(
    graph: GraphDefinition,
    state: &TaskGraphState,
    target: GraphNodeId,
) -> TransitionDecision {
    if !legal_targets(graph, state.active_node).contains(&target) {
        return TransitionDecision::Refuse {
            reason: format!("illegal transition {} -> {}", state.active_node.0, target.0),
        };
    }
    if target == GraphNodeId("complete") {
        let missing = missing_requirements(state);
        if !missing.is_empty() {
            return TransitionDecision::Defer { missing };
        }
    }
    TransitionDecision::Admit { target }
}
