use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_context::compaction::{rebuild_plan, CompactionDecision};
use lkjagent_context::model::Frame;

use crate::step::frames::result;
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason, TaskState};

pub(super) fn compact_step(
    mut state: RuntimeState,
    prefix: Vec<Frame>,
    summary: Frame,
    memory_ids: Vec<i64>,
    policy: ContextBudgetPolicy,
) -> StepResult {
    match rebuild_plan(&state.context, prefix, summary, policy) {
        CompactionDecision::Rebuild(plan) => {
            let before = plan.before_tokens;
            let after = plan.after_tokens;
            state.compaction = None;
            state.context = plan.next;
            result(
                state,
                vec![Effect::CompactionRecorded {
                    before_tokens: before,
                    after_tokens: after,
                    memory_ids,
                    policy,
                }],
                Some(StopReason::Compaction),
            )
        }
        CompactionDecision::Keep => result(state, vec![], None),
        CompactionDecision::Fail { reason } => {
            state.compaction = None;
            state.task = TaskState::Paused {
                reason: reason.clone(),
            };
            result(
                state,
                vec![Effect::Pause { reason }],
                Some(StopReason::Compaction),
            )
        }
    }
}
