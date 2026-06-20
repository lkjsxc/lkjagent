use lkjagent_graph::{
    completion::refresh_completion_state, completion_decision, EvidenceKind, GraphNodeId,
    TaskGraphState, TaskPhase, TransitionDecision,
};

pub(super) fn refresh_graph_phase(graph: &mut TaskGraphState) {
    refresh_completion_state(graph);
    match completion_decision(graph) {
        TransitionDecision::Admit { .. } => {
            graph.phase = TaskPhase::Completion;
            graph.active_node = GraphNodeId("complete");
        }
        TransitionDecision::Defer { .. } => refresh_incomplete_phase(graph),
        TransitionDecision::Recover { .. } | TransitionDecision::Refuse { .. } => {
            graph.phase = TaskPhase::Recovery;
            graph.active_node = GraphNodeId("recover");
        }
    }
}

pub(super) fn evidence_kind_for(requirement: &str) -> EvidenceKind {
    match requirement {
        "plan" => EvidenceKind::Plan,
        "verification" => EvidenceKind::Verification,
        "document-structure" => EvidenceKind::File,
        _ => EvidenceKind::Observation,
    }
}

fn refresh_incomplete_phase(graph: &mut TaskGraphState) {
    if !graph.plan.ready {
        graph.phase = TaskPhase::Planning;
        graph.active_node = GraphNodeId("plan");
    } else if graph.context.selected_packages.is_empty() {
        graph.phase = TaskPhase::Context;
        graph.active_node = GraphNodeId("context");
    } else if has_unfinished_step(graph) {
        graph.phase = TaskPhase::Execution;
        graph.active_node = if matches!(
            graph.family,
            lkjagent_graph::TaskFamily::Documentation | lkjagent_graph::TaskFamily::KnowledgeBase
        ) {
            GraphNodeId("document")
        } else {
            GraphNodeId("execute")
        };
    } else if needs_verification(graph) && graph.evidence.has("observation") {
        graph.phase = TaskPhase::Verification;
        graph.active_node = GraphNodeId("verify");
    } else if matches!(
        graph.family,
        lkjagent_graph::TaskFamily::Documentation | lkjagent_graph::TaskFamily::KnowledgeBase
    ) && !graph.evidence.has("document-structure")
    {
        graph.phase = TaskPhase::Execution;
        graph.active_node = GraphNodeId("document");
    } else {
        graph.phase = TaskPhase::Execution;
        graph.active_node = GraphNodeId("execute");
    }
}

fn has_unfinished_step(graph: &TaskGraphState) -> bool {
    graph.plan.steps.iter().any(|step| {
        matches!(
            step.status,
            lkjagent_graph::case_plan::StepStatus::Active
                | lkjagent_graph::case_plan::StepStatus::Pending
        )
    })
}

fn needs_verification(graph: &TaskGraphState) -> bool {
    graph
        .evidence
        .requirements
        .iter()
        .any(|requirement| requirement.id == "verification" && !graph.evidence.has("verification"))
}
