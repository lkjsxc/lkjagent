use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use rusqlite::Connection;
use serde_json::Value;

use super::attestation;
use super::io::{
    err, field, file_hash, hash, inside, pairs, raw_manifest, scenario_hash, table, valid_hash,
};
use super::Cell;

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
pub(super) fn validate(
    root: &Path,
    campaign: &Path,
    row: &BTreeMap<String, String>,
    cell: &Cell,
    baseline: &Value,
    source: &str,
    executable_hash: &str,
    scenario: &str,
    run_ids: &mut BTreeSet<String>,
    stores: &mut BTreeSet<String>,
    provider_controls: &mut BTreeSet<String>,
) -> Result<(), String> {
    let run_id = field(row, "run_id");
    if field(row, "source_commit") != source || field(row, "run_ref") != format!("runs/{run_id}")
        || !run_ids.insert(run_id.to_string()) { return Err(format!("{run_id} has false source, ref, or reused id")); }
    let run_dir = inside(campaign, field(row, "run_ref"))?;
    let local = table(&run_dir.join("matrix-row.tsv"))?;
    if local.len() != 1 || local[0] != *row { return Err(format!("{run_id} local matrix row mismatch")); }
    let mut expected = overlay(baseline.clone(), &cell.factors)?;
    expected.as_object_mut().ok_or("config is not an object")?.insert("workspace_root".into(), Value::String(run_dir.join("workspace").to_string_lossy().into()));
    let bytes = serde_json::to_string(&expected).map_err(err)?;
    if fs::read_to_string(run_dir.join("config.json")).map_err(err)? != bytes
        || field(row, "config_sha256") != hash(bytes.as_bytes())
    {
        return Err(format!("{run_id} config mismatch"));
    }
    if field(row, "scenario_sha256")
        != scenario_hash(&root.join("evaluation/scenarios").join(scenario))?
        || field(row, "executable_sha256") != executable_hash
    {
        return Err(format!("{run_id} input hash mismatch"));
    }
    let db = run_dir.join("run.sqlite3");
    if db.is_symlink() { return Err("database is a symlink".into()); }
    if !stores.insert(file_hash(&db)?) { return Err("copied campaign database".into()); }
    let conn = Connection::open(&db).map_err(err)?;
    let integrity: String = conn.query_row("PRAGMA integrity_check", [], |item| item.get(0)).map_err(err)?;
    if integrity != "ok" { return Err(format!("{run_id} backup integrity failed")); }
    let facts: (i64, i64, i64, i64, i64, i64, i64, i64) = conn
        .query_row(
            "SELECT
        (SELECT COUNT(*) FROM runtime_decisions),
        (SELECT COUNT(*) FROM provider_exchanges),
        (SELECT COUNT(*) FROM provider_exchanges WHERE finished_at IS NOT NULL
            AND outcome_json NOT LIKE '%endpoint_error%'),
        (SELECT COUNT(*) FROM provider_exchanges WHERE outcome_json LIKE '%parse_fault%'),
        (SELECT COUNT(*) FROM tool_admissions),
        (SELECT COUNT(*) FROM tool_admissions WHERE status = 'Admitted'),
        (SELECT COUNT(*) FROM observations),
        (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked')",
            [],
            |item| {
                Ok((
                    item.get(0)?,
                    item.get(1)?,
                    item.get(2)?,
                    item.get(3)?,
                    item.get(4)?,
                    item.get(5)?,
                    item.get(6)?,
                    item.get(7)?,
                ))
            },
        )
        .map_err(err)?;
    let (decisions, exchanges, real, _parse, _admissions, _admitted, observations, blockers) =
        facts;
    let (first_decision, first_outcome, first_ref): (String, String, String) = conn.query_row("SELECT decision_id, outcome_json, exchange_ref
        FROM provider_exchanges WHERE finished_at IS NOT NULL AND outcome_json NOT LIKE '%endpoint_error%'
        ORDER BY started_at, id LIMIT 1", [], |item| Ok((item.get(0)?, item.get(1)?, item.get(2)?))).map_err(err)?;
    let mut first_statement = conn.prepare("SELECT action_tool, status FROM tool_admissions
        WHERE decision_id = ?1 ORDER BY created_at, id").map_err(err)?;
    let first_actions = first_statement.query_map([&first_decision], |item| Ok((item.get::<_,String>(0)?, item.get::<_,String>(1)?)))
        .map_err(err)?.collect::<Result<Vec<_>,_>>().map_err(err)?;
    let first_admissions = first_actions.len(); let first_admitted = first_actions.iter().filter(|(_,status)| status=="Admitted").count();
    let first_action_text = first_actions.iter().map(|(tool,status)| format!("{tool}:{status}")).collect::<Vec<_>>().join(",");
    let first_parse = !first_outcome.contains("parse_fault");
    if decisions < 1 || exchanges != 1 || real != 1 {
        return Err(format!("{run_id} lacks real completed exchange"));
    }
    attestation::validate_exchange_logs(&conn, &run_dir, run_id)?;
    let response: Value = serde_json::from_str(&fs::read_to_string(run_dir.join("data").join(&first_ref).join("response.json")).map_err(err)?).map_err(err)?;
    let timing: Value = serde_json::from_str(&fs::read_to_string(run_dir.join("data").join(&first_ref).join("timing.json")).map_err(err)?).map_err(err)?;
    let recovery_events: i64 = conn.query_row("SELECT COUNT(*) FROM runtime_events WHERE kind LIKE '%recovery%'", [], |item| item.get(0)).map_err(err)?;
    let no_progress_events: i64 = conn.query_row("SELECT COUNT(*) FROM state_cells WHERE payload_schema='recovery.no-progress'", [], |item| item.get(0)).map_err(err)?;
    let expected_outcome = if !first_parse {
        "probe-parse-fault"
    } else if first_admissions > 0 && first_admitted == 0 {
        "probe-admission-rejected"
    } else if first_admitted > 0 {
        "probe-admitted"
    } else {
        "probe-message"
    };
    let material = format!("{expected_outcome}\0{first_outcome}\0{first_action_text}\0{observations}\0{blockers}\0{recovery_events}\0{no_progress_events}");
    if field(row, "outcome") != expected_outcome || field(row, "outcome_fingerprint") != hash(material.as_bytes()) {
        return Err(format!("{run_id} outcome mismatch"));
    }
    let provider = pairs(&run_dir.join("provider-manifest.tsv"))?;
    if field(&provider, "transport") != "http"
        || field(&provider, "real_requests").parse::<i64>().ok() != Some(exchanges)
        || !valid_hash(field(&provider, "endpoint_sha256"))
        || !valid_hash(field(&provider, "model_sha256"))
        || !valid_hash(field(&provider, "environment_sha256"))
    {
        return Err(format!("{run_id} provider manifest invalid"));
    }
    provider_controls.insert(format!(
        "{}:{}:{}:{}",
        field(&provider, "endpoint_sha256"),
        field(&provider, "model_sha256"),
        field(&provider, "environment_sha256"),
        field(&provider, "credential_present")
    ));
    let result = pairs(&run_dir.join("result.tsv"))?;
    if field(&result, "snapshot_method") != "sqlite-online-backup"
        || field(&result, "status") != "conditional"
        || field(&result, "reason") != "requires-fault-and-frozen-live-campaign"
        || field(&result, "source_commit") != source
        || field(&result, "run_id") != run_id
        || field(&result, "runner_log_sha256") != file_hash(&run_dir.join("runner.log"))?
        || !run_dir.join("runner-redacted.log").is_file()
    {
        return Err(format!("{run_id} result overclaims"));
    }
    let redacted_runner = fs::read_to_string(run_dir.join("runner-redacted.log")).map_err(err)?;
    if redacted_runner.lines().any(|line| !(line.is_empty() || line.starts_with("$ ") || line.starts_with("exit=")
        || line.starts_with("[redacted sha256:") && line.ends_with(']') && line.len()==82)) {
        return Err(format!("{run_id} runner redaction invalid")); }
    let metrics = pairs(&run_dir.join("metrics.tsv"))?;
    if field(&metrics, "provider_exchanges").parse::<i64>().ok() != Some(exchanges)
        || field(&metrics, "first_pass_parse").parse::<i64>().ok() != Some(i64::from(first_parse))
        || field(&metrics, "first_pass_admission") != if first_admissions == 0 { "not-applicable" }
            else if first_admitted > 0 { "1" } else { "0" }
        || field(&metrics, "endpoint_calls").parse::<i64>().ok() != Some(exchanges)
        || field(&metrics, "action_identity") != if first_action_text.is_empty() { "none" } else { first_action_text.as_str() }
        || field(&metrics, "prompt_tokens") != reported(response.pointer("/usage/prompt_tokens"))
        || field(&metrics, "completion_tokens") != reported(response.pointer("/usage/completion_tokens"))
        || field(&metrics, "cached_tokens") != reported(response.pointer("/usage/cached_tokens"))
        || field(&metrics, "duration_ms") != reported(timing.get("duration_ms"))
        || field(&metrics, "observations").parse::<i64>().ok() != Some(observations)
        || field(&metrics, "unexpected_blockers").parse::<i64>().ok() != Some(blockers)
        || field(&metrics, "recovery_events").parse::<i64>().ok() != Some(recovery_events)
        || field(&metrics, "no_progress_events").parse::<i64>().ok() != Some(no_progress_events)
        || field(&metrics, "recovery_factor_exercised") != if no_progress_events > 0 { "1" } else { "0" }
        || field(&metrics, "fault_schedule_exercised") != "0"
        || ["required_source_recall", "unsupported_claims", "repeated_failure", "recovery_time_ms",
            "primary_task_success", "semantic_checks"].iter().any(|key| field(&metrics, key) != "not-measured")
        || field(&metrics, "full_live_floor_measured") != "0"
    {
        return Err(format!("{run_id} metrics mismatch"));
    }
    attestation::validate_workspace(&conn, &run_dir, run_id)?;
    attestation::validate_exports(&conn, &run_dir, run_id)?;
    raw_manifest(&run_dir)?;
    Ok(())
}

fn reported(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_u64)
        .map_or_else(|| "not-reported".into(), |item| item.to_string())
}

#[rustfmt::skip]
fn overlay(mut base: Value, factors: &BTreeMap<String, Value>) -> Result<Value, String> {
    let map = base.as_object_mut().ok_or("baseline config is not an object")?;
    for (key, value) in factors { map.insert(key.clone(), value.clone()); }
    Ok(base)
}
