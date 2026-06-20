use crate::case_recovery::FaultKind;
use crate::model::{GraphDefinition, TaskFamily, TaskPhase};
use crate::policy::ContextPressureLevel;
use crate::state::TaskGraphState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
    Always,
    HasEvidence(&'static str),
    MissingEvidence(&'static str),
    AllNodeEvidence,
    PlanReady,
    ContextSelected,
    ActivePlanStepReady,
    ActivePlanStepDone,
    PendingChecksClear,
    CompletionReady,
    FamilyIn(&'static [TaskFamily]),
    PhaseIs(TaskPhase),
    ContextPressureAtLeast(ContextPressureLevel),
    FaultCountAtLeast(FaultKind, u8),
    ToolAllowed(&'static [&'static str]),
    DocumentTopologyReady,
    DocumentAuditReady,
    OwnerQuestionOpen,
    OwnerQuestionAbsent,
}

pub fn evaluate_guard(
    guard: Guard,
    graph: &GraphDefinition,
    state: &TaskGraphState,
) -> Result<(), String> {
    if guard_passes(guard, graph, state) {
        Ok(())
    } else {
        Err(guard_name(guard).to_string())
    }
}

pub fn guard_name(guard: Guard) -> &'static str {
    match guard {
        Guard::Always => "always",
        Guard::HasEvidence(id) => id,
        Guard::MissingEvidence(id) => id,
        Guard::AllNodeEvidence => "node-evidence",
        Guard::PlanReady => "plan",
        Guard::ContextSelected => "context",
        Guard::ActivePlanStepReady => "active-plan-step",
        Guard::ActivePlanStepDone => "plan-step-done",
        Guard::PendingChecksClear => "pending-checks-clear",
        Guard::CompletionReady => "completion-ready",
        Guard::FamilyIn(_) => "task-family",
        Guard::PhaseIs(_) => "phase",
        Guard::ContextPressureAtLeast(_) => "context-pressure",
        Guard::FaultCountAtLeast(_, _) => "fault-count",
        Guard::ToolAllowed(_) => "tool-policy",
        Guard::DocumentTopologyReady => "document-topology",
        Guard::DocumentAuditReady => "document-audit",
        Guard::OwnerQuestionOpen => "owner-question-open",
        Guard::OwnerQuestionAbsent => "owner-question-absent",
    }
}

fn guard_passes(guard: Guard, graph: &GraphDefinition, state: &TaskGraphState) -> bool {
    match guard {
        Guard::Always => true,
        Guard::HasEvidence(id) => state.evidence.has(id),
        Guard::MissingEvidence(id) => !state.evidence.has(id),
        Guard::AllNodeEvidence => node_evidence_satisfied(graph, state),
        Guard::PlanReady => state.plan.ready,
        Guard::ContextSelected => !state.context.selected_packages.is_empty(),
        Guard::ActivePlanStepReady => {
            state.plan.active_step.is_some() || !state.plan.steps.is_empty()
        }
        Guard::ActivePlanStepDone => state
            .plan
            .steps
            .iter()
            .any(|step| matches!(step.status, crate::case_plan::StepStatus::Done)),
        Guard::PendingChecksClear => state.evidence.pending_checks.is_empty(),
        Guard::CompletionReady => state.completion.ready,
        Guard::FamilyIn(families) => families.contains(&state.family),
        Guard::PhaseIs(phase) => state.phase == phase,
        Guard::ContextPressureAtLeast(level) => state.context.pressure >= level,
        Guard::FaultCountAtLeast(kind, count) => state.recovery.count(kind) >= count,
        Guard::ToolAllowed(tools) => graph
            .nodes
            .iter()
            .find(|node| node.id == state.active_node)
            .is_some_and(|node| tools.iter().all(|tool| node.allowed_actions.contains(tool))),
        Guard::DocumentTopologyReady => state.document.as_ref().is_some_and(|doc| {
            matches!(
                doc.topology_status,
                crate::case_document::TopologyStatus::Present
            )
        }),
        Guard::DocumentAuditReady => state.document.as_ref().is_some_and(|doc| {
            matches!(
                doc.audit_status,
                crate::case_document::TopologyStatus::Present
            )
        }),
        Guard::OwnerQuestionOpen => state.open_questions.iter().any(|question| {
            question.owner_required && question.status == crate::case_fields::FieldStatus::Open
        }),
        Guard::OwnerQuestionAbsent => state.open_questions.is_empty(),
    }
}

fn node_evidence_satisfied(graph: &GraphDefinition, state: &TaskGraphState) -> bool {
    graph
        .nodes
        .iter()
        .find(|node| node.id == state.active_node)
        .is_none_or(|node| {
            node.evidence
                .iter()
                .all(|item| state.evidence.has(item.name))
        })
}
