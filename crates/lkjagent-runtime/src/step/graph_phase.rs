use lkjagent_graph::{
    best_next_transition, completion::refresh_completion_state, completion_decision, source_graph,
    EvidenceKind, GraphNodeId, NodeKind, TaskGraphState, TaskPhase, TransitionDecision,
    TransitionIntent, TransitionLegality,
};

pub(super) fn refresh_graph_phase(graph: &mut TaskGraphState, intent: TransitionIntent) {
    refresh_completion_state(graph);
    match completion_decision(graph) {
        TransitionDecision::Admit { .. } => {
            graph.phase = TaskPhase::Completion;
            graph.active_node = GraphNodeId("complete");
        }
        TransitionDecision::Defer { .. } => {
            if route_document_structure_gap(graph) {
            } else if !apply_selected_transition(graph, intent) {
                refresh_incomplete_phase(graph);
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
        "plan" => EvidenceKind::Plan,
        "verification" => EvidenceKind::Verification,
        "artifact-readiness" | "document-structure" => EvidenceKind::File,
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
    } else if document_gap(graph) {
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

fn route_document_structure_gap(graph: &mut TaskGraphState) -> bool {
    if !matches!(
        graph.family,
        lkjagent_graph::TaskFamily::Documentation | lkjagent_graph::TaskFamily::KnowledgeBase
    ) || !graph.evidence.has("observation")
    {
        return false;
    }
    if graph.evidence.has("document-structure") && !needs_artifact_readiness(graph) {
        return false;
    }
    graph.phase = TaskPhase::Execution;
    graph.active_node = GraphNodeId("document");
    graph.next_action_class = if graph.evidence.has("document-structure") {
        "artifact-readiness".to_string()
    } else {
        "document-structure".to_string()
    };
    true
}

fn document_gap(graph: &TaskGraphState) -> bool {
    matches!(
        graph.family,
        lkjagent_graph::TaskFamily::Documentation | lkjagent_graph::TaskFamily::KnowledgeBase
    ) && (!graph.evidence.has("document-structure") || needs_artifact_readiness(graph))
}

fn needs_artifact_readiness(graph: &TaskGraphState) -> bool {
    graph
        .evidence
        .requirements
        .iter()
        .any(|requirement| requirement.id == "artifact-readiness")
        && !graph.evidence.has("artifact-readiness")
}

fn apply_selected_transition(graph: &mut TaskGraphState, intent: TransitionIntent) -> bool {
    let source = source_graph();
    let mut moved = false;
    let mut current_intent = intent;
    for _ in 0..8 {
        let selected = best_next_transition(&source, graph, current_intent);
        if selected.legality != TransitionLegality::Legal {
            break;
        }
        let Some(target) = selected.target else {
            break;
        };
        graph.active_node = target;
        graph.phase = phase_for_node(&source, target);
        graph.next_action_class = selected
            .forced_action_class
            .unwrap_or_else(|| format!("graph-node:{}", target.0));
        moved = true;
        if !should_auto_continue(&source, graph, target, intent) {
            break;
        }
        current_intent = TransitionIntent::Continue;
    }
    moved
}

fn should_auto_continue(
    source: &lkjagent_graph::GraphDefinition,
    graph: &TaskGraphState,
    target: GraphNodeId,
    intent: TransitionIntent,
) -> bool {
    let Some(node) = source.nodes.iter().find(|node| node.id == target) else {
        return false;
    };
    match intent {
        TransitionIntent::AfterPlan => matches!(node.kind, NodeKind::Planning | NodeKind::State),
        TransitionIntent::AfterObservation => {
            node.kind == NodeKind::State
                && !(target == GraphNodeId("advance-plan") && has_unfinished_step(graph))
        }
        _ => false,
    }
}

fn phase_for_node(source: &lkjagent_graph::GraphDefinition, target: GraphNodeId) -> TaskPhase {
    let kind = source
        .nodes
        .iter()
        .find(|node| node.id == target)
        .map(|node| node.kind)
        .unwrap_or(NodeKind::Planning);
    match kind {
        NodeKind::Intent | NodeKind::Planning => TaskPhase::Planning,
        NodeKind::Context => TaskPhase::Context,
        NodeKind::State | NodeKind::Execution | NodeKind::Document => TaskPhase::Execution,
        NodeKind::Verification => TaskPhase::Verification,
        NodeKind::Recovery => TaskPhase::Recovery,
        NodeKind::Compaction => TaskPhase::Compaction,
        NodeKind::Completion => TaskPhase::Completion,
        NodeKind::Memory | NodeKind::Maintenance => TaskPhase::Maintenance,
    }
}
