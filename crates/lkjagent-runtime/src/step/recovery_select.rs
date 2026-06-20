use lkjagent_graph::{
    best_next_transition, source_graph, GraphNodeId, TaskGraphState, TransitionIntent,
    TransitionSelection,
};

use crate::step::fault_wait::RecoveryFault;

pub(super) fn recovery_transition(
    graph: &TaskGraphState,
    fault: RecoveryFault,
) -> TransitionSelection {
    let direct = best_next_transition(&source_graph(), graph, intent(fault));
    if direct.target.is_some() && direct.legality == lkjagent_graph::TransitionLegality::Legal {
        return direct;
    }
    let mut routed = graph.clone();
    routed.active_node = GraphNodeId("recover");
    let selected = best_next_transition(&source_graph(), &routed, intent(fault));
    if selected.target.is_some() {
        selected
    } else {
        fallback_selection(fault)
    }
}

fn intent(fault: RecoveryFault) -> TransitionIntent {
    match fault {
        RecoveryFault::Parse => TransitionIntent::AfterParseFault,
        RecoveryFault::Params => TransitionIntent::AfterParamFault,
        RecoveryFault::Repeat => TransitionIntent::AfterRepeatFault,
        RecoveryFault::Tool => TransitionIntent::AfterToolFault,
    }
}

fn fallback_selection(fault: RecoveryFault) -> TransitionSelection {
    TransitionSelection {
        target: Some(fallback_target(fault)),
        legality: lkjagent_graph::TransitionLegality::Legal,
        reason: "fallback recovery target".to_string(),
        missing: Vec::new(),
        quality: None,
        forced_action_class: None,
    }
}

fn fallback_target(fault: RecoveryFault) -> GraphNodeId {
    match fault {
        RecoveryFault::Parse => GraphNodeId("recover-parse"),
        RecoveryFault::Params => GraphNodeId("recover-params"),
        RecoveryFault::Repeat => GraphNodeId("recover-repeat"),
        RecoveryFault::Tool => GraphNodeId("recover-tool"),
    }
}
