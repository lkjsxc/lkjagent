use crate::completion::missing_requirements;
use crate::state::{CompactionPlan, TaskGraphState};

pub fn compaction_plan(state: &TaskGraphState) -> CompactionPlan {
    CompactionPlan {
        case_id: state.case_id,
        phase: state.phase,
        active_node: state.active_node,
        plan: state.plan.clone(),
        evidence: state.evidence.clone(),
        missing_evidence: missing_requirements(state),
        touched_paths: state.touched_paths.clone(),
        selected_packages: state.selected_packages.clone(),
        recovery: state.recovery.clone(),
    }
}
