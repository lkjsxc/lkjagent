use lkjagent_context::assemble::append_frame;
use lkjagent_context::compaction::{rebuild_plan, CompactionDecision};
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::render_notice;
use lkjagent_store::events::EventKind;

use crate::maintenance::{compaction_distillation_prompt, task_summary_required};
use crate::prompt::token_estimate;
use crate::step::frames::result;
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason, TaskState};

pub(super) fn compact_step(
    mut state: RuntimeState,
    prefix: Vec<Frame>,
    summary: Frame,
    memory_ids: Vec<i64>,
) -> StepResult {
    match rebuild_plan(&state.context, prefix, summary) {
        CompactionDecision::Rebuild(plan) => {
            let before = plan.before_tokens;
            let after = plan.after_tokens;
            let summary_required = task_summary_required(&state.task);
            let prompt = compaction_distillation_prompt(summary_required);
            state.context = plan.next;
            state.context = append_frame(
                &state.context,
                Frame::new(
                    FrameKind::Notice(NoticeKind::Compaction),
                    render_notice("compaction", &prompt),
                    token_estimate(&prompt).saturating_add(8),
                ),
            );
            result(
                state,
                vec![
                    Effect::RecordEvent {
                        kind: EventKind::Compaction,
                        content: format!(
                            "before_tokens={before}\nafter_tokens={after}\nmemory_ids={memory_ids:?}"
                        ),
                        tokens: 32,
                    },
                    Effect::CompactionRecorded {
                        before_tokens: before,
                        after_tokens: after,
                        memory_ids,
                    },
                    Effect::DistillCompaction {
                        prompt,
                        max_turns: 4,
                        task_summary_required: summary_required,
                    },
                ],
                Some(StopReason::Compaction),
            )
        }
        CompactionDecision::Keep => result(state, vec![], None),
        CompactionDecision::Fail { reason } => {
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
