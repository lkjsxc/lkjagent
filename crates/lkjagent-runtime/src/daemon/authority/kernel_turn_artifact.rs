#[path = "kernel_turn_artifact_render.rs"]
mod render;

use lkjagent_store::artifact_ledger::{ArtifactLedgerRow, WeakPathRow};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::super::authority::RuntimeAuthoritySnapshot;
use crate::error::RuntimeResult;

pub(super) struct ArtifactSnapshotFields {
    pub artifact_id: Option<String>,
    pub root: Option<String>,
    pub kind: Option<String>,
    pub weak_paths: Vec<String>,
    pub audit_status: Option<String>,
    pub batch_cursor: Option<String>,
    pub latest_observation: Option<String>,
    pub plan_status: Option<String>,
    pub atom_total: usize,
    pub atom_ready: usize,
    pub atom_missing: usize,
    pub next_atom: Option<String>,
    pub next_path: Option<String>,
    pub active_contract: Option<String>,
    pub measured_total: usize,
    pub accepted_floor: usize,
    pub assembly_pending: bool,
    pub readiness: Option<String>,
    pub completion_blockers: Vec<String>,
}

pub(super) fn artifact_fields(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
) -> RuntimeResult<ArtifactSnapshotFields> {
    let Some(case_id) = snapshot.case_id else {
        return Ok(empty_artifact_fields());
    };
    let Some(row) = lkjagent_store::artifact_ledger::latest_for_case(conn, case_id)? else {
        return Ok(empty_artifact_fields());
    };
    let weak = lkjagent_store::artifact_ledger::weak_paths(conn, row.id)?;
    let cursor = lkjagent_store::artifact_cursor::latest_batch_cursor(conn, row.id)?;
    let readiness = lkjagent_store::artifact_graph::readiness_for_case(conn, case_id)?;
    let plan = lkjagent_store::artifact_graph::plan_for_root(conn, &row.root)?;
    let active = match plan.as_ref() {
        Some(plan) => lkjagent_store::artifact_graph::active_contract_for_plan(conn, plan.id)?,
        None => None,
    };
    let latest_observation = active
        .as_ref()
        .and_then(|contract| {
            plan.as_ref()
                .map(|plan| render::contract_observation(plan, contract))
        })
        .or_else(|| readiness.as_ref().map(render::readiness_observation))
        .or_else(|| {
            cursor
                .as_ref()
                .and_then(|cursor| render::cursor_observation(cursor, &row.kind))
        })
        .or_else(|| ledger_observation(&row, &weak));
    let batch_cursor = cursor.as_ref().map(|cursor| cursor.root.clone());
    let audit_status = audit_status(&row.topology_status, &row.readiness_status);
    Ok(ArtifactSnapshotFields {
        artifact_id: Some(row.artifact_id),
        root: Some(row.root),
        kind: Some(row.kind),
        weak_paths: weak.into_iter().map(|path| path.path).collect(),
        audit_status,
        batch_cursor,
        latest_observation,
        plan_status: readiness.as_ref().map(|row| row.plan_status.clone()),
        atom_total: readiness.as_ref().map_or(0, |row| row.atom_total as usize),
        atom_ready: readiness.as_ref().map_or(0, |row| row.atom_ready as usize),
        atom_missing: readiness
            .as_ref()
            .map_or(0, |row| row.atom_missing as usize),
        next_atom: readiness
            .as_ref()
            .and_then(|row| none_to_option(&row.next_atom_id)),
        next_path: readiness
            .as_ref()
            .and_then(|row| none_to_option(&row.next_path)),
        active_contract: readiness
            .as_ref()
            .and_then(|row| none_to_option(&row.active_contract_id)),
        measured_total: readiness
            .as_ref()
            .map_or(0, |row| row.measured_total as usize),
        accepted_floor: readiness
            .as_ref()
            .map_or(0, |row| row.accepted_floor as usize),
        assembly_pending: readiness
            .as_ref()
            .is_some_and(|row| row.assembly_pending == "true"),
        readiness: readiness.as_ref().map(|row| row.status.clone()),
        completion_blockers: readiness
            .as_ref()
            .map(|row| lkjagent_store::artifact_graph::split_lines(&row.completion_blockers))
            .unwrap_or_default(),
    })
}

fn empty_artifact_fields() -> ArtifactSnapshotFields {
    ArtifactSnapshotFields {
        artifact_id: None,
        root: None,
        kind: None,
        weak_paths: Vec::new(),
        audit_status: None,
        batch_cursor: None,
        latest_observation: None,
        plan_status: None,
        atom_total: 0,
        atom_ready: 0,
        atom_missing: 0,
        next_atom: None,
        next_path: None,
        active_contract: None,
        measured_total: 0,
        accepted_floor: 0,
        assembly_pending: false,
        readiness: None,
        completion_blockers: Vec::new(),
    }
}

fn none_to_option(value: &str) -> Option<String> {
    (value != "none" && !value.trim().is_empty()).then(|| value.to_string())
}

fn ledger_observation(row: &ArtifactLedgerRow, weak: &[WeakPathRow]) -> Option<String> {
    if row.readiness_status != "failed" || row.topology_status == "missing" {
        return None;
    }
    let failures = if row.kind == "story" {
        "- story_semantic_missing: ledger-readiness\n- story_scale_missing: ledger-readiness"
    } else {
        "- content_readiness_failed: ledger-readiness"
    };
    let weak_paths = weak
        .iter()
        .map(|path| path.path.as_str())
        .collect::<Vec<_>>()
        .join(",");
    Some(format!(
        "artifact audit failed\nroot={}\nreadiness=missing-semantic-content\nweak_paths={}\nfailures:\n{}\nnext_decision_required=true\ncandidate_action=artifact.next\nartifact_ledger_id={}",
        row.root, weak_paths, failures, row.id
    ))
}

fn audit_status(topology: &str, readiness: &str) -> Option<String> {
    match (topology, readiness) {
        ("missing", _) => Some("missing".to_string()),
        ("failed", _) => Some("failed".to_string()),
        ("passed", "passed") => Some("ready".to_string()),
        ("passed", _) => Some("passed".to_string()),
        (_, "failed") => Some("failed".to_string()),
        (_, "passed") => Some("ready".to_string()),
        _ => None,
    }
}

pub(super) fn next_snapshot_id(conn: &Connection) -> RuntimeResult<u64> {
    Ok(store_state::get(conn, "authority snapshot id")?
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        .saturating_add(1))
}
