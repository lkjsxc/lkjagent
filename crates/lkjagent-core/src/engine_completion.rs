use crate::engine::Command;
use crate::model::{Event, EventKind, TaskSnapshot, TaskState};

pub(crate) fn close_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>) {
    let missing_checks = !snapshot.task.checks.is_empty() && snapshot.check_results.is_empty();
    let passed = !missing_checks && snapshot.check_results.iter().all(|result| result.passed);
    if passed {
        snapshot.task.state = TaskState::Closed;
        record_event(
            commands,
            EventKind::TaskClosed,
            snapshot.task.summary.clone(),
        );
    } else {
        block_task(snapshot, commands, "task checks failed");
    }
}

pub(crate) fn block_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>, reason: &str) {
    snapshot.task.state = TaskState::Blocked;
    snapshot.task.summary = reason.to_string();
    record_event(commands, EventKind::TaskBlocked, reason.to_string());
}

pub(crate) fn record_event(commands: &mut Vec<Command>, kind: EventKind, content: String) {
    commands.push(Command::RecordEvent(Event { kind, content }));
}
