use crate::checks::{evaluate, CommandFact, FileFact};
use crate::engine::Command;
use crate::model::*;
use crate::parse::{Action, ParseFault, ParsedOutput, PlanLine};
use crate::plan::plan_steps;
use crate::render::Prompt;

pub(crate) fn handle_fault(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    step_id: u64,
    prompt: &Prompt,
    fault: ParseFault,
) {
    let Some(index) = step_index(snapshot, step_id) else {
        return;
    };
    let diagnosis = format!("{fault:?}");
    let step = &mut snapshot.steps[index];
    step.state = StepState::Active;
    step.attempts_used = step.attempts_used.saturating_add(1);
    let attempt = attempt(step, prompt, AttemptOutcome::ParseFault, &diagnosis);
    snapshot.attempts.push(attempt.clone());
    commands.push(Command::RecordAttempt(attempt));
    if snapshot.steps[index].attempts_used >= 3 {
        snapshot.steps[index].state = StepState::Blocked;
        record_event(commands, EventKind::StepBlocked, diagnosis);
    }
}

pub(crate) fn handle_model(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    step_id: u64,
    prompt: &Prompt,
    parsed: ParsedOutput,
) {
    let Some(index) = step_index(snapshot, step_id) else {
        return;
    };
    let attempt_row = attempt(&snapshot.steps[index], prompt, AttemptOutcome::Ok, "ok");
    snapshot.attempts.push(attempt_row.clone());
    commands.push(Command::RecordAttempt(attempt_row));
    match parsed {
        ParsedOutput::Content(content) => {
            finish_content(&mut snapshot.steps[index], commands, content)
        }
        ParsedOutput::Plan(lines) => finish_plan(snapshot, commands, index, lines),
        ParsedOutput::Action(action) => {
            keep_exploring(&mut snapshot.steps[index], commands, action)
        }
        ParsedOutput::Finish(summary) | ParsedOutput::Message(summary) => {
            finish_message(snapshot, commands, index, summary);
        }
        ParsedOutput::Question(question) => wait_for_answer(snapshot, commands, index, question),
        ParsedOutput::Verdict(result) => finish_verdict(snapshot, commands, index, result),
    }
}

pub(crate) fn handle_checks(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    step_id: u64,
    files: &[FileFact],
    command_facts: &[CommandFact],
) {
    let Some(index) = step_index(snapshot, step_id) else {
        return;
    };
    let results = snapshot.steps[index]
        .checks
        .iter()
        .map(|spec| evaluate(spec, files, command_facts))
        .collect::<Vec<_>>();
    let passed = results.iter().all(|result| result.passed);
    snapshot.check_results.extend(results.clone());
    commands.push(Command::RecordChecks(results));
    if passed {
        snapshot.steps[index].state = StepState::Done;
        record_event(
            commands,
            EventKind::StepDone,
            snapshot.steps[index].title.clone(),
        );
    } else {
        snapshot.steps[index].attempts_used += 1;
    }
}

pub(crate) fn close_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>) {
    let passed = snapshot.check_results.iter().all(|result| result.passed);
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

fn finish_content(step: &mut Step, commands: &mut Vec<Command>, content: String) {
    if let Some(path) = &step.output_path {
        commands.push(Command::WriteFile {
            path: path.clone(),
            content,
        });
    }
    step.state = StepState::Done;
    record_event(commands, EventKind::StepDone, step.title.clone());
}

fn keep_exploring(step: &mut Step, commands: &mut Vec<Command>, action: Action) {
    step.state = StepState::Active;
    step.actions_used = step.actions_used.saturating_add(1);
    step.inputs = format!("last_action={} count={}", action.tool, step.actions_used);
    commands.push(Command::RunExplore(action));
    if step.actions_used >= step.action_budget && step.action_budget > 0 {
        step.state = StepState::Blocked;
    }
}

fn finish_message(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    summary: String,
) {
    snapshot.steps[index].state = StepState::Done;
    snapshot.task.summary = summary.clone();
    record_event(commands, EventKind::StepDone, summary);
}

fn wait_for_answer(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    question: String,
) {
    snapshot.task.state = TaskState::Waiting;
    snapshot.steps[index].state = StepState::Active;
    record_event(commands, EventKind::Question, question);
}

fn finish_verdict(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    result: CheckResult,
) {
    let passed = result.passed;
    snapshot.check_results.push(result.clone());
    snapshot.steps[index].state = if passed {
        StepState::Done
    } else {
        StepState::Active
    };
    commands.push(Command::RecordChecks(vec![result]));
}

fn step_index(snapshot: &TaskSnapshot, step_id: u64) -> Option<usize> {
    snapshot.steps.iter().position(|step| step.id == step_id)
}

fn attempt(step: &Step, prompt: &Prompt, outcome: AttemptOutcome, diagnosis: &str) -> Attempt {
    Attempt {
        step_id: step.id,
        ordinal: step.actions_used + step.attempts_used + 1,
        prompt_fingerprint: prompt.fingerprint.clone(),
        outcome,
        diagnosis: diagnosis.to_string(),
        tokens_in: 0,
        tokens_out: 0,
    }
}

fn record_event(commands: &mut Vec<Command>, kind: EventKind, content: String) {
    commands.push(Command::RecordEvent(Event { kind, content }));
}

fn finish_plan(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    lines: Vec<PlanLine>,
) {
    let additions = plan_steps(&snapshot.steps[index], lines);
    snapshot.steps[index].state = StepState::Done;
    snapshot.steps.extend(additions.clone());
    commands.push(Command::AddSteps(additions));
}
