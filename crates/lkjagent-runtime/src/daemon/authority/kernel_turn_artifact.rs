use lkjagent_store::artifact_cursor::BatchCursorRow;
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
    let latest_observation = cursor
        .as_ref()
        .and_then(|cursor| cursor_observation(cursor, &row.kind))
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
    }
}

fn cursor_observation(cursor: &BatchCursorRow, kind: &str) -> Option<String> {
    let planned = split_paths(&cursor.planned_paths);
    if planned.is_empty() {
        return None;
    }
    let completed = split_paths(&cursor.completed_paths);
    let failed = split_paths(&cursor.failed_paths);
    let remaining = planned
        .iter()
        .filter(|path| !completed.contains(path) && !failed.contains(path))
        .cloned()
        .collect::<Vec<_>>();
    if remaining.is_empty() {
        return Some(format!(
            "artifact_next_result=ready_for_audit\nroot={}\nkind={kind}\nmissing=0\nnext_decision_required=true\ncandidate_action=artifact.audit",
            cursor.root
        ));
    }
    let contract = single_path_contract(&cursor.root, kind, &remaining[0]);
    Some(format!(
        "artifact_next_result=write_contract_pending\nroot={}\nkind={kind}\nmissing={}\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}",
        cursor.root,
        remaining.len(),
        contract
    ))
}

fn split_paths(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn single_path_contract(root: &str, kind: &str, path: &str) -> String {
    if path.contains("/manuscript/") || path.starts_with("manuscript/") {
        return format!(
            "tool=fs.batch_write\nroot={root}\nkind=story\npaths:\n- {path}\nlimits:\n- max_files=1\n- max_file_bytes=12000\n- max_batch_bytes=12000\nrequired_sections:\n- finished chapter prose\n- scene action and dialogue or interiority\n- continuity with prior facts\nforbidden_weak_phrase_classes:\n- scaffold-only\n- outline-only\n- story-bible-only\n- placeholder\n- owner-terms-only\n- generic-example\nmodel_instruction=author finished manuscript prose for only this chapter path with the line protocol"
        );
    }
    format!(
        "tool=fs.batch_write\nroot={root}\nkind={kind}\npaths:\n- {path}\nlimits:\n- max_files=1\n- max_file_bytes=1800\n- max_batch_bytes=1800\nrequired_sections:\n- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes\nforbidden_weak_phrase_classes:\n- scaffold-only\n- placeholder\n- owner-terms-only\n- generic-example\nmodel_instruction=author only this one listed path with 25 to 45 words and the line protocol"
    )
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
