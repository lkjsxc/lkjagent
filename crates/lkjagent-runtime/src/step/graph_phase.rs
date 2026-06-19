use lkjagent_graph::{
    completion_decision, EvidenceKind, GraphNodeId, TaskGraphState, TaskPhase, TransitionDecision,
};

pub(super) fn refresh_graph_phase(graph: &mut TaskGraphState) {
    match completion_decision(graph) {
        TransitionDecision::Admit { .. } => {
            graph.phase = TaskPhase::Completion;
            graph.active_node = GraphNodeId("complete");
        }
        TransitionDecision::Defer { .. } => {
            if has_evidence(graph, "verification") {
                graph.phase = TaskPhase::Verification;
                graph.active_node = GraphNodeId("verify");
            } else {
                graph.phase = TaskPhase::Execution;
                graph.active_node = GraphNodeId("execute");
            }
        }
        TransitionDecision::Recover { .. } | TransitionDecision::Refuse { .. } => {
            graph.phase = TaskPhase::Recovery;
            graph.active_node = GraphNodeId("recover");
        }
    }
}

pub(super) fn evidence_kind_for(requirement: &str) -> EvidenceKind {
    match requirement {
        "verification" => EvidenceKind::Verification,
        "document-structure" => EvidenceKind::File,
        "plan" => EvidenceKind::Note,
        _ => EvidenceKind::Observation,
    }
}

fn has_evidence(graph: &TaskGraphState, requirement: &str) -> bool {
    graph
        .evidence
        .iter()
        .any(|evidence| evidence.requirement == requirement)
}
