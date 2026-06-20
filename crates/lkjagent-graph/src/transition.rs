use crate::guards::evaluate_guard;
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

pub fn admitted_targets(graph: &GraphDefinition, state: &TaskGraphState) -> Vec<GraphNodeId> {
    graph
        .edges
        .iter()
        .filter(|edge| edge.from == state.active_node)
        .filter(|edge| {
            edge.guards
                .iter()
                .all(|guard| evaluate_guard(*guard, graph, state).is_ok())
        })
        .map(|edge| edge.to)
        .collect()
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
