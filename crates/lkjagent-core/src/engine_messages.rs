use crate::engine::Command;
use crate::engine_completion::record_event;
use crate::model::{EventKind, StepState, TaskSnapshot, TaskState};

pub(crate) fn finish_message(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    summary: String,
) {
    snapshot.steps[index].state = StepState::Done;
    snapshot.task.summary = summary.clone();
    record_event(commands, EventKind::StepDone, summary);
}

pub(crate) fn wait_for_answer(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    question: String,
) {
    snapshot.task.state = TaskState::Waiting;
    snapshot.steps[index].state = StepState::Active;
    record_event(commands, EventKind::Question, question);
}

pub(crate) fn block_unexpected_message(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
) {
    snapshot.steps[index].state = StepState::Blocked;
    record_event(
        commands,
        EventKind::StepBlocked,
        "message cannot settle selected action work".to_string(),
    );
}
