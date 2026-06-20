use lkjagent_graph::case_context::PackageCompression;
use lkjagent_graph::case_fields::{ConstraintRecord, FieldStatus, SuccessCriterion};
use lkjagent_graph::{admit_transition, source_graph, EvidenceKind, TaskGraphState};
use lkjagent_protocol::Action;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::graph_parse::node_id;
use crate::step::action_params::action_param;
use crate::step::graph_output_evidence::ensure_evidence;
use crate::step::graph_output_plan_helpers::{
    extend_assumptions, extend_constraints, extend_risks, lines, parse_checks, parse_steps,
    push_note_effect, push_plan_effect,
};
use crate::step::Effect;

pub(super) fn apply_plan(
    graph: &mut TaskGraphState,
    action: &Action,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> bool {
    graph.plan.objective = action_param(action, "objective");
    graph.plan.reason = action_param(action, "reason");
    graph.plan.steps = parse_steps(&action_param(action, "steps"), graph.active_node);
    graph.plan.checks = parse_checks(&action_param(action, "checks"));
    graph.workspace.candidate_paths = lines(&action_param(action, "paths"));
    extend_constraints(graph, &action_param(action, "constraints"));
    extend_assumptions(graph, &action_param(action, "assumptions"));
    extend_risks(graph, &action_param(action, "risks"));
    graph.plan.ready = !graph.plan.objective.trim().is_empty()
        && !graph.plan.steps.is_empty()
        && (!graph.plan.checks.is_empty() || !graph.workspace.candidate_paths.is_empty());
    graph.plan.active_step = graph.plan.steps.first().map(|step| step.id.clone());
    push_plan_effect(graph, effects);
    graph.plan.ready && ensure_evidence(graph, "plan", EvidenceKind::Plan, output, None, effects)
}

pub(super) fn apply_context(
    graph: &mut TaskGraphState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> bool {
    graph.context.selected_packages = lines(&action_param(action, "packages"));
    graph.context.loaded_packages = graph.context.selected_packages.clone();
    graph.context.compression = graph
        .context
        .selected_packages
        .iter()
        .map(|package| PackageCompression {
            package: package.clone(),
            level: graph.context.pressure,
        })
        .collect();
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::RecordGraphContext {
            case_id,
            packages: graph.context.selected_packages.clone(),
            reason: action_param(action, "reason"),
        });
    }
    true
}

pub(super) fn apply_transition(
    graph: &mut TaskGraphState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> bool {
    let from_node = graph.active_node;
    let target = node_id(&action_param(action, "target"));
    if !matches!(
        admit_transition(source_graph(), graph, target),
        lkjagent_graph::TransitionDecision::Admit { .. }
    ) {
        return false;
    }
    graph.active_node = target;
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::RecordGraphTransition {
            case_id,
            from_node: from_node.0.to_string(),
            to_node: target.0.to_string(),
            decision: "admitted".to_string(),
            reason: action_param(action, "reason"),
        });
    }
    true
}

pub(super) fn apply_note(
    graph: &mut TaskGraphState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> bool {
    let summary = action_param(action, "summary");
    let kind = action_param(action, "kind");
    match kind.as_str() {
        "constraint" => graph
            .constraints
            .push(ConstraintRecord::hard(summary, "graph.note")),
        "assumption" => graph
            .assumptions
            .push(lkjagent_graph::case_fields::AssumptionRecord {
                summary,
                status: FieldStatus::Open,
            }),
        "risk" => graph.risks.push(lkjagent_graph::case_fields::RiskRecord {
            summary,
            mitigation: "track during plan review".to_string(),
            status: FieldStatus::Open,
        }),
        "success" => graph.success_criteria.push(SuccessCriterion {
            summary,
            status: FieldStatus::Open,
        }),
        "path" => graph.workspace.candidate_paths.push(summary),
        _ => return false,
    }
    push_note_effect(graph, action, kind, effects);
    true
}
