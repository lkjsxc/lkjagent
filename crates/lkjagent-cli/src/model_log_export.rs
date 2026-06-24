use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;
use crate::store::open_store;

pub(super) fn export_exchange(
    data_dir: &Path,
    case_id: &str,
    turn_id: i64,
) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let Some(detail) =
        lkjagent_store::provider_exchange::detail_for_case_turn(&conn, case_id, turn_id)?
    else {
        return Err(CliError::failure(format!(
            "provider_exchange_not_found case={case_id} turn={turn_id}"
        )));
    };
    let path = data_dir
        .join("logs/model/archive")
        .join(format!("case-{case_id}"));
    fs::create_dir_all(&path)?;
    copy_turn_files(data_dir, case_id, turn_id, &path)?;
    let file = path.join(format!("turn-{turn_id:06}.json"));
    fs::write(&file, replay_json(&detail))?;
    Ok(format!(
        "provider_exchange_export={}",
        file.to_string_lossy()
    ))
}

fn copy_turn_files(
    data_dir: &Path,
    case_id: &str,
    turn_id: i64,
    archive: &Path,
) -> Result<(), CliError> {
    let Some(source) = find_turn_dir(data_dir, case_id, turn_id)? else {
        return Ok(());
    };
    let files = archive.join("files");
    fs::create_dir_all(&files)?;
    for name in TURN_FILES {
        let source_file = source.join(name);
        if source_file.is_file() {
            fs::copy(&source_file, files.join(name))?;
        }
    }
    Ok(())
}

fn find_turn_dir(
    data_dir: &Path,
    case_id: &str,
    turn_id: i64,
) -> Result<Option<PathBuf>, CliError> {
    let root = data_dir.join("logs/model");
    if !root.is_dir() {
        return Ok(None);
    }
    for entry in fs::read_dir(root)? {
        let candidate = entry?
            .path()
            .join(format!("case-{case_id}"))
            .join(format!("turn-{turn_id:06}"));
        if candidate.is_dir() {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}

const TURN_FILES: &[&str] = &[
    "request.json",
    "authority.json",
    "response.json",
    "timing.json",
    "errors.ndjson",
    "parsed-action.json",
    "admission.json",
    "observation.txt",
    "export.json",
];

fn replay_json(detail: &lkjagent_store::provider_exchange::ProviderExchangeDetail) -> String {
    format!(
        "{{\"id\":\"{}\",\"case\":\"{}\",\"turn\":{},\"status\":\"{}\",\"provider\":\"{}\",\"model\":\"{}\",\"request_json\":\"{}\",\"response_json\":{},\"usage_json\":{}}}\n",
        esc(&detail.row.id),
        esc(&detail.row.case_id),
        detail.row.turn_id,
        esc(&detail.row.status),
        esc(&detail.row.provider),
        esc(&detail.row.model),
        esc(&detail.request_json),
        opt_json(detail.response_json.as_deref()),
        opt_json(detail.usage_json.as_deref())
    )
}

fn opt_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), |value| format!("\"{}\"", esc(value)))
}

fn esc(value: &str) -> String {
    lkjagent_runtime::model_log::json_escape(value)
}
