use std::path::Path;
use std::time::Instant;

use lkjagent_core::engine::TurnOutcome;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::parse::parse_fault_diagnosis;
use lkjagent_core::render::Prompt;
use lkjagent_effects::exchange::{write_exchange, ExchangeFiles};

use crate::model_io::CompletionRecord;

pub struct ExchangeContext<'a> {
    pub logs: &'a Path,
    pub snapshot: &'a TaskSnapshot,
    pub step_ordinal: u32,
    pub attempt_ordinal: u32,
    pub prompt: &'a Prompt,
    pub decision_id: String,
    pub tool_view_fingerprint: String,
    pub context_frame_fingerprint: String,
    pub timeout_seconds: Option<u64>,
    pub started: Instant,
}

pub fn write_success(
    context: ExchangeContext<'_>,
    record: &CompletionRecord,
    outcome: &TurnOutcome,
) -> Result<(), String> {
    write_files(
        &context,
        Bodies {
            request: request_json(&context),
            response: response_json(record),
            outcome: outcome_json(outcome),
            timing: timing_json(context.started, context.timeout_seconds),
        },
    )
}

pub fn write_error(context: ExchangeContext<'_>, error: &str) -> Result<(), String> {
    write_files(
        &context,
        Bodies {
            request: request_json(&context),
            response: serde_json::json!({"error": error}).to_string(),
            outcome: serde_json::json!({"outcome":"endpoint_error","diagnosis": error}).to_string(),
            timing: timing_json(context.started, context.timeout_seconds),
        },
    )
}

struct Bodies {
    request: String,
    response: String,
    outcome: String,
    timing: String,
}

fn write_files(context: &ExchangeContext<'_>, bodies: Bodies) -> Result<(), String> {
    write_exchange(
        context.logs,
        context.snapshot.task.id,
        context.step_ordinal,
        context.attempt_ordinal,
        ExchangeFiles {
            request: &bodies.request,
            response: &bodies.response,
            outcome: &bodies.outcome,
            timing: &bodies.timing,
        },
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn request_json(context: &ExchangeContext<'_>) -> String {
    serde_json::json!({
        "fingerprint": context.prompt.fingerprint,
        "decision_id": context.decision_id,
        "tool_view_fingerprint": context.tool_view_fingerprint,
        "context_frame_fingerprint": context.context_frame_fingerprint,
        "timeout_seconds": context.timeout_seconds,
        "system": context.prompt.system,
        "user": context.prompt.user,
        "max_tokens": context.prompt.max_tokens,
        "stop": context.prompt.stop
    })
    .to_string()
}

fn response_json(record: &CompletionRecord) -> String {
    serde_json::json!({
        "content": record.content,
        "finish_reason": record.finish_reason,
        "closure_mode": record.closure_mode,
        "usage": {
            "prompt_tokens": record.prompt_tokens,
            "completion_tokens": record.completion_tokens,
            "cached_tokens": record.cached_tokens
        },
        "cache_metrics": record.cache_metrics,
        "anomaly": record.anomaly
    })
    .to_string()
}

fn outcome_json(outcome: &TurnOutcome) -> String {
    match outcome {
        TurnOutcome::Model(_) => serde_json::json!({"outcome":"parsed"}).to_string(),
        TurnOutcome::ParseFault(fault) => serde_json::json!({
            "outcome":"parse_fault",
            "diagnosis": parse_fault_diagnosis(fault)
        })
        .to_string(),
        other => serde_json::json!({"outcome": format!("{other:?}")}).to_string(),
    }
}

fn timing_json(started: Instant, timeout_seconds: Option<u64>) -> String {
    serde_json::json!({
        "duration_ms": started.elapsed().as_millis(),
        "timeout_seconds": timeout_seconds
    })
    .to_string()
}
