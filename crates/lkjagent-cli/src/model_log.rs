use std::fs;
use std::path::Path;

use crate::args::ModelLogCommand;
use crate::config::load_context_policy_for_status;
use crate::error::CliError;
use crate::store::{now_stamp, open_store};

pub fn model_log(data_dir: &Path, command: ModelLogCommand) -> Result<String, CliError> {
    match command {
        ModelLogCommand::Current { print } => current_model_log(data_dir, print),
        ModelLogCommand::List { limit } => list_exchanges(data_dir, limit),
        ModelLogCommand::Show { case_id, turn_id } => show_exchange(data_dir, &case_id, turn_id),
        ModelLogCommand::Export { case_id, turn_id } => {
            export_exchange(data_dir, &case_id, turn_id)
        }
        ModelLogCommand::RawCase { case_id, limit } => raw_case(data_dir, &case_id, limit),
    }
}

fn current_model_log(data_dir: &Path, print: bool) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let path = lkjagent_runtime::model_log::current_log_path(data_dir);
    let policy = load_context_policy_for_status(data_dir)?;
    lkjagent_runtime::model_log::write_current_log(&conn, &path, &now_stamp(), policy)?;
    if print {
        fs::read_to_string(path).map_err(Into::into)
    } else {
        Ok(format!("model_log={}", path.to_string_lossy()))
    }
}

fn list_exchanges(data_dir: &Path, limit: usize) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let rows = lkjagent_store::provider_exchange::list_recent(&conn, limit)?;
    if rows.is_empty() {
        return Ok("provider_exchange=none".to_string());
    }
    Ok(rows
        .iter()
        .map(|row| {
            format!(
                "id={} case={} turn={} status={} model={} created_at={}",
                row.id, row.case_id, row.turn_id, row.status, row.model, row.created_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

fn export_exchange(data_dir: &Path, case_id: &str, turn_id: i64) -> Result<String, CliError> {
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
    let file = path.join(format!("turn-{turn_id:06}.json"));
    fs::write(&file, replay_json(&detail))?;
    Ok(format!(
        "provider_exchange_export={}",
        file.to_string_lossy()
    ))
}

fn raw_case(data_dir: &Path, case_id: &str, limit: usize) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let mut stmt = conn.prepare(
        "SELECT turn_id, status, model, created_at FROM provider_exchange
         WHERE case_id = ?1 ORDER BY turn_id DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map((case_id, limit as i64), |row| {
        Ok(format!(
            "case={case_id} turn={} status={} model={} created_at={}",
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?
        ))
    })?;
    let lines = rows.collect::<rusqlite::Result<Vec<_>>>()?;
    if lines.is_empty() {
        Ok(format!("provider_exchange_case={case_id} none"))
    } else {
        Ok(lines.join("\n"))
    }
}

fn show_exchange(data_dir: &Path, case_id: &str, turn_id: i64) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let Some(detail) =
        lkjagent_store::provider_exchange::detail_for_case_turn(&conn, case_id, turn_id)?
    else {
        return Err(CliError::failure(format!(
            "provider_exchange_not_found case={case_id} turn={turn_id}"
        )));
    };
    let mut out = format!(
        "id={}\ncase={}\nturn={}\nstatus={}\nprovider={}\nmodel={}\ncreated_at={}\nrequest_hash={}\n",
        detail.row.id,
        detail.row.case_id,
        detail.row.turn_id,
        detail.row.status,
        detail.row.provider,
        detail.row.model,
        detail.row.created_at,
        detail.row.request_hash
    );
    if let Some(hash) = &detail.row.response_hash {
        out.push_str(&format!("response_hash={hash}\n"));
    }
    if let Some(latency) = detail.latency_ms {
        out.push_str(&format!("latency_ms={latency}\n"));
    }
    out.push_str("request_json:\n");
    out.push_str(&detail.request_json);
    out.push_str("\nresponse_json:\n");
    out.push_str(detail.response_json.as_deref().unwrap_or("null"));
    if let Some(usage) = &detail.usage_json {
        out.push_str("\nusage_json:\n");
        out.push_str(usage);
    }
    Ok(out)
}

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
