#[path = "export_json.rs"]
mod export_json;

use std::fs;
use std::path::Path;

use crate::error::{RuntimeError, RuntimeResult};
use export_json::{existing_files, json_escape, refresh_content};

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
    let Some(next) = refresh_content(dir, &content, ALL_FILES) else {
        return Ok(());
    };
    atomic_write(&path, &next)
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

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Store(error.to_string())
}
