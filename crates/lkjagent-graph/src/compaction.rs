use crate::completion::missing_requirements;
use crate::source_graph;
use crate::state::{CompactionPlan, TaskGraphState};
use crate::transition::legal_targets;

pub fn compaction_plan(state: &TaskGraphState) -> CompactionPlan {
    CompactionPlan {
        case_id: state.case_id,
        phase: state.phase,
        active_node: state.active_node,
        objective: state.objective.normalized.clone(),
        non_goals: state.objective.non_goals.clone(),
        plan_steps: state
            .plan
            .steps
            .iter()
            .map(|step| step.title.clone())
            .collect(),
        constraints: state
            .constraints
            .iter()
            .map(|constraint| constraint.summary.clone())
            .collect(),
        risks: state
            .risks
            .iter()
            .map(|risk| risk.summary.clone())
            .collect(),
        success_criteria: state
            .success_criteria
            .iter()
            .map(|criterion| criterion.summary.clone())
            .collect(),
        evidence: state.evidence.records.clone(),
        missing_evidence: missing_requirements(state),
        touched_paths: state.workspace.touched_paths.clone(),
        selected_packages: state.context.selected_packages.clone(),
        package_compression: state
            .context
            .compression
            .iter()
            .map(|item| format!("{}:{:?}", item.package, item.level))
            .collect(),
        recovery: state.recovery.clone(),
        completion_ready: state.completion.ready,
        legal_next_transitions: legal_targets(source_graph(), state.active_node)
            .iter()
            .map(|node| node.0.to_string())
            .collect(),
    }
}
