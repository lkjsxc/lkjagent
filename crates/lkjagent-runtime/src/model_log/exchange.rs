#[path = "exchange_support.rs"]
mod exchange_support;

use std::path::{Path, PathBuf};

use lkjagent_store::provider_exchange::{
    complete_exchange, fail_exchange, record_request, ProviderExchangeCompletion,
    ProviderExchangeFailure, ProviderExchangeRequest,
};
use rusqlite::Connection;

use crate::error::RuntimeResult;
use exchange_support::{atomic_write, sanitize_path_segment, stable_hash};

pub use exchange_support::json_escape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderLogContext {
    pub case_id: String,
    pub turn_id: i64,
    pub prompt_frame_id: Option<String>,
    pub authority_decision_id: Option<String>,
    pub provider: String,
    pub model: String,
    pub created_at: String,
    pub authority_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderLogHandle {
    pub id: String,
    pub dir: PathBuf,
}

pub fn record_provider_request(
    conn: &Connection,
    root: &Path,
    context: &ProviderLogContext,
    request_json: &str,
) -> RuntimeResult<ProviderLogHandle> {
    let request_hash = stable_hash(request_json);
    let id = exchange_id(context, &request_hash);
    let dir = exchange_dir(root, context);
    atomic_write(&dir.join("request.json"), request_json)?;
    atomic_write(&dir.join("authority.json"), &context.authority_json)?;
    record_request(
        conn,
        &ProviderExchangeRequest {
            id: &id,
            case_id: &context.case_id,
            turn_id: context.turn_id,
            prompt_frame_id: context.prompt_frame_id.as_deref(),
            authority_decision_id: context.authority_decision_id.as_deref(),
            admission_decision_id: None,
            provider: &context.provider,
            model: &context.model,
            created_at: &context.created_at,
            request_json,
            request_hash: &request_hash,
            redaction_schema_version: 1,
        },
    )?;
    Ok(ProviderLogHandle { id, dir })
}

pub fn record_provider_response(
    conn: &Connection,
    handle: &ProviderLogHandle,
    response_json: &str,
    finish_reason: &str,
    usage_json: Option<&str>,
    latency_ms: i64,
) -> RuntimeResult<()> {
    let response_hash = stable_hash(response_json);
    let provider_anomaly = response_json.contains("\"provider_anomaly\":{");
    let status = if provider_anomaly {
        "provider_anomaly"
    } else {
        "succeeded"
    };
    atomic_write(&handle.dir.join("response.json"), response_json)?;
    atomic_write(
        &handle.dir.join("timing.json"),
        &format!("{{\"latency_ms\":{latency_ms}}}\n"),
    )?;
    complete_exchange(
        conn,
        &ProviderExchangeCompletion {
            id: &handle.id,
            response_json,
            response_hash: &response_hash,
            finish_reason,
            usage_json,
            stats_json: None,
            latency_ms,
            status,
        },
    )?;
    if provider_anomaly {
        super::export::record_provider_anomaly_export(
            &handle.dir,
            &handle.id,
            finish_reason,
            latency_ms,
        )?;
    } else {
        super::export::record_success_export(&handle.dir, &handle.id, finish_reason, latency_ms)?;
    }
    Ok(())
}

pub fn record_provider_error(
    conn: &Connection,
    handle: &ProviderLogHandle,
    error_class: &str,
    message: &str,
    latency_ms: i64,
) -> RuntimeResult<()> {
    let error_json = format!(
        "{{\"error_class\":\"{}\",\"message\":\"{}\"}}\n",
        json_escape(error_class),
        json_escape(message)
    );
    let response_hash = stable_hash(&error_json);
    atomic_write(&handle.dir.join("errors.ndjson"), &error_json)?;
    fail_exchange(
        conn,
        &ProviderExchangeFailure {
            id: &handle.id,
            error_class,
            response_json: Some(&error_json),
            response_hash: Some(&response_hash),
            latency_ms,
        },
    )?;
    super::export::record_error_export(&handle.dir, &handle.id, error_class, latency_ms)?;
    Ok(())
}

fn exchange_dir(root: &Path, context: &ProviderLogContext) -> PathBuf {
    root.join("model")
        .join(day_segment(&context.created_at))
        .join(format!("case-{}", context.case_id))
        .join(format!("turn-{turn:06}", turn = context.turn_id))
}

fn day_segment(created_at: &str) -> String {
    format!("epoch-{}", sanitize_path_segment(created_at))
}

fn exchange_id(context: &ProviderLogContext, request_hash: &str) -> String {
    format!(
        "case-{}-turn-{}-{}",
        context.case_id, context.turn_id, request_hash
    )
}
