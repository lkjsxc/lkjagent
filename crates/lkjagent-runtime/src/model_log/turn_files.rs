use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_protocol::{parse_live_completion, Action, EnvelopeMode, ParseFault};
use rusqlite::Connection;

use super::{json_escape, ProviderLogHandle};
use crate::error::{RuntimeError, RuntimeResult};

pub fn record_parsed_action(
    handle: &ProviderLogHandle,
    content: &str,
    closure_mode: &str,
) -> RuntimeResult<()> {
    atomic_write(
        &handle.dir.join("parsed-action.json"),
        &parse_json(content, closure_mode),
    )
}

pub fn record_provider_admission(
    conn: &Connection,
    tool: &str,
    admitted: bool,
    reason: &str,
    example: &str,
) -> RuntimeResult<()> {
    let Some(dir) = latest_dir(conn)? else {
        return Ok(());
    };
    let json = format!(
        "{{\"tool\":\"{}\",\"admitted\":{},\"reason\":\"{}\",\"example\":\"{}\"}}\n",
        json_escape(tool),
        admitted,
        json_escape(reason),
        json_escape(example)
    );
    atomic_write(&dir.join("admission.json"), &json)
}

pub fn record_provider_observation(conn: &Connection, observation: &str) -> RuntimeResult<()> {
    let Some(dir) = latest_dir(conn)? else {
        return Ok(());
    };
    atomic_write(&dir.join("observation.txt"), observation)
}

fn parse_json(content: &str, closure_mode: &str) -> String {
    let outcome = parse_live_completion(content, Default::default());
    match (outcome.action, outcome.fault) {
        (Some(action), None) => action_json(
            content,
            closure_mode,
            outcome.envelope_mode,
            &outcome.normalized_text_hash,
            &action,
        ),
        (_, Some(fault)) => fault_json(
            content,
            closure_mode,
            outcome.envelope_mode,
            &outcome.normalized_text_hash,
            &fault,
        ),
        (None, None) => fault_json(
            content,
            closure_mode,
            outcome.envelope_mode,
            &outcome.normalized_text_hash,
            &ParseFault::MissingActionEnvelope,
        ),
    }
}

fn action_json(
    content: &str,
    closure_mode: &str,
    envelope_mode: EnvelopeMode,
    normalized_text_hash: &str,
    action: &Action,
) -> String {
    format!(
        "{{\"status\":\"ok\",\"closure_mode\":\"{}\",\"envelope_mode\":\"{:?}\",\"normalized_text_hash\":\"{}\",\"content_bytes\":{},\"tool\":\"{}\",\"params\":[{}]}}\n",
        json_escape(closure_mode),
        envelope_mode,
        json_escape(normalized_text_hash),
        content.len(),
        json_escape(&action.tool),
        action
            .params
            .iter()
            .map(|param| format!(
                "{{\"name\":\"{}\",\"bytes\":{}}}",
                json_escape(&param.name),
                param.value.len()
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn fault_json(
    content: &str,
    closure_mode: &str,
    envelope_mode: EnvelopeMode,
    normalized_text_hash: &str,
    fault: &ParseFault,
) -> String {
    format!(
        "{{\"status\":\"fault\",\"closure_mode\":\"{}\",\"envelope_mode\":\"{:?}\",\"normalized_text_hash\":\"{}\",\"content_bytes\":{},\"error\":\"{}\"}}\n",
        json_escape(closure_mode),
        envelope_mode,
        json_escape(normalized_text_hash),
        content.len(),
        json_escape(&format!("{fault:?}"))
    )
}

fn latest_dir(conn: &Connection) -> RuntimeResult<Option<PathBuf>> {
    Ok(lkjagent_store::state::get(conn, "provider exchange dir")?.map(PathBuf::from))
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

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Store(error.to_string())
}
