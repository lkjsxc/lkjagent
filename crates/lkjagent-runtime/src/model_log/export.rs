use std::fs;
use std::path::Path;

use crate::error::{RuntimeError, RuntimeResult};

pub(super) fn record_success_export(
    dir: &Path,
    id: &str,
    finish_reason: &str,
    latency_ms: i64,
) -> RuntimeResult<()> {
    let content = format!(
        "{{\"id\":\"{}\",\"status\":\"succeeded\",\"finish_reason\":\"{}\",\"latency_ms\":{},\"files\":[\"request.json\",\"authority.json\",\"response.json\",\"timing.json\"]}}\n",
        json_escape(id),
        json_escape(finish_reason),
        latency_ms
    );
    atomic_write(&dir.join("export.json"), &content)
}

pub(super) fn record_error_export(
    dir: &Path,
    id: &str,
    error_class: &str,
    latency_ms: i64,
) -> RuntimeResult<()> {
    let content = format!(
        "{{\"id\":\"{}\",\"status\":\"failed\",\"error_class\":\"{}\",\"latency_ms\":{},\"files\":[\"request.json\",\"authority.json\",\"errors.ndjson\"]}}\n",
        json_escape(id),
        json_escape(error_class),
        latency_ms
    );
    atomic_write(&dir.join("export.json"), &content)
}

fn atomic_write(path: &Path, content: &str) -> RuntimeResult<()> {
    let parent = path
        .parent()
        .ok_or_else(|| RuntimeError::Store("provider export path has no parent".to_string()))?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, content).map_err(io_error)?;
    fs::rename(&tmp, path).map_err(io_error)?;
    Ok(())
}

fn json_escape(value: &str) -> String {
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
