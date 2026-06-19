use crate::budget::{ContextBudgetPolicy, ContextPressure};
use crate::model::{ContextState, Frame};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildPlan {
    pub before_tokens: usize,
    pub after_tokens: usize,
    pub next: ContextState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactionDecision {
    Keep,
    Rebuild(RebuildPlan),
    Fail { reason: String },
}

pub fn pressure_for(
    state: &ContextState,
    policy: ContextBudgetPolicy,
    predicted_next_input: usize,
) -> ContextPressure {
    policy.pressure(state.used_tokens(), predicted_next_input)
}

pub fn needs_compaction(
    state: &ContextState,
    policy: ContextBudgetPolicy,
    predicted_next_input: usize,
) -> bool {
    matches!(
        pressure_for(state, policy, predicted_next_input),
        ContextPressure::Orange | ContextPressure::Red
    )
}

pub fn rebuild_plan(
    current: &ContextState,
    prefix: Vec<Frame>,
    task_summary: Frame,
    policy: ContextBudgetPolicy,
) -> CompactionDecision {
    let next = ContextState::new(prefix, vec![task_summary]);
    let after = next.used_tokens();
    if after <= policy.post_compaction_target {
        CompactionDecision::Rebuild(RebuildPlan {
            before_tokens: current.used_tokens(),
            after_tokens: after,
            next,
        })
    } else {
        CompactionDecision::Fail {
            reason: "compaction could not reach the post-compaction target".to_string(),
        }
    }
}
