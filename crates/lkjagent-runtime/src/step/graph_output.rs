use lkjagent_context::assemble::append_frame;
use lkjagent_graph::{
    completion_decision, EvidenceKind, GraphNodeId, TaskGraphState, TaskPhase, TransitionDecision,
};
use lkjagent_protocol::Action;
use lkjagent_tools::dispatch::DispatchOutput;
use lkjagent_tools::observe::OutputKind;

use crate::graph_state::{evidence_record, graph_notice_frame, status_str};
use crate::step::action_params::action_param;
use crate::step::Effect;
use crate::task::{PendingAction, RuntimeState};

pub(super) fn update_graph_after_output(
    state: &mut RuntimeState,
    pending: &PendingAction,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) {
    if !matches!(&output.kind, OutputKind::Observation { status } if status == "ok") {
        return;
    }
    let Some(graph) = state.graph.as_mut() else {
        return;
    };
    if add_tool_evidence(graph, pending, output, effects) {
        refresh_graph_phase(graph);
        push_case_update(graph, effects);
        state.context = append_frame(&state.context, graph_notice_frame(graph));
    }
}

fn add_tool_evidence(
    graph: &mut TaskGraphState,
    pending: &PendingAction,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> bool {
    match pending.action.tool.as_str() {
        "graph.evidence" => add_explicit_graph_evidence(graph, &pending.action, effects),
        "shell.run" => {
            let observed = ensure_evidence(
                graph,
                "observation",
                EvidenceKind::Observation,
                output,
                None,
                effects,
            );
            let verified = ensure_evidence(
                graph,
                "verification",
                EvidenceKind::Verification,
                output,
                None,
                effects,
            );
            observed || verified
        }
        "fs.read" | "fs.write" | "fs.edit" | "memory.find" | "memory.save" => {
            let path = action_param(&pending.action, "path");
            let path = (!path.is_empty()).then_some(path);
            ensure_evidence(
                graph,
                "observation",
                EvidenceKind::Observation,
                output,
                path,
                effects,
            )
        }
        _ => false,
    }
}

fn add_explicit_graph_evidence(
    graph: &mut TaskGraphState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> bool {
    let requirement = action_param(action, "kind");
    let summary = action_param(action, "summary");
    let path = action_param(action, "path");
    let path = (!path.is_empty()).then_some(path);
    let kind = evidence_kind_for(&requirement);
    add_evidence(graph, &requirement, kind, summary, path, effects)
}

fn ensure_evidence(
    graph: &mut TaskGraphState,
    requirement: &str,
    kind: EvidenceKind,
    output: &DispatchOutput,
    path: Option<String>,
    effects: &mut Vec<Effect>,
) -> bool {
    if !graph
        .evidence_requirements
        .iter()
        .any(|item| item == requirement)
    {
        return false;
    }
    let summary = output
        .content
        .lines()
        .next()
        .unwrap_or("tool output")
        .to_string();
    add_evidence(graph, requirement, kind, summary, path, effects)
}

fn add_evidence(
    graph: &mut TaskGraphState,
    requirement: &str,
    kind: EvidenceKind,
    summary: String,
    path: Option<String>,
    effects: &mut Vec<Effect>,
) -> bool {
    if graph
        .evidence
        .iter()
        .any(|row| row.requirement == requirement)
    {
        return false;
    }
    let evidence = evidence_record(requirement, kind, summary, path);
    push_evidence_record(graph, &evidence, effects);
    graph.evidence.push(evidence);
    graph
        .pending_checks
        .retain(|check| check != "focused verification");
    true
}

fn push_evidence_record(
    graph: &TaskGraphState,
    evidence: &lkjagent_graph::EvidenceRecord,
    effects: &mut Vec<Effect>,
) {
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::RecordGraphEvidence {
            case_id,
            requirement: evidence.requirement.clone(),
            kind: evidence.kind.as_str().to_string(),
            summary: evidence.summary.clone(),
            path: evidence.path.clone(),
        });
    }
}

fn push_case_update(graph: &TaskGraphState, effects: &mut Vec<Effect>) {
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::UpdateGraphCase {
            case_id,
            phase: graph.phase.as_str().to_string(),
            active_node: graph.active_node.0.to_string(),
            status: status_str(graph.status).to_string(),
        });
    }
}

fn refresh_graph_phase(graph: &mut TaskGraphState) {
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

fn has_evidence(graph: &TaskGraphState, requirement: &str) -> bool {
    graph
        .evidence
        .iter()
        .any(|evidence| evidence.requirement == requirement)
}

fn evidence_kind_for(requirement: &str) -> EvidenceKind {
    match requirement {
        "verification" => EvidenceKind::Verification,
        "document-structure" => EvidenceKind::File,
        "plan" => EvidenceKind::Note,
        _ => EvidenceKind::Observation,
    }
}
