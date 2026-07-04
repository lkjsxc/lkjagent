use std::path::Path;
use std::time::Instant;

use lkjagent_core::engine::{Command, TurnOutcome};
use lkjagent_core::model::{Attempt, TaskSnapshot};
use lkjagent_core::parse::parse_expected_for_decision;
use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::RuntimeDecision;

use crate::exchange_record::{write_error, write_success, ExchangeContext};
use crate::model_io::Endpoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRecord {
    pub step_id: u64,
    pub tokens_in: Option<u32>,
    pub tokens_out: Option<u32>,
    pub cached_tokens: Option<u32>,
    pub count_budget: bool,
}

pub fn call<E: Endpoint>(
    logs: &Path,
    snapshot: &TaskSnapshot,
    step_id: u64,
    prompt: &Prompt,
    decision: &RuntimeDecision,
    endpoint: &mut E,
) -> Result<(TurnOutcome, Option<CallRecord>), String> {
    let step = snapshot
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .ok_or_else(|| "active step missing".to_string())?;
    let ordinal = step.actions_used + step.attempts_used + 1;
    let started = Instant::now();
    match endpoint.complete(prompt, step.attempts_used) {
        Ok(record) => {
            let outcome = parse_expected_for_decision(decision, &record.content)
                .map_or_else(TurnOutcome::ParseFault, TurnOutcome::Model);
            write_success(
                context(logs, snapshot, step.ordinal, ordinal, prompt, started),
                &record,
                &outcome,
            )?;
            Ok((
                outcome.clone(),
                Some(CallRecord {
                    step_id,
                    tokens_in: record.prompt_tokens,
                    tokens_out: record.completion_tokens,
                    cached_tokens: record.cached_tokens,
                    count_budget: matches!(outcome, TurnOutcome::ParseFault(_)),
                }),
            ))
        }
        Err(error) => {
            write_error(
                context(logs, snapshot, step.ordinal, ordinal, prompt, started),
                &error,
            )?;
            Ok((TurnOutcome::EndpointError(error), None))
        }
    }
}

pub fn apply_record(snapshot: &mut TaskSnapshot, commands: &mut [Command], record: &CallRecord) {
    if record.count_budget {
        snapshot.task.budget_used = snapshot.task.budget_used.saturating_add(1);
    }
    if let Some(attempt) = snapshot
        .attempts
        .iter_mut()
        .rev()
        .find(|attempt| attempt.step_id == record.step_id)
    {
        apply_tokens(attempt, record);
        for command in commands.iter_mut() {
            if let Command::RecordAttempt(command_attempt) = command {
                if command_attempt.step_id == attempt.step_id
                    && command_attempt.ordinal == attempt.ordinal
                {
                    apply_tokens(command_attempt, record);
                }
            }
        }
    }
}

fn context<'a>(
    logs: &'a Path,
    snapshot: &'a TaskSnapshot,
    step_ordinal: u32,
    attempt_ordinal: u32,
    prompt: &'a Prompt,
    started: Instant,
) -> ExchangeContext<'a> {
    ExchangeContext {
        logs,
        snapshot,
        step_ordinal,
        attempt_ordinal,
        prompt,
        started,
    }
}

fn apply_tokens(attempt: &mut Attempt, record: &CallRecord) {
    attempt.tokens_in = record.tokens_in.unwrap_or(0);
    attempt.tokens_out = record.tokens_out.unwrap_or(0);
    attempt.cached_tokens = record.cached_tokens.unwrap_or(0);
}
