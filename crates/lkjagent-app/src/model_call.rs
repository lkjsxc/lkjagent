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
    pub cache_status: String,
    pub count_budget: bool,
    pub exchange_ref: String,
    pub outcome_json: String,
    pub timeout_seconds: Option<u64>,
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
    let timeout_seconds = endpoint.timeout_seconds();
    match endpoint.complete(prompt, step.attempts_used) {
        Ok(record) => {
            let outcome = parse_expected_for_decision(decision, &record.content)
                .map_or_else(TurnOutcome::ParseFault, TurnOutcome::Model);
            write_success(
                context(
                    ContextInput {
                        logs,
                        snapshot,
                        step_ordinal: step.ordinal,
                        attempt_ordinal: ordinal,
                        prompt,
                        decision,
                        timeout_seconds,
                    },
                    started,
                ),
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
                    cache_status: cache_status(&record),
                    count_budget: matches!(outcome, TurnOutcome::ParseFault(_)),
                    exchange_ref: exchange_ref(snapshot, step.ordinal, ordinal),
                    outcome_json: outcome_summary(&outcome),
                    timeout_seconds,
                }),
            ))
        }
        Err(error) => {
            write_error(
                context(
                    ContextInput {
                        logs,
                        snapshot,
                        step_ordinal: step.ordinal,
                        attempt_ordinal: ordinal,
                        prompt,
                        decision,
                        timeout_seconds,
                    },
                    started,
                ),
                &error,
            )?;
            let outcome = TurnOutcome::EndpointError(error);
            Ok((
                outcome.clone(),
                Some(CallRecord {
                    step_id,
                    tokens_in: None,
                    tokens_out: None,
                    cached_tokens: None,
                    cache_status: "unknown".to_string(),
                    count_budget: false,
                    exchange_ref: exchange_ref(snapshot, step.ordinal, ordinal),
                    outcome_json: outcome_summary(&outcome),
                    timeout_seconds,
                }),
            ))
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

struct ContextInput<'a> {
    logs: &'a Path,
    snapshot: &'a TaskSnapshot,
    step_ordinal: u32,
    attempt_ordinal: u32,
    prompt: &'a Prompt,
    decision: &'a RuntimeDecision,
    timeout_seconds: Option<u64>,
}

fn context<'a>(input: ContextInput<'a>, started: Instant) -> ExchangeContext<'a> {
    ExchangeContext {
        logs: input.logs,
        snapshot: input.snapshot,
        step_ordinal: input.step_ordinal,
        attempt_ordinal: input.attempt_ordinal,
        prompt: input.prompt,
        decision_id: input.decision.id.clone(),
        tool_view_fingerprint: input.decision.tool_view_fingerprint().unwrap_or_default(),
        context_frame_fingerprint: input.decision.context_frame_fingerprint.clone(),
        timeout_seconds: input.timeout_seconds,
        started,
    }
}

fn exchange_ref(snapshot: &TaskSnapshot, step_ordinal: u32, ordinal: u32) -> String {
    format!(
        "logs/matter-{}/operation-{}/attempt-{}",
        snapshot.task.id, step_ordinal, ordinal
    )
}

fn outcome_summary(outcome: &TurnOutcome) -> String {
    match outcome {
        TurnOutcome::Model(_) => serde_json::json!({"outcome":"parsed"}).to_string(),
        TurnOutcome::ParseFault(fault) => serde_json::json!({
            "outcome":"parse_fault",
            "diagnosis": format!("{fault:?}")
        })
        .to_string(),
        TurnOutcome::EndpointError(error) => serde_json::json!({
            "outcome":"endpoint_error",
            "diagnosis": error
        })
        .to_string(),
        other => serde_json::json!({"outcome": format!("{other:?}")}).to_string(),
    }
}

fn apply_tokens(attempt: &mut Attempt, record: &CallRecord) {
    attempt.tokens_in = record.tokens_in.unwrap_or(0);
    attempt.tokens_out = record.tokens_out.unwrap_or(0);
    attempt.cached_tokens = record.cached_tokens.unwrap_or(0);
    attempt.cache_status = record.cache_status.clone();
}

fn cache_status(record: &crate::model_io::CompletionRecord) -> String {
    if record.cached_tokens.is_some() {
        "known".to_string()
    } else if record.cache_metrics.is_empty() {
        "unknown".to_string()
    } else {
        "provider_specific".to_string()
    }
}
