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
        artifact_root: snapshot.artifact_root.clone(),
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
