use crate::checks::{CommandFact, FileFact};
use crate::engine_checks::handle_checks;
use crate::engine_steps::{
    block_task, close_task, handle_endpoint_error, handle_fault, handle_model,
};
use crate::model::*;
use crate::parse::{Action, ParseFault, ParsedOutput};
use crate::render::{render_prompt, Prompt};

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
    RecordChecks {
        step_id: u64,
        results: Vec<CheckResult>,
    },
    AddSteps(Vec<Step>),
}

pub fn next_work(snapshot: &TaskSnapshot) -> Work {
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
