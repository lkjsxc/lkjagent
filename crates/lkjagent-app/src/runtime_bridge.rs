use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_store::decision_rows::{
    insert_runtime_decision, next_decision_id, settle_decision, unfinished_decisions,
};
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case};
use rusqlite::Connection;

use crate::recovery_bridge::recover_or_reuse;
use crate::runtime_projection::{ensure_runtime_cell, suppress_decision_cell};

pub fn prepare_runtime_decision(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    context_frame_fingerprint: &str,
    now: &str,
) -> Result<RuntimeDecision, String> {
    let case_id = snapshot.task.id.to_string();
    insert_case(conn, &case_id, &snapshot.task.objective, now)
        .map_err(|error| error.to_string())?;
    let unfinished = unfinished_decisions(conn, &case_id).map_err(|error| error.to_string())?;
    if let Some(decision) = recover_or_reuse(conn, &unfinished, now)? {
        return Ok(decision);
    }
    let mut state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    ensure_runtime_cell(conn, snapshot, &state_snapshot, now)?;
    state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    let id = next_decision_id(conn, &case_id).map_err(|error| error.to_string())?;
    let mut decision =
        select_runtime_decision(&state_snapshot, &id, &[]).map_err(|error| error.message)?;
    decision.context_frame_fingerprint = context_frame_fingerprint.to_string();
    insert_runtime_decision(conn, &decision, "pending", now).map_err(|error| error.to_string())?;
    Ok(decision)
}

pub fn settle_runtime_decision(
    conn: &Connection,
    decision: &RuntimeDecision,
    status: &str,
    now: &str,
) -> Result<(), String> {
    settle_decision(conn, &decision.id, status, now).map_err(|error| error.to_string())?;
    suppress_decision_cell(conn, decision, now)
}
