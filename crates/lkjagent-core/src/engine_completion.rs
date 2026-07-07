use crate::engine::Command;
use crate::model::{Event, EventKind, StepKind, StepState, TaskSnapshot, TaskState, TemplateId};

pub(crate) fn close_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>) {
    if let Some(reason) = completion_blocker(snapshot) {
        block_task(snapshot, commands, &reason);
        return;
    }
    snapshot.task.state = TaskState::Closed;
    record_event(
        commands,
        EventKind::TaskClosed,
        snapshot.task.summary.clone(),
    );
}

pub(crate) fn completion_blocker(snapshot: &TaskSnapshot) -> Option<String> {
    for (index, step) in snapshot.steps.iter().enumerate() {
        if step.state == StepState::Done || skipped_is_superseded(snapshot, index) {
            continue;
        }
        return Some(format!(
            "step {} {} is {}",
            step.id,
            step.title,
            step_state(step.state)
        ));
    }
    check_blocker(snapshot)
}

fn skipped_is_superseded(snapshot: &TaskSnapshot, index: usize) -> bool {
    let step = &snapshot.steps[index];
    if step.state != StepState::Skipped || step.kind != StepKind::Verify {
        return false;
    }
    let later = snapshot.steps.iter().skip(index + 1).collect::<Vec<_>>();
    let repair_done = later.iter().any(|item| {
        matches!(item.kind, StepKind::Write | StepKind::Revise) && item.state == StepState::Done
    });
    let verify_done = later.iter().any(|item| {
        item.kind == StepKind::Verify && item.state == StepState::Done && item.checks == step.checks
    });
    repair_done && verify_done
}

fn check_blocker(snapshot: &TaskSnapshot) -> Option<String> {
    if snapshot.task.checks.is_empty() {
        return artifact_evidence_required(snapshot.task.template)
            .then(|| "artifact evidence missing".to_string());
    }
    if snapshot.check_results.len() < snapshot.task.checks.len() {
        return Some("task checks missing".to_string());
    }
    snapshot
        .check_results
        .iter()
        .find(|result| !result.passed)
        .map(|result| format!("task check failed: {}", result.name))
}

fn artifact_evidence_required(template: TemplateId) -> bool {
    matches!(
        template,
        TemplateId::DocsTree
            | TemplateId::FileWork
            | TemplateId::Journal
            | TemplateId::LegacyArtifact
    )
}

fn step_state(state: StepState) -> &'static str {
    match state {
        StepState::Pending => "pending",
        StepState::Active => "active",
        StepState::Done => "done",
        StepState::Blocked => "blocked",
        StepState::Skipped => "skipped without supersession evidence",
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
