use std::path::Path;

use lkjagent_core::engine::{apply_turn, next_work_with_decision, TurnOutcome, Work};
use lkjagent_core::model::{TaskSnapshot, TaskState};
use lkjagent_store::effect_recovery::recover_unsettled_effects;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

use crate::clock::{Clock, SystemClock};
use crate::context_bridge::{prepare_prompt_context, snapshot_with_prompt_context};
use crate::daemon_intake::{idle_snapshot, load_runtime_snapshot};
use crate::effect_dispatch::{dispatch_effects, mark_effects};
use crate::endpoint::LlmEndpoint;
use crate::exchange_bridge::{persist_prompt_frame, persist_provider_exchange};
use crate::model_call::{apply_record, call};
#[rustfmt::skip]
use crate::runtime_bridge::{prepare_runtime_decision, prepare_turn_admissions,
    settle_dispatched_turn, settle_failed_dispatch, AdmissionOutcome, TurnSettlement};
use crate::turn_effects::{gather_checks, tag_check_evidence};

pub use crate::model_io::{CompletionRecord, Endpoint, ScriptedEndpoint};
pub fn run_daemon(data_dir: &Path) -> Result<(), String> {
    let mut endpoint = LlmEndpoint::new(data_dir);
    loop {
        let snapshot = run_until_idle(data_dir, &mut endpoint, 1)?;
        if !matches!(snapshot.task.state, TaskState::Open) {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

pub fn run_until_idle<E: Endpoint>(
    data_dir: &Path,
    endpoint: &mut E,
    max_turns: usize,
) -> Result<TaskSnapshot, String> {
    let mut clock = SystemClock;
    run_until_idle_with_clock(data_dir, endpoint, max_turns, &mut clock)
}

pub fn run_until_idle_with_clock<E: Endpoint, C: Clock>(
    data_dir: &Path,
    endpoint: &mut E,
    max_turns: usize,
    clock: &mut C,
) -> Result<TaskSnapshot, String> {
    let workspace = crate::config::workspace_root(data_dir)?;
    let logs = data_dir.join("logs");
    let mut conn =
        Connection::open(data_dir.join("lkjagent.sqlite3")).map_err(|error| error.to_string())?;
    setup(&conn).map_err(|error| error.to_string())?;
    let now = clock.now();
    crate::daemon_lock::claim(&mut conn, &now)?;
    let groups = lkjagent_store::workspace_rows::active_rebalance_groups(&conn)
        .map_err(|error| error.to_string())?;
    if groups.len() > 1 {
        return Err("multiple active rebalance groups".to_string());
    }
    for operation in groups {
        crate::workspace_rebalance_group::recover_startup(&conn, data_dir, &operation, &now)?;
    }
    for operation in lkjagent_store::workspace_rows::prepared_operations(&conn)
        .map_err(|error| error.to_string())?
    {
        if operation.kind == "rebalance" {
            crate::workspace_rebalance_apply::recover_prepared(&conn, data_dir, &operation, &now)?;
            continue;
        }
        if operation.kind != "archive" {
            continue;
        }
        let id = serde_json::from_str::<serde_json::Value>(&operation.preimage_json)
            .ok()
            .and_then(|value| value.get("id")?.as_str().map(str::to_string))
            .ok_or_else(|| format!("workspace operation {} has no record id", operation.id))?;
        crate::record_archive::archive(&conn, data_dir, &id, &now)?;
    }
    crate::workspace_scaffold::ensure_root(&workspace)?;
    recover_unsettled_effects(&mut conn, &workspace, &now).map_err(|error| error.to_string())?;
    let _reconciled = crate::workspace_search::reconcile_entry(&conn, &workspace, data_dir)?;
    let mut snapshot = match load_runtime_snapshot(&mut conn, data_dir, clock)? {
        Some(snapshot) if matches!(snapshot.task.state, TaskState::Open | TaskState::Waiting) => {
            snapshot
        }
        Some(snapshot) => return Ok(snapshot),
        None => return Ok(idle_snapshot()),
    };
    for _ in 0..max_turns {
        if !matches!(snapshot.task.state, TaskState::Open) {
            break;
        }
        snapshot = run_turn(&mut conn, &workspace, &logs, snapshot, endpoint, clock)?;
    }
    Ok(snapshot)
}

#[rustfmt::skip]
fn run_turn<E: Endpoint, C: Clock>(
    conn: &mut Connection,
    workspace: &Path,
    logs: &Path,
    snapshot: TaskSnapshot,
    endpoint: &mut E,
    clock: &mut C,
) -> Result<TaskSnapshot, String> {
    let selected_at = clock.now();
    let context = prepare_prompt_context(conn, &snapshot, &selected_at)?;
    let decision = prepare_runtime_decision(conn, &snapshot, &context.fingerprint, &selected_at)?;
    let prompt_snapshot = snapshot_with_prompt_context(&snapshot, &context);
    let work = next_work_with_decision(&prompt_snapshot, &decision);
    let mut check_effects = Vec::new();
    let mut check_refs = None;
    let outcome = match &work {
        Work::CallModel { step_id, prompt } => {
            persist_prompt_frame(conn, logs, &decision, prompt, &context, &selected_at)?;
            let (outcome, record) = call(logs, &snapshot, *step_id, prompt, &decision, endpoint)?;
            let (mut next, mut commands) = apply_turn(&snapshot, &work, outcome);
            let now = clock.now();
            if let Some(record) = &record {
                apply_record(&mut next, &mut commands, record);
                persist_provider_exchange(conn, &decision, record, &selected_at, &now)?;
            }
            tag_check_evidence(conn, &mut next, &mut commands, &decision.id, None)?;
            let prepared = match prepare_turn_admissions(
                conn, workspace, &snapshot, &work, &decision, &commands, &now,
            )? {
                AdmissionOutcome::Prepared(prepared) => prepared,
                AdmissionOutcome::Failed(settled) => return Ok(settled),
            };
            mark_effects(conn, &prepared, "applying", &now)?;
            let dispatch = dispatch_effects(conn, workspace, &mut next, &commands, &prepared);
            let turn = TurnSettlement {
                workspace, snapshot: &snapshot, next: &next, work: &work, decision: &decision,
                effects: &prepared, checks: &[], commands: &commands, now: &now,
            };
            return match dispatch {
                Ok(()) => settle_dispatched_turn(conn, &turn),
                Err(failure) => settle_failed_dispatch(conn, &turn, &failure),
            };
        }
        Work::RunChecks { step_id } => {
            let gathered = gather_checks(conn, workspace, &snapshot, *step_id, &decision, &clock.now())?;
            check_effects = gathered.effects;
            check_refs = Some(gathered.refs);
            gathered.outcome
        }
        Work::CloseTask
        | Work::ResolveState
        | Work::RunNativeEffect(_)
        | Work::BlockTask(_)
        | Work::Wait => TurnOutcome::Noop,
    };
    let (mut next, mut commands) = apply_turn(&snapshot, &work, outcome);
    let now = clock.now();
    tag_check_evidence(
        conn, &mut next, &mut commands, &decision.id, check_refs.as_deref(),
    )?;
    let prepared = if matches!(&work, Work::RunChecks { .. }) {
        Vec::new()
    } else {
        match prepare_turn_admissions(
            conn, workspace, &snapshot, &work, &decision, &commands, &now,
        )? {
            AdmissionOutcome::Prepared(prepared) => prepared,
            AdmissionOutcome::Failed(settled) => return Ok(settled),
        }
    };
    if !prepared.is_empty() { mark_effects(conn, &prepared, "applying", &now)?; }
    let dispatch = dispatch_effects(conn, workspace, &mut next, &commands, &prepared);
    let turn = TurnSettlement {
        workspace, snapshot: &snapshot, next: &next, work: &work, decision: &decision,
        effects: &prepared, checks: &check_effects, commands: &commands, now: &now,
    };
    match dispatch {
        Ok(()) => settle_dispatched_turn(conn, &turn),
        Err(failure) => settle_failed_dispatch(conn, &turn, &failure),
    }
}
