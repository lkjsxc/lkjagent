use crate::checks::{CommandFact, FileFact};
use crate::engine_checks::handle_checks;
use crate::engine_completion::{block_task, close_task};
use crate::engine_steps::{handle_endpoint_error, handle_fault, handle_model};
use crate::model::*;
use crate::parse::{Action, ParseFault, ParsedOutput};
use crate::render::{render_prompt, render_prompt_for_decision, Prompt};
use crate::runtime_decision::RuntimeDecision;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Work {
    CallModel { step_id: u64, prompt: Prompt },
    RunChecks { step_id: u64 },
    CloseTask,
    BlockTask(String),
    Wait,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    Model(ParsedOutput),
    ParseFault(ParseFault),
    EndpointError(String),
    Checks(Vec<FileFact>, Vec<CommandFact>),
    Noop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    WriteFile {
        path: String,
        content: String,
    },
    RunExplore(Action),
    RecordAttempt(Attempt),
    RecordEvent(Event),
    RecordMemory {
        topic: String,
        content: String,
    },
    RecordChecks {
        step_id: u64,
        results: Vec<CheckResult>,
    },
    AddSteps(Vec<Step>),
}

pub fn next_work_with_decision(snapshot: &TaskSnapshot, decision: &RuntimeDecision) -> Work {
    match snapshot.task.state {
        TaskState::Waiting | TaskState::Blocked | TaskState::Closed => return Work::Wait,
        TaskState::Open => {}
    }
    let operation = decision.operation.0.as_str();
    if operation == "runtime.idle" || operation == "owner.answer" {
        return Work::Wait;
    }
    if operation == "completion.close" {
        return Work::CloseTask;
    }
    if let Some(step_id) = step_operation(operation, "check.run/") {
        return Work::RunChecks { step_id };
    }
    if let Some(step_id) = step_operation(operation, "model.call/") {
        return call_model_work(snapshot, decision, step_id);
    }
    Work::BlockTask(format!("unsupported runtime decision: {operation}"))
}

pub fn next_work(snapshot: &TaskSnapshot) -> Work {
    next_work_rendered(snapshot)
}

fn next_work_rendered(snapshot: &TaskSnapshot) -> Work {
    match snapshot.task.state {
        TaskState::Waiting | TaskState::Blocked | TaskState::Closed => return Work::Wait,
        TaskState::Open => {}
    }
    let Some(step) = snapshot
        .steps
        .iter()
        .find(|step| matches!(step.state, StepState::Pending | StepState::Active))
    else {
        return Work::CloseTask;
    };
    if step.kind == StepKind::Verify && step.checks.iter().all(deterministic) {
        return Work::RunChecks { step_id: step.id };
    }
    Work::CallModel {
        step_id: step.id,
        prompt: render_prompt(&snapshot.task, &snapshot.steps, step),
    }
}

fn call_model_work(snapshot: &TaskSnapshot, decision: &RuntimeDecision, step_id: u64) -> Work {
    let Some(step) = snapshot.steps.iter().find(|step| step.id == step_id) else {
        return Work::BlockTask(format!("decision step not found: {step_id}"));
    };
    Work::CallModel {
        step_id,
        prompt: render_prompt_for_decision(&snapshot.task, &snapshot.steps, step, decision),
    }
}

fn step_operation(operation: &str, prefix: &str) -> Option<u64> {
    operation
        .strip_prefix(prefix)
        .and_then(|value| value.parse::<u64>().ok())
}

pub fn apply_turn(
    snapshot: &TaskSnapshot,
    work: &Work,
    outcome: TurnOutcome,
) -> (TaskSnapshot, Vec<Command>) {
    let mut next = snapshot.clone();
    let mut commands = Vec::new();
    match (work, outcome) {
        (Work::CallModel { step_id, prompt }, TurnOutcome::ParseFault(fault)) => {
            handle_fault(&mut next, &mut commands, *step_id, prompt, fault);
        }
        (Work::CallModel { step_id, prompt }, TurnOutcome::EndpointError(error)) => {
            handle_endpoint_error(&mut next, &mut commands, *step_id, prompt, &error);
        }
        (Work::CallModel { step_id, prompt }, TurnOutcome::Model(parsed)) => {
            next.task.budget_used = next.task.budget_used.saturating_add(1);
            handle_model(&mut next, &mut commands, *step_id, prompt, parsed);
        }
        (Work::RunChecks { step_id }, TurnOutcome::Checks(files, command_facts)) => {
            handle_checks(&mut next, &mut commands, *step_id, &files, &command_facts);
        }
        (Work::CloseTask, _) => close_task(&mut next, &mut commands),
        (Work::BlockTask(reason), _) => block_task(&mut next, &mut commands, reason),
        _ => {}
    }
    (next, commands)
}

fn deterministic(spec: &CheckSpec) -> bool {
    !matches!(spec, CheckSpec::Judged { .. })
}
