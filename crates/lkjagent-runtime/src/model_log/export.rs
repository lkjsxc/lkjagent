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
    let previous_files = listed_files(&content);
    let stripped = remove_array_field(&content, "missing_files");
    let Some((open, close)) = array_bounds(&stripped, "files") else {
        return Ok(());
    };
    let next = format!(
        "{}[{}]{}",
        &stripped[..open],
        existing_files(dir, ALL_FILES),
        &stripped[close + 1..]
    );
    atomic_write(
        &path,
        &insert_missing_files(&next, &missing_files(dir, &previous_files)),
    )
}

fn existing_files(dir: &Path, candidates: &[&str]) -> String {
    candidates
        .iter()
        .filter(|file| dir.join(file).is_file())
        .map(|file| format!("\"{}\"", json_escape(file)))
        .collect::<Vec<_>>()
        .join(",")
}

fn missing_files(dir: &Path, previous_files: &[String]) -> String {
    previous_files
        .iter()
        .filter(|file| !dir.join(file).is_file())
        .map(|file| {
            format!(
                "{{\"path\":\"{}\",\"reason\":\"listed_file_absent\"}}",
                json_escape(file)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn listed_files(content: &str) -> Vec<String> {
    let Some((open, close)) = array_bounds(content, "files") else {
        return Vec::new();
    };
    content[open + 1..close]
        .split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, value)| value.to_string())
        .collect()
}

fn array_bounds(content: &str, field: &str) -> Option<(usize, usize)> {
    let marker = format!("\"{field}\":");
    let start = content.find(&marker)?;
    let open = content[start..].find('[')? + start;
    let close = content[open..].find(']')? + open;
    Some((open, close))
}

fn remove_array_field(content: &str, field: &str) -> String {
    let marker = format!(",\"{field}\":");
    let Some(start) = content.find(&marker) else {
        return content.to_string();
    };
    let Some((_, close)) = array_bounds(&content[start + 1..], field) else {
        return content.to_string();
    };
    format!("{}{}", &content[..start], &content[start + close + 2..])
}

fn insert_missing_files(content: &str, missing_files: &str) -> String {
    let newline = content.ends_with('\n');
    let trimmed = content.trim_end_matches('\n');
    let Some(end) = trimmed.rfind('}') else {
        return content.to_string();
    };
    let inserted = format!(
        "{},\"missing_files\":[{}]{}",
        &trimmed[..end],
        missing_files,
        &trimmed[end..]
    );
    if newline {
        format!("{inserted}\n")
    } else {
        inserted
    }
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
