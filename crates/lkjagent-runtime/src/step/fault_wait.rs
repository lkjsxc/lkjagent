use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;

use crate::prompt::token_estimate;
use crate::step::frames::append_notice;
use crate::step::Effect;
use crate::task::{RuntimeState, TaskState};

#[derive(Clone, Copy)]
pub(super) enum RecoveryFault {
    Parse,
    Repeat,
    Tool,
}

pub(super) fn enter_recovery_wait(
    mut state: RuntimeState,
    fault: RecoveryFault,
    count: u8,
    effects: &mut Vec<Effect>,
) -> RuntimeState {
    let question = wait_question(fault, count);
    state = append_notice(state, NoticeKind::Error, &question);
    effects.push(Effect::RecordEvent {
        kind: EventKind::Notice,
        content: question.clone(),
        tokens: token_estimate(&question) as i64,
    });
    state.task = TaskState::Waiting { question };
    state
}

fn wait_question(fault: RecoveryFault, count: u8) -> String {
    let prefix = match fault {
        RecoveryFault::Parse => "Consecutive parse faults",
        RecoveryFault::Repeat => "Consecutive repeated actions",
        RecoveryFault::Tool => "Consecutive tool errors",
    };
    format!(
        "{prefix} reached count={count}. Send guidance to continue, narrow the task, or provide corrected state before more turns are spent."
    )
}
