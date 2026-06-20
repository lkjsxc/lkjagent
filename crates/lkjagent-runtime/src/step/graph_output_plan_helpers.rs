use lkjagent_graph::case_fields::{AssumptionRecord, ConstraintRecord, FieldStatus, RiskRecord};
use lkjagent_graph::case_plan::{CheckStatus, PlanStep, StepId, StepStatus, VerificationCheck};
use lkjagent_graph::{GraphNodeId, TaskGraphState};
use lkjagent_protocol::Action;

use crate::step::action_params::action_param;
use crate::step::{Effect, GraphPlanStepEffect};

pub(super) fn parse_steps(value: &str, node: GraphNodeId) -> Vec<PlanStep> {
    step_lines(value)
        .into_iter()
        .enumerate()
        .map(|(index, title)| PlanStep {
            id: StepId(format!("step-{}", index.saturating_add(1))),
            title,
            rationale: "required by graph.plan".to_string(),
            status: if index == 0 {
                StepStatus::Active
            } else {
                StepStatus::Pending
            },
            node,
            target_paths: Vec::new(),
            required_evidence: vec!["observation".to_string()],
            verification: Vec::new(),
        })
        .collect()
}

pub(super) fn advance_active_step(graph: &mut TaskGraphState) -> bool {
    let Some(active) = graph.plan.active_step.clone() else {
        return false;
    };
    let Some(index) = graph.plan.steps.iter().position(|step| step.id == active) else {
        graph.plan.active_step = None;
        return true;
    };
    if matches!(graph.plan.steps[index].status, StepStatus::Done) {
        return false;
    }
    graph.plan.steps[index].status = StepStatus::Done;
    let next = graph
        .plan
        .steps
        .iter_mut()
        .skip(index.saturating_add(1))
        .find(|step| matches!(step.status, StepStatus::Pending));
    graph.plan.active_step = next.map(|step| {
        step.status = StepStatus::Active;
        step.id.clone()
    });
    true
}

pub(super) fn parse_checks(value: &str) -> Vec<VerificationCheck> {
    lines(value)
        .into_iter()
        .enumerate()
        .map(|(index, command)| VerificationCheck {
            id: format!("check-{}", index.saturating_add(1)),
            command,
            status: CheckStatus::Pending,
        })
        .collect()
}

pub(super) fn extend_constraints(graph: &mut TaskGraphState, value: &str) {
    graph.constraints.extend(
        lines(value)
            .into_iter()
            .map(|summary| ConstraintRecord::hard(summary, "graph.plan")),
    );
}

pub(super) fn extend_assumptions(graph: &mut TaskGraphState, value: &str) {
    graph
        .assumptions
        .extend(lines(value).into_iter().map(|summary| AssumptionRecord {
            summary,
            status: FieldStatus::Open,
        }));
}

pub(super) fn extend_risks(graph: &mut TaskGraphState, value: &str) {
    graph
        .risks
        .extend(lines(value).into_iter().map(|summary| RiskRecord {
            summary,
            mitigation: "verify during execution".to_string(),
            status: FieldStatus::Open,
        }));
}

pub(super) fn push_plan_effect(graph: &TaskGraphState, effects: &mut Vec<Effect>) {
    let Some(case_id) = graph.case_id else {
        return;
    };
    effects.push(Effect::RecordGraphPlan {
        case_id,
        steps: graph
            .plan
            .steps
            .iter()
            .map(|step| GraphPlanStepEffect {
                step_id: step.id.0.clone(),
                title: step.title.clone(),
                rationale: step.rationale.clone(),
                status: step_status(step.status).to_string(),
                node: step.node.0.to_string(),
                target_paths: step.target_paths.clone(),
                checks: step.verification.clone(),
            })
            .collect(),
    });
}

pub(super) fn push_note_effect(
    graph: &TaskGraphState,
    action: &Action,
    kind: String,
    effects: &mut Vec<Effect>,
) {
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::RecordGraphNote {
            case_id,
            kind,
            summary: action_param(action, "summary"),
        });
    }
}

fn step_status(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pending => "pending",
        StepStatus::Active => "active",
        StepStatus::Blocked => "blocked",
        StepStatus::Done => "done",
        StepStatus::Skipped => "skipped",
    }
}

pub(super) fn lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn step_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .flat_map(|line| line.split(';'))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
