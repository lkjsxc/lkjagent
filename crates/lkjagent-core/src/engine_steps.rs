use crate::engine::Command;
use crate::engine_actions::{action_fingerprint, finish_summary, memory_save};
use crate::engine_completion::record_event;
use crate::engine_extend::{add_steps, insert_after, split_after_fault};
use crate::engine_plan::finish_plan;
use crate::model::*;
use crate::parse::{Action, ParseFault, ParsedOutput};
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
    let diagnosis = crate::parse::parse_fault_diagnosis(&fault);
    let step = &mut snapshot.steps[index];
    step.state = StepState::Active;
    step.attempts_used = step.attempts_used.saturating_add(1);
    let attempt = attempt(step, prompt, AttemptOutcome::ParseFault, &diagnosis);
    snapshot.attempts.push(attempt.clone());
    commands.push(Command::RecordAttempt(attempt));
    if snapshot.steps[index].attempts_used >= 3 {
        snapshot.steps[index].state = StepState::Blocked;
        record_event(commands, EventKind::StepBlocked, diagnosis);
        let additions = split_after_fault(&snapshot.steps[index]);
        let additions = insert_after(snapshot, index, &additions);
        add_steps(commands, additions, "split write after repeated faults");
    }
}

pub(crate) fn handle_endpoint_error(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    step_id: u64,
    prompt: &Prompt,
    error: &str,
) {
    let Some(index) = step_index(snapshot, step_id) else {
        return;
    };
    let step = &mut snapshot.steps[index];
    step.state = StepState::Active;
    step.attempts_used = step.attempts_used.saturating_add(1);
    let attempt = attempt(step, prompt, AttemptOutcome::EndpointError, error);
    snapshot.attempts.push(attempt.clone());
    commands.push(Command::RecordAttempt(attempt));
    if snapshot.steps[index].attempts_used >= 10 {
        snapshot.steps[index].state = StepState::Blocked;
        record_event(commands, EventKind::StepBlocked, error.to_string());
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
    if let ParsedOutput::Action(action) = &parsed {
        if finish_summary(action).is_none() && repeated_action(&snapshot.steps[index], action) {
            let attempt = guard_repeated_action(&mut snapshot.steps[index], commands, prompt);
            snapshot.attempts.push(attempt);
            return;
        }
    }
    let attempt_row = attempt(&snapshot.steps[index], prompt, AttemptOutcome::Ok, "ok");
    snapshot.attempts.push(attempt_row.clone());
    commands.push(Command::RecordAttempt(attempt_row));
    match parsed {
        ParsedOutput::Content(content) => {
            finish_content(&mut snapshot.steps[index], commands, content)
        }
        ParsedOutput::Plan(lines) => finish_plan(snapshot, commands, index, lines),
        ParsedOutput::Action(action) => match finish_summary(&action) {
            Some(summary) => finish_message(snapshot, commands, index, summary),
            None => keep_exploring(&mut snapshot.steps[index], commands, action),
        },
        ParsedOutput::Message(summary) if snapshot.steps[index].kind == StepKind::Ask => {
            wait_for_answer(snapshot, commands, index, summary);
        }
        ParsedOutput::Message(summary) => finish_message(snapshot, commands, index, summary),
        ParsedOutput::Verdict(result) => finish_verdict(snapshot, commands, index, result),
    }
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
    step.inputs = format!(
        "last_action_fingerprint={}\nlast_action={}\ncount={}",
        action_fingerprint(&action),
        action.tool,
        step.actions_used
    );
    if let Some((topic, content)) = memory_save(&action) {
        commands.push(Command::RecordMemory { topic, content });
    }
    commands.push(Command::RunExplore(action));
    if step.actions_used >= step.action_budget && step.action_budget > 0 {
        step.state = StepState::Blocked;
    }
}

fn guard_repeated_action(step: &mut Step, commands: &mut Vec<Command>, prompt: &Prompt) -> Attempt {
    let diagnosis = "repeated action; state the next different action or finish";
    step.state = StepState::Active;
    step.attempts_used = step.attempts_used.saturating_add(1);
    let attempt = attempt(step, prompt, AttemptOutcome::ParseFault, diagnosis);
    commands.push(Command::RecordAttempt(attempt.clone()));
    attempt
}

fn repeated_action(step: &Step, action: &Action) -> bool {
    let Some(previous) = step
        .inputs
        .lines()
        .find_map(|line| line.strip_prefix("last_action_fingerprint="))
    else {
        return false;
    };
    previous == action_fingerprint(action)
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
    snapshot.check_results.push(result.clone());
    snapshot.steps[index].state = StepState::Done;
    if !result.passed {
        snapshot.steps[index].state = StepState::Active;
    }
    commands.push(Command::RecordChecks {
        step_id: snapshot.steps[index].id,
        decision_id: None,
        results: vec![result],
    });
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
        cached_tokens: 0,
        cache_status: "unknown".to_string(),
    }
}
