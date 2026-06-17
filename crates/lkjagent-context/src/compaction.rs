use crate::budget::{POST_COMPACTION_TARGET, WHOLE_WINDOW_TRIGGER};
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

pub fn needs_compaction(state: &ContextState) -> bool {
    state.used_tokens() >= WHOLE_WINDOW_TRIGGER
}

pub fn rebuild_plan(
    current: &ContextState,
    prefix: Vec<Frame>,
    task_summary: Frame,
) -> CompactionDecision {
    let next = ContextState::new(prefix, vec![task_summary]);
    let after = next.used_tokens();
    if after <= POST_COMPACTION_TARGET {
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
