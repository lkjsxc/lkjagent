use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_store::provider_exchange::{
    complete_exchange, fail_exchange, record_request, ProviderExchangeCompletion,
    ProviderExchangeFailure, ProviderExchangeRequest,
};
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};

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
        },
    )?;
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

fn atomic_write(path: &Path, content: &str) -> RuntimeResult<()> {
    let parent = path
        .parent()
        .ok_or_else(|| RuntimeError::Store("provider log path has no parent".to_string()))?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let tmp = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("log")
    ));
    fs::write(&tmp, content).map_err(io_error)?;
    fs::rename(&tmp, path).map_err(io_error)?;
    Ok(())
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn sanitize_path_segment(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

pub fn json_escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Store(error.to_string())
}
