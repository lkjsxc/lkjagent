use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::super::authority::RuntimeAuthoritySnapshot;
use super::endpoint_decision;
use crate::error::RuntimeResult;
use crate::kernel::ToolName;
use crate::kernel_driver::KernelTurnRecord;

pub(super) fn persist_state(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
    record: &KernelTurnRecord,
    endpoint_retry_pending: bool,
) -> RuntimeResult<()> {
    let decision = &record.decision;
    store_state::set(
        conn,
        "authority snapshot id",
        &record.snapshot_id.to_string(),
    )?;
    store_state::set(conn, "authority event id", &record.event_id.to_string())?;
    store_state::set(
        conn,
        "authority decision id",
        &record.decision_id.to_string(),
    )?;
    set_optional(conn, "authority prompt frame id", record.prompt_frame_id)?;
    store_state::set(conn, "authority mission", decision.mission.as_str())?;
    store_state::set(conn, "authority active mode", decision.active_mode.as_str())?;
    store_state::set(
        conn,
        "authority endpoint decision",
        &format!(
            "{:?}",
            endpoint_decision(snapshot, decision, endpoint_retry_pending)
        ),
    )?;
    store_state::set(conn, "authority case id", &optional_i64(snapshot.case_id))?;
    store_state::set(
        conn,
        "authority node",
        optional(snapshot.graph_node.as_deref()),
    )?;
    store_state::set(
        conn,
        "authority phase",
        optional(snapshot.graph_phase.as_deref()),
    )?;
    store_state::set(
        conn,
        "authority evidence gaps",
        &join_strings(&decision.missing_evidence),
    )?;
    store_state::set(
        conn,
        "authority artifact root",
        optional(snapshot.artifact_root.as_deref()),
    )?;
    store_state::set(
        conn,
        "authority recovery route",
        optional(decision.recovery_plan.as_deref()),
    )?;
    store_state::set(
        conn,
        "authority allowed tools",
        &join_tools(&decision.admission_view.admitted_tools),
    )?;
    store_state::set(
        conn,
        "authority blocked tools",
        &join_tools(&decision.admission_view.blocked_tools),
    )?;
    store_state::set(
        conn,
        "authority next action",
        decision
            .admission_view
            .exact_next_action
            .as_deref()
            .unwrap_or("none"),
    )?;
    store_state::set(
        conn,
        "authority fingerprint",
        decision.authority_fingerprint.as_str(),
    )?;
    store_state::set(conn, "kernel mission", decision.mission.as_str())?;
    store_state::set(conn, "kernel active mode", decision.active_mode.as_str())?;
    store_state::set(conn, "kernel event id", &record.event_id.to_string())?;
    store_state::set(
        conn,
        "kernel authority fingerprint",
        decision.authority_fingerprint.as_str(),
    )?;
    store_state::set(
        conn,
        "kernel staleness fingerprint",
        decision.staleness_fingerprint.as_str(),
    )?;
    store_state::set(conn, "kernel shadow error", "none")?;
    Ok(())
}

fn set_optional(conn: &Connection, key: &str, value: Option<i64>) -> RuntimeResult<()> {
    store_state::set(
        conn,
        key,
        &value.map_or_else(|| "none".to_string(), |id| id.to_string()),
    )?;
    Ok(())
}

fn optional(value: Option<&str>) -> &str {
    value.unwrap_or("none")
}

fn optional_i64(value: Option<i64>) -> String {
    value.map_or_else(|| "none".to_string(), |id| id.to_string())
}

fn join_strings(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn join_tools(values: &[ToolName]) -> String {
    let names: Vec<&str> = values.iter().map(ToolName::as_str).collect();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(",")
    }
}
