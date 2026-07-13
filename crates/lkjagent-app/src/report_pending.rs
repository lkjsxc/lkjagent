use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_store::error::{StoreError, StoreResult};
use rusqlite::{params, OptionalExtension, Transaction};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone)]
pub(crate) struct PendingState {
    pub slug: String,
    pub remaining_units: Vec<String>,
}

pub(crate) fn long_kind(kind: &str) -> bool {
    matches!(
        kind,
        "managed-report-map" | "managed-report-member" | "managed-report-complete"
    )
}

pub(crate) fn source_revision(
    tx: &Transaction<'_>,
    kind: &str,
    parameters: &[u8],
    default: &[u8],
) -> StoreResult<Vec<u8>> {
    if kind == "managed-report-complete" {
        aggregate_revision(tx, parameters)
    } else {
        Ok(default.to_vec())
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn evaluate(
    tx: &Transaction<'_>,
    matter: &str,
    decision: &str,
    kind: &str,
    parameters: &[u8],
    path: &str,
    bytes: &[u8],
    workspace: &OpenedWorkspace,
) -> StoreResult<Option<bool>> {
    Ok(match kind {
        "managed-report-map" => Some(crate::report_topology_checks::evaluate_map(
            tx, decision, parameters, path, bytes,
        )),
        "managed-report-member" => Some(crate::report_topology_checks::evaluate_member(
            tx, decision, parameters, path, bytes,
        )),
        "managed-report-complete" => Some(evaluate_complete(tx, matter, parameters, workspace)?),
        _ => None,
    })
}

pub(crate) fn pending_state(
    tx: &Transaction<'_>,
    matter: &str,
    event: &str,
    workspace: &OpenedWorkspace,
) -> StoreResult<Option<PendingState>> {
    let payload: Option<String> = tx.query_row(
        "SELECT CAST(predicate_payload AS TEXT) FROM obligations WHERE matter_id=?1 AND required=1 AND predicate_kind='managed-report-map' LIMIT 1",
        [matter], |row| row.get(0)).optional()?;
    let Some(payload) = payload else {
        return Ok(None);
    };
    let value: Value = serde_json::from_str(&payload).map_err(invalid)?;
    let slug = value["slug"]
        .as_str()
        .ok_or_else(invalid_static)?
        .to_string();
    let children = value["children"]
        .as_array()
        .ok_or_else(invalid_static)?
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    let (map_stale, stale) =
        crate::report_current::reopen_stale(tx, workspace, matter, event, &slug, &children)?;
    let mut remaining = Vec::new();
    if map_stale || !passed(tx, matter, &format!("report-map/{slug}"))? {
        remaining.push("index".into());
    }
    for child in children {
        if stale.contains(child) || !passed(tx, matter, &format!("report-member/{slug}/{child}"))? {
            remaining.push(child.to_string());
        }
    }
    if remaining.is_empty() && passed(tx, matter, &format!("report-complete/{slug}"))? {
        Ok(None)
    } else {
        Ok(Some(PendingState {
            slug,
            remaining_units: remaining,
        }))
    }
}

pub(crate) fn write_state(
    tx: &Transaction<'_>,
    matter: &str,
    event: &str,
    default_payload: &[u8],
    fingerprint: &[u8],
    pending: Option<PendingState>,
    current: bool,
) -> StoreResult<()> {
    tx.execute("UPDATE state_cells SET status='suppressed' WHERE matter_id=?1 AND ((namespace='check' AND cell_key IN ('current-passed','failed')) OR (namespace='report' AND cell_key='pending'))", [matter])?;
    if let Some(state) = pending {
        let payload =
            json!({"slug":state.slug,"remaining_units":state.remaining_units}).to_string();
        tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,'report','pending',?2,'active',?3,?4) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint", params![matter, payload.as_bytes(), event, sha(payload.as_bytes())])?;
    } else if current {
        tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,'check','current-passed',?2,'active',?3,?4) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint", params![matter, default_payload, event, fingerprint])?;
    }
    Ok(())
}

#[rustfmt::skip]
fn evaluate_complete(tx: &Transaction<'_>, matter: &str, parameters: &[u8], workspace: &OpenedWorkspace) -> StoreResult<bool> {
    if !crate::report_current::all_match(tx, workspace, parameters) {
        return Ok(false);
    }
    let value: Value = serde_json::from_slice(parameters).map_err(invalid)?;
    let slug = value["slug"].as_str().ok_or_else(invalid_static)?;
    let minimum_words = value["minimum_words"].as_u64().ok_or_else(invalid_static)? as usize;
    if !passed(tx, matter, &format!("report-map/{slug}"))? {
        return Ok(false);
    }
    let children = value["children"].as_array().ok_or_else(invalid_static)?;
    let mut seen = BTreeSet::new();
    let mut words = 0;
    for child in children {
        let unit = child["unit"].as_str().ok_or_else(invalid_static)?;
        let path = child["path"].as_str().ok_or_else(invalid_static)?;
        if !passed(tx, matter, &format!("report-member/{slug}/{unit}"))? {
            return Ok(false);
        }
        let Ok(text) = current_text(tx, path) else {
            return Ok(false);
        };
        let body = child_body(&text).ok_or_else(invalid_static)?;
        if body.trim().is_empty()
            || crate::journal_checks::known_placeholder(&body)
            || !seen.insert(body.trim().to_string())
        {
            return Ok(false);
        }
        words += body.split_whitespace().count();
    }
    Ok(words >= minimum_words)
}

fn aggregate_revision(tx: &Transaction<'_>, parameters: &[u8]) -> StoreResult<Vec<u8>> {
    let value: Value = serde_json::from_slice(parameters).map_err(invalid)?;
    let mut seed = String::new();
    for path in value["paths"]
        .as_array()
        .ok_or_else(invalid_static)?
        .iter()
        .filter_map(Value::as_str)
    {
        let revision: Option<String> = tx.query_row("SELECT current_revision_id FROM workspace_documents WHERE current_path=?1 AND managed=1", [path.as_bytes()], |row| row.get(0)).optional()?;
        seed.push_str(path);
        seed.push('@');
        seed.push_str(revision.as_deref().unwrap_or("missing"));
        seed.push('\n');
    }
    Ok(Sha256::digest(seed.as_bytes()).to_vec())
}

fn current_text(tx: &Transaction<'_>, path: &str) -> StoreResult<String> {
    let bytes: Vec<u8> = tx.query_row("SELECT r.content FROM workspace_documents d JOIN workspace_revisions r ON r.id=d.current_revision_id WHERE d.current_path=?1 AND d.managed=1", [path.as_bytes()], |row| row.get(0))?;
    String::from_utf8(bytes).map_err(|error| StoreError::InvalidState(error.to_string()))
}

fn child_body(text: &str) -> Option<String> {
    let lines = text.lines().collect::<Vec<_>>();
    let end = lines
        .iter()
        .enumerate()
        .skip(6)
        .find_map(|(n, line)| (*line == "---").then_some(n))?;
    Some(lines.get(end + 3..)?.join("\n").trim().to_string())
}

fn passed(tx: &Transaction<'_>, matter: &str, suffix: &str) -> StoreResult<bool> {
    Ok(tx.query_row("SELECT count(*) FROM obligations o JOIN checks c ON c.id=o.current_check_id WHERE o.id=?1 AND o.matter_id=?2 AND o.required=1 AND o.status='passed' AND c.current=1 AND c.passed=1", params![format!("{matter}/{suffix}"), matter], |row| row.get::<_, i64>(0))? == 1)
}
fn sha(bytes: &[u8]) -> Vec<u8> {
    Sha256::digest(bytes).to_vec()
}
fn invalid(error: impl std::fmt::Display) -> StoreError {
    StoreError::InvalidState(error.to_string())
}
fn invalid_static() -> StoreError {
    StoreError::InvalidState("report topology payload is malformed".into())
}
