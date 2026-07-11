use crate::docs_tree::validate_plan;
use crate::engine::Command;
use crate::engine_completion::{block_task, record_event};
use crate::engine_extend::insert_after;
use crate::model::{Event, EventKind, StepState, TaskSnapshot, TaskState};
use crate::parse::{Action, PlanLine};
use crate::plan::plan_steps;
use crate::runtime_decision::EffectCommand;

pub(crate) fn memory_save(action: &Action) -> Option<(String, String)> {
    if action.tool == "memory.save" {
        Some((param(action, "topic")?, param(action, "content")?))
    } else {
        None
    }
}

pub(crate) fn handle_native_effect(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    effect: &EffectCommand,
) {
    match (
        effect.name.as_str(),
        effect.path.as_deref(),
        effect.content.as_deref(),
    ) {
        ("workspace.write_text", Some(path), Some(content)) if !content.trim().is_empty() => {
            commands.push(Command::WriteFile {
                path: path.to_string(),
                content: content.to_string(),
            });
        }
        ("workspace.append_text", Some(path), Some(content)) if !content.trim().is_empty() => {
            commands.push(Command::AppendFile {
                path: path.to_string(),
                content: content.to_string(),
            });
        }
        _ => block_task(snapshot, commands, "unsupported native effect command"),
    }
}

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

pub(crate) fn finish_plan(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    lines: Vec<PlanLine>,
) {
    let additions = plan_steps(&snapshot.steps[index], lines);
    if snapshot.steps[index].title == "docs tree plan" {
        if let Err(diagnosis) = validate_plan(&snapshot.steps[index], &additions) {
            record_plan_fault(snapshot, commands, index, diagnosis);
            return;
        }
    }
    snapshot.steps[index].state = StepState::Done;
    let additions = insert_after(snapshot, index, &additions);
    commands.push(Command::AddSteps(additions));
}

fn record_plan_fault(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    diagnosis: String,
) {
    let step = &mut snapshot.steps[index];
    step.state = StepState::Active;
    step.attempts_used += 1;
    let kind = if step.attempts_used >= 3 {
        step.state = StepState::Blocked;
        EventKind::StepBlocked
    } else {
        EventKind::Notice
    };
    commands.push(Command::RecordEvent(Event {
        kind,
        content: diagnosis,
    }));
}

pub(crate) fn action_fingerprint(action: &Action) -> String {
    let mut hash = 14_695_981_039_346_656_037_u64;
    mix(&mut hash, action.tool.as_bytes());
    for (name, value) in &action.params {
        mix(&mut hash, name.as_bytes());
        mix(&mut hash, value.as_bytes());
    }
    format!("{hash:016x}")
}

fn mix(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(1_099_511_628_211);
    }
}

fn param(action: &Action, name: &str) -> Option<String> {
    action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value.clone())
}
