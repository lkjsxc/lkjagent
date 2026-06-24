use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::RuntimeAuthoritySnapshot;
use crate::error::RuntimeResult;
use crate::kernel::{
    build_snapshot, reduce_with_event_id, RuntimeEvent, RuntimeEventId, SnapshotAdapterInput,
};

pub(super) fn persist_kernel_shadow(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
) -> RuntimeResult<()> {
    let input = adapter_input(conn, snapshot)?;
    let event = event_for(snapshot);
    match build_snapshot(input) {
        Ok(runtime_snapshot) => {
            let event_id = numeric_state(conn, "authority event id")?.unwrap_or(0);
            match reduce_with_event_id(&runtime_snapshot, RuntimeEventId(event_id), event) {
                Ok(decision) => persist_decision_fields(conn, &decision)?,
                Err(error) => store_state::set(conn, "kernel shadow error", &format!("{error:?}"))?,
            }
        }
        Err(error) => store_state::set(conn, "kernel shadow error", &format!("{error:?}"))?,
    }
    Ok(())
}

fn adapter_input(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
) -> RuntimeResult<SnapshotAdapterInput> {
    Ok(SnapshotAdapterInput {
        snapshot_id: numeric_state(conn, "authority snapshot id")?.unwrap_or(0),
        case_id: snapshot.case_id.map(|id| id.to_string()),
        graph_node: snapshot.graph_node.clone(),
        graph_phase: snapshot.graph_phase.clone(),
        pending_owner_count: snapshot.pending_owner_rows,
        required_evidence: snapshot.required_evidence.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        artifact_root: snapshot.artifact_root.clone(),
        context_hard_pressure: snapshot.compaction_required,
        maintenance_due: snapshot.maintenance_due,
        maintenance_active: snapshot.maintenance_active,
        latest_decision_id: store_state::get(conn, "authority decision id")?,
        prompt_frame_fingerprint: store_state::get(conn, "authority prompt frame id")?,
        ..SnapshotAdapterInput::default()
    })
}

fn event_for(snapshot: &RuntimeAuthoritySnapshot) -> RuntimeEvent {
    if snapshot.compaction_required {
        RuntimeEvent::ContextPressureDetected
    } else if snapshot.recoverable_owner_case {
        RuntimeEvent::TurnBudgetExhausted
    } else if snapshot.pending_owner_rows > 0 || snapshot.active_owner_case {
        RuntimeEvent::OwnerMessageReceived
    } else if snapshot.maintenance_due || snapshot.maintenance_active {
        RuntimeEvent::MaintenanceTick
    } else {
        RuntimeEvent::CaseResumed
    }
}

fn persist_decision_fields(
    conn: &Connection,
    decision: &crate::kernel::RuntimeDecision,
) -> RuntimeResult<()> {
    store_state::set(conn, "kernel mission", decision.mission.as_str())?;
    store_state::set(conn, "kernel active mode", decision.active_mode.as_str())?;
    store_state::set(conn, "kernel event id", &decision.event_id.0.to_string())?;
    store_state::set(conn, "kernel shadow error", "none")?;
    Ok(())
}

fn numeric_state(conn: &Connection, key: &str) -> RuntimeResult<Option<u64>> {
    Ok(store_state::get(conn, key)?.and_then(|value| value.parse().ok()))
}
