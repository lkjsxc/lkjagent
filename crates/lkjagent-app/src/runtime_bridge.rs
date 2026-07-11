use lkjagent_core::engine::{Command, Work};
use lkjagent_core::model::{Attempt, AttemptOutcome, Event, EventKind, StepState, TaskSnapshot};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_store::decision_rows::{
    insert_runtime_decision, next_decision_id, settle_decision, unfinished_decisions,
};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case};
use rusqlite::Connection;

use crate::recovery_bridge::{record_recovery_fact, recover_or_reuse};
use crate::runtime_projection::{ensure_runtime_cell, suppress_decision_cell};
use crate::snapshot_state::persist_snapshot_cell;

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
    let decision = select_runtime_decision(&state_snapshot, &id, context_frame_fingerprint, &[])
        .map_err(|error| error.message)?;
    insert_runtime_decision(conn, &decision, "pending", now).map_err(|error| error.to_string())?;
    Ok(decision)
}

pub fn settle_effect_error(
    conn: &mut Connection,
    snapshot: &TaskSnapshot,
    work: &Work,
    error: String,
    now: &str,
) -> Result<TaskSnapshot, String> {
    let mut failed = snapshot.clone();
    let mut commands = Vec::new();
    if let Work::CallModel { step_id, prompt } = work {
        if let Some(step) = failed.steps.iter_mut().find(|step| step.id == *step_id) {
            let ordinal = step.actions_used + step.attempts_used + 1;
            step.state = StepState::Active;
            step.attempts_used = step.attempts_used.saturating_add(1);
            failed.task.budget_used = failed.task.budget_used.saturating_add(1);
            let attempt = Attempt {
                step_id: *step_id,
                ordinal,
                prompt_fingerprint: prompt.fingerprint.clone(),
                outcome: AttemptOutcome::EffectError,
                diagnosis: error.clone(),
                tokens_in: 0,
                tokens_out: 0,
                cached_tokens: 0,
                cache_status: "unknown".to_string(),
            };
            failed.attempts.push(attempt.clone());
            commands.push(Command::RecordAttempt(attempt));
        }
    }
    let event = Event {
        kind: EventKind::Notice,
        content: format!("effect_error: {error}"),
    };
    failed.events.push(event.clone());
    commands.push(Command::RecordEvent(event));
    commit_turn(conn, &failed, &commands, now).map_err(|error| error.to_string())?;
    Ok(failed)
}

pub fn settle_effect_failure(
    conn: &mut Connection,
    snapshot: &TaskSnapshot,
    work: &Work,
    decision: &RuntimeDecision,
    error: String,
    now: &str,
) -> Result<TaskSnapshot, String> {
    let settled = settle_effect_error(conn, snapshot, work, error.clone(), now)?;
    record_recovery_fact(
        conn,
        &decision.case_id,
        &decision.id,
        "effect",
        &error,
        0,
        now,
    )?;
    persist_snapshot_cell(conn, &settled, now)?;
    settle_runtime_decision(conn, decision, "effect_error", now)?;
    Ok(settled)
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
