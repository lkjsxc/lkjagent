use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::super::authority::RuntimeAuthoritySnapshot;
use crate::error::RuntimeResult;
use crate::kernel::{RuntimeEvent, SnapshotAdapterInput};
use crate::mode::{ActiveMode, RuntimeSnapshot};

pub(super) fn adapter_input(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
) -> RuntimeResult<SnapshotAdapterInput> {
    let artifact = artifact_fields(conn, snapshot)?;
    Ok(SnapshotAdapterInput {
        snapshot_id: next_snapshot_id(conn)?,
        case_id: snapshot.case_id.map(|id| id.to_string()),
        graph_node: snapshot.graph_node.clone(),
        graph_phase: snapshot.graph_phase.clone(),
        active_mode_hint: store_state::get(conn, "authority active mode")?,
        queue_head: pending_queue_head(conn)?,
        pending_owner_count: pending_owner_count(snapshot),
        required_evidence: snapshot.required_evidence.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        existing_evidence: existing_evidence(snapshot),
        artifact_id: artifact.artifact_id,
        artifact_root: artifact.root.or_else(|| snapshot.artifact_root.clone()),
        artifact_kind: artifact.kind,
        artifact_batch_cursor: artifact.batch_cursor,
        artifact_weak_paths: artifact.weak_paths,
        artifact_audit_status: artifact.audit_status,
        latest_observation: artifact.latest_observation,
        context_hard_pressure: snapshot.compaction_required,
        maintenance_due: snapshot.maintenance_due,
        maintenance_active: snapshot.maintenance_active,
        provider_retry_count: u32::from(snapshot.endpoint_retry_pending),
        latest_decision_id: store_state::get(conn, "authority decision id")?,
        prompt_frame_fingerprint: store_state::get(conn, "authority prompt frame id")?,
        ..SnapshotAdapterInput::default()
    })
}

pub(super) fn event_for(snapshot: &RuntimeAuthoritySnapshot) -> RuntimeEvent {
    if snapshot.compaction_required {
        return RuntimeEvent::ContextPressureDetected;
    }
    if snapshot.recoverable_owner_case {
        return RuntimeEvent::TurnBudgetExhausted;
    }
    if completion_ready(snapshot) {
        return RuntimeEvent::CompletionRequested;
    }
    if snapshot.pending_owner_rows > 0 || snapshot.active_owner_case {
        return RuntimeEvent::OwnerMessageReceived;
    }
    if snapshot.maintenance_due || snapshot.maintenance_active {
        return RuntimeEvent::MaintenanceTick;
    }
    RuntimeEvent::CaseResumed
}

pub(super) fn mode_snapshot(
    snapshot: &RuntimeAuthoritySnapshot,
    mode: ActiveMode,
) -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mode: mode,
        case_id: snapshot.case_id.map(|id| id.to_string()),
        graph_node: snapshot.graph_node.clone(),
        graph_phase: snapshot.graph_phase.clone(),
        owner_work_exists: snapshot.pending_owner_rows > 0 || snapshot.active_owner_case,
        recovery_ladder_active: snapshot.recoverable_owner_case,
        context_pressure_active: snapshot.compaction_required,
        maintenance_eligible: maintenance_eligible(snapshot),
        required_evidence: snapshot.required_evidence.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        active_artifact: snapshot.artifact_root.clone(),
        last_tool_attempt: None,
        latest_fault: None,
        repeated_action: false,
        external_owner_input_required: false,
    }
}

struct ArtifactSnapshotFields {
    artifact_id: Option<String>,
    root: Option<String>,
    kind: Option<String>,
    weak_paths: Vec<String>,
    audit_status: Option<String>,
    batch_cursor: Option<String>,
    latest_observation: Option<String>,
}

fn artifact_fields(
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
        .and_then(|cursor| cursor_observation(cursor, &row.kind));
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

fn cursor_observation(
    cursor: &lkjagent_store::artifact_cursor::BatchCursorRow,
    kind: &str,
) -> Option<String> {
    if !cursor.completed_paths.trim().is_empty() || !cursor.failed_paths.trim().is_empty() {
        return None;
    }
    Some(format!(
        "artifact_next_result=root_missing\nroot={}\nkind={kind}\nmissing=root\nruntime_event=ArtifactRootMissing\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}",
        cursor.root, cursor.last_valid_example
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

fn existing_evidence(snapshot: &RuntimeAuthoritySnapshot) -> Vec<String> {
    snapshot
        .required_evidence
        .iter()
        .filter(|item| !snapshot.missing_evidence.contains(item))
        .cloned()
        .collect()
}

fn maintenance_eligible(snapshot: &RuntimeAuthoritySnapshot) -> bool {
    !snapshot.active_owner_case
        && snapshot.pending_owner_rows == 0
        && (snapshot.maintenance_due || snapshot.maintenance_active)
}

fn next_snapshot_id(conn: &Connection) -> RuntimeResult<u64> {
    Ok(store_state::get(conn, "authority snapshot id")?
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        .saturating_add(1))
}

fn pending_owner_count(snapshot: &RuntimeAuthoritySnapshot) -> usize {
    snapshot.pending_owner_rows
        + usize::from(snapshot.active_owner_case && snapshot.case_id.is_none())
}

fn pending_queue_head(conn: &Connection) -> RuntimeResult<Option<String>> {
    Ok(lkjagent_store::queue::list(conn)?
        .into_iter()
        .find(|row| row.status == "pending")
        .map(|row| row.id.to_string()))
}

fn completion_ready(snapshot: &RuntimeAuthoritySnapshot) -> bool {
    snapshot.active_owner_case && snapshot.missing_evidence.is_empty()
}
