use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::render_notice;

use crate::prompt::token_estimate;
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason};

pub(super) fn append_notice(
    mut state: RuntimeState,
    kind: NoticeKind,
    content: &str,
) -> RuntimeState {
    let rendered = render_notice(notice_name(kind), content);
    state.context = append_frame(
        &state.context,
        Frame::new(
            FrameKind::Notice(kind),
            rendered,
            token_estimate(content).saturating_add(8),
        ),
    );
    state
}

pub(super) fn result(
    state: RuntimeState,
    effects: Vec<Effect>,
    stop_reason: Option<StopReason>,
) -> StepResult {
    StepResult {
        state,
        effects,
        stop_reason,
    }
}

fn notice_name(kind: NoticeKind) -> &'static str {
    match kind {
        NoticeKind::Truncation => "truncation",
        NoticeKind::Budget => "budget",
        NoticeKind::Error => "error",
        NoticeKind::Compaction => "compaction",
        NoticeKind::Maintenance => "maintenance",
    }
}
