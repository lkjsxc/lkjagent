use crate::completion::missing_requirements;
use crate::state::{CompactionPlan, TaskGraphState};

pub fn compaction_plan(state: &TaskGraphState) -> CompactionPlan {
    CompactionPlan {
        case_id: state.case_id,
        phase: state.phase,
        active_node: state.active_node,
        objective: state.objective.normalized.clone(),
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
        evidence: state.evidence.records.clone(),
        missing_evidence: missing_requirements(state),
        touched_paths: state.workspace.touched_paths.clone(),
        selected_packages: state.context.selected_packages.clone(),
        recovery: state.recovery.clone(),
        completion_ready: state.completion.ready,
    }
}
