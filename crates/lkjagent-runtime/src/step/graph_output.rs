use lkjagent_context::assemble::append_frame;
use lkjagent_graph::{EvidenceKind, GraphNodeId, TaskFamily, TaskGraphState, TransitionIntent};
use lkjagent_tools::dispatch::DispatchOutput;
use lkjagent_tools::observe::OutputKind;

use crate::graph_state::graph_notice_frame;
use crate::step::action_params::action_param;
use crate::step::graph_output_evidence::{
    add_document_evidence, add_explicit_graph_evidence, add_shell_evidence, ensure_evidence,
    push_case_update,
};
use crate::step::graph_output_plan::{apply_context, apply_note, apply_plan, apply_transition};
use crate::step::graph_output_plan_helpers::advance_active_step;
use crate::step::graph_phase::refresh_graph_phase;
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
    if add_graph_update(graph, pending, output, effects) {
        refresh_graph_phase(graph, transition_intent(&pending.action));
        push_case_update(graph, effects);
        state.context = append_frame(&state.context, graph_notice_frame(graph));
    }
}

fn transition_intent(action: &lkjagent_protocol::Action) -> TransitionIntent {
    match action.tool.as_str() {
        "graph.plan" | "graph.context" => TransitionIntent::AfterPlan,
        "graph.evidence" if action_param(action, "kind") == "verification" => {
            TransitionIntent::AfterVerification
        }
        "verify.cargo" | "verify.xtask" | "doc.audit" => TransitionIntent::AfterVerification,
        "agent.done" => TransitionIntent::AttemptCompletion,
        _ => TransitionIntent::AfterObservation,
    }
}

fn add_graph_update(
    graph: &mut TaskGraphState,
    pending: &PendingAction,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> bool {
    match pending.action.tool.as_str() {
        "graph.plan" => apply_plan(graph, &pending.action, output, effects),
        "graph.context" => apply_context(graph, &pending.action, effects),
        "graph.transition" => apply_transition(graph, &pending.action, effects),
        "graph.note" => apply_note(graph, &pending.action, effects),
        "graph.evidence" => {
            let evidenced = add_explicit_graph_evidence(graph, &pending.action, effects);
            evidenced || advance_active_step(graph)
        }
        "verify.cargo" | "verify.xtask" => {
            ensure_evidence(
                graph,
                "verification",
                EvidenceKind::Verification,
                output,
                None,
                effects,
            ) || advance_active_step(graph)
        }
        "doc.audit" | "doc.scaffold" => {
            add_document_evidence(graph, output, effects) || advance_active_step(graph)
        }
        "fs.batch_write"
            if graph.active_node == GraphNodeId("document")
                || matches!(
                    graph.family,
                    TaskFamily::Documentation | TaskFamily::KnowledgeBase
                ) =>
        {
            add_document_evidence(graph, output, effects) || advance_active_step(graph)
        }
        "shell.run" => add_shell_evidence(graph, output, effects) || advance_active_step(graph),
        "fs.read" | "fs.read_many" | "fs.write" | "fs.edit" | "fs.patch" | "fs.list"
        | "fs.tree" | "fs.search" | "fs.stat" | "fs.mkdir" | "fs.batch_write"
        | "workspace.summary" | "workspace.index" | "memory.find" | "memory.save" => {
            let path = action_param(&pending.action, "path");
            let observed = ensure_evidence(
                graph,
                "observation",
                EvidenceKind::Observation,
                output,
                (!path.is_empty()).then_some(path),
                effects,
            );
            observed || advance_active_step(graph)
        }
        _ => false,
    }
}
