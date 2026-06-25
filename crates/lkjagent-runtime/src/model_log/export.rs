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
        "{{\"id\":\"{}\",\"status\":\"succeeded\",\"finish_reason\":\"{}\",\"latency_ms\":{},\"files\":[{}]}}\n",
        json_escape(id),
        json_escape(finish_reason),
        latency_ms,
        existing_files(dir, SUCCESS_FILES),
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
        "{{\"id\":\"{}\",\"status\":\"failed\",\"error_class\":\"{}\",\"latency_ms\":{},\"files\":[{}]}}\n",
        json_escape(id),
        json_escape(error_class),
        latency_ms,
        existing_files(dir, ERROR_FILES),
    );
    atomic_write(&dir.join("export.json"), &content)
}

const SUCCESS_FILES: &[&str] = &[
    "request.json",
    "authority.json",
    "response.json",
    "timing.json",
    "parsed-action.json",
    "admission.json",
    "observation.txt",
];

const ERROR_FILES: &[&str] = &[
    "request.json",
    "authority.json",
    "errors.ndjson",
    "parsed-action.json",
];

const ALL_FILES: &[&str] = &[
    "request.json",
    "authority.json",
    "response.json",
    "timing.json",
    "errors.ndjson",
    "parsed-action.json",
    "admission.json",
    "observation.txt",
];

pub(super) fn refresh_export_files(dir: &Path) -> RuntimeResult<()> {
    let path = dir.join("export.json");
    if !path.is_file() {
        return Ok(());
    }
    let content = fs::read_to_string(&path).map_err(io_error)?;
    let Some(start) = content.find("\"files\":") else {
        return Ok(());
    };
    let Some(open) = content[start..].find('[').map(|index| start + index) else {
        return Ok(());
    };
    let Some(close) = content[open..].find(']').map(|index| open + index) else {
        return Ok(());
    };
    let next = format!(
        "{}[{}]{}",
        &content[..open],
        existing_files(dir, ALL_FILES),
        &content[close + 1..]
    );
    atomic_write(&path, &next)
}

fn existing_files(dir: &Path, candidates: &[&str]) -> String {
    candidates
        .iter()
        .filter(|file| dir.join(file).is_file())
        .map(|file| format!("\"{}\"", json_escape(file)))
        .collect::<Vec<_>>()
        .join(",")
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
