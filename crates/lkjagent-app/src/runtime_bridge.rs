use std::path::Path;

use lkjagent_core::engine::{Command, Work};
use lkjagent_core::model::{Attempt, AttemptOutcome, Event, EventKind, StepState, TaskSnapshot};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision_at;
use lkjagent_store::admission_rows::PreparedEffect;
use lkjagent_store::decision_rows::{
    insert_runtime_decision, next_decision_id, settle_decision, unfinished_decisions,
};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case};
use rusqlite::Connection;

use crate::admission_bridge::persist_tool_admissions;
use crate::effect_dispatch::DispatchFailure;
use crate::observation_bridge::{persist_observations, settle_dispatch_failure};
use crate::recovery_bridge::{
    record_command_recovery_facts, record_recovery_fact, recover_or_reuse,
};
use crate::runtime_projection::{ensure_runtime_cell, suppress_decision_cell};
use crate::snapshot_state::persist_snapshot_cell;
use crate::turn_effects::{settle_check_effects, PendingCheckEffect};

#[rustfmt::skip]
pub struct TurnSettlement<'a> {
    pub workspace: &'a Path, pub snapshot: &'a TaskSnapshot, pub next: &'a TaskSnapshot,
    pub work: &'a Work, pub decision: &'a RuntimeDecision, pub effects: &'a [PreparedEffect],
    pub checks: &'a [PendingCheckEffect], pub commands: &'a [Command], pub now: &'a str,
}

pub fn prepare_runtime_decision(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    context_frame_fingerprint: &str,
    now: &str,
) -> Result<Option<RuntimeDecision>, String> {
    let case_id = snapshot.task.id.to_string();
    insert_case(conn, &case_id, &snapshot.task.objective, now)
        .map_err(|error| error.to_string())?;
    crate::endpoint_recovery::release_changed_waits(conn, &case_id, now)?;
    let unfinished = unfinished_decisions(conn, &case_id).map_err(|error| error.to_string())?;
    let (interrupted, budget_blocked) = match recover_or_reuse(conn, &unfinished, now) {
        Ok(Some(decision)) => {
            if !crate::runtime_budget::enforce(conn, &case_id, now, &unfinished)? {
                return Ok(Some(decision));
            }
            (false, true)
        }
        Ok(None) => (
            false,
            crate::runtime_budget::enforce(conn, &case_id, now, &[])?,
        ),
        Err(reason) => {
            crate::exchange_bridge::block_interrupted_decisions(conn, &unfinished, &reason, now)?;
            (true, false)
        }
    };
    let mut state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    if !interrupted && !budget_blocked {
        ensure_runtime_cell(conn, snapshot, &state_snapshot, now)?;
    }
    state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    let id = next_decision_id(conn, &case_id).map_err(|error| error.to_string())?;
    let decision =
        select_runtime_decision_at(&state_snapshot, &id, context_frame_fingerprint, &[], now)
            .map_err(|error| error.message)?;
    if matches!(
        decision.operation.0.as_str(),
        "runtime.idle" | "runtime.wait"
    ) {
        return Ok(None);
    }
    insert_runtime_decision(conn, &decision, "pending", now).map_err(|error| error.to_string())?;
    Ok(Some(decision))
}

pub fn settle_effect_error(
    conn: &Connection,
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

pub enum AdmissionOutcome {
    Prepared(Vec<PreparedEffect>),
    Failed(TaskSnapshot),
}

#[rustfmt::skip]
pub fn prepare_turn_admissions(conn: &Connection, workspace: &Path, snapshot: &TaskSnapshot,
    work: &Work, decision: &RuntimeDecision, commands: &[Command], now: &str,
) -> Result<AdmissionOutcome, String> {
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    let prepared = match persist_tool_admissions(&tx, workspace, decision, commands, now) {
        Ok(prepared) => prepared,
        Err(error) if error.starts_with("admission persistence failed:") => return Err(error),
        Err(error) => {
            let settled = settle_failure_rows(&tx, snapshot, work, decision, error,
                ("admission", "admission_error"), now)?;
            tx.commit().map_err(|error| error.to_string())?;
            return Ok(AdmissionOutcome::Failed(settled));
        }
    };
    tx.commit().map_err(|error| error.to_string())?;
    Ok(AdmissionOutcome::Prepared(prepared))
}

#[rustfmt::skip]
fn settle_failure_rows(conn: &Connection, snapshot: &TaskSnapshot, work: &Work,
    decision: &RuntimeDecision, error: String, kind: (&str, &str),
    now: &str) -> Result<TaskSnapshot, String> {
    let settled = settle_effect_error(conn, snapshot, work, error.clone(), now)?;
    record_recovery_fact(conn, &decision.case_id, &decision.id, kind.0, &error, 0, now)?;
    persist_snapshot_cell(conn, &settled, now)?;
    settle_runtime_decision(conn, decision, kind.1, now)?;
    Ok(settled)
}

#[rustfmt::skip]
pub fn settle_dispatched_turn(conn: &Connection, turn: &TurnSettlement<'_>) -> Result<TaskSnapshot, String> {
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    settle_check_effects(&tx, turn.checks)?;
    let postcondition = persist_observations(&tx, turn.workspace, turn.decision,
        turn.next, turn.effects, turn.now)?;
    let settled = if let Some(error) = postcondition {
        settle_failure_rows(&tx, turn.snapshot, turn.work, turn.decision, error,
            ("effect", "effect_error"), turn.now)?
    } else {
        commit_turn(&tx, turn.next, turn.commands, turn.now).map_err(|error| error.to_string())?;
        record_command_recovery_facts(&tx, turn.next, turn.commands, &turn.decision.id, turn.now)?;
        persist_snapshot_cell(&tx, turn.next, turn.now)?;
        settle_runtime_decision(&tx, turn.decision, "settled", turn.now)?;
        turn.next.clone()
    };
    tx.commit().map_err(|error| error.to_string())?; Ok(settled)
}

#[rustfmt::skip]
pub fn settle_failed_dispatch(conn: &Connection, turn: &TurnSettlement<'_>,
    failure: &DispatchFailure) -> Result<TaskSnapshot, String> {
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    settle_check_effects(&tx, turn.checks)?;
    settle_dispatch_failure(&tx, turn.workspace, turn.decision, turn.next,
        turn.effects, failure, turn.now)?;
    let settled = settle_failure_rows(&tx, turn.snapshot, turn.work, turn.decision,
        failure.error.clone(), ("effect", "effect_error"), turn.now)?;
    tx.commit().map_err(|error| error.to_string())?; Ok(settled)
}

#[rustfmt::skip]
pub fn settle_runtime_decision(conn: &Connection, decision: &RuntimeDecision,
    status: &str, now: &str) -> Result<(), String> {
    crate::progress_bridge::record(conn, decision, now)?;
    let changed = settle_decision(conn, &decision.id, status, now).map_err(|error| error.to_string())?;
    if changed != 1 { return Err(format!("decision settlement updated {changed} rows")); }
    let actual: String = conn.query_row("SELECT status FROM runtime_decisions WHERE id = ?1",
        [&decision.id], |row| row.get(0)).map_err(|error| error.to_string())?;
    if actual != status { return Err(format!("decision settlement status remained {actual}")); }
    suppress_decision_cell(conn, decision, now)
}
