use std::path::Path;

use lkjagent_core::engine::{apply_turn, next_work_with_decision, TurnOutcome, Work};
use lkjagent_core::model::{TaskSnapshot, TaskState};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

use crate::admission_bridge::persist_tool_admissions;
use crate::clock::{Clock, SystemClock};
use crate::context_bridge::{prepare_prompt_context, snapshot_with_prompt_context};
use crate::daemon_intake::{idle_snapshot, load_runtime_snapshot};
use crate::effect_error::settle as settle_effect_error;
use crate::endpoint::LlmEndpoint;
use crate::exchange_bridge::persist_provider_exchange;
use crate::model_call::{apply_record, call};
use crate::observation_bridge::persist_observations;
use crate::prompt_bridge::persist_prompt_frame;
use crate::recovery_bridge::{record_command_recovery_facts, record_recovery_fact};
use crate::runtime_bridge::{prepare_runtime_decision, settle_runtime_decision};
use crate::snapshot_state::persist_snapshot_cell;
use crate::turn_effects::{dispatch_effects, gather_checks, tag_check_evidence};

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
    let db = data_dir.join("lkjagent.sqlite3");
    let workspace = crate::config::workspace_root(data_dir)?;
    let logs = data_dir.join("logs");
    crate::workspace_scaffold::ensure_root(&workspace)?;
    let mut conn = Connection::open(db).map_err(|error| error.to_string())?;
    setup(&conn).map_err(|error| error.to_string())?;
    let heartbeat = clock.now();
    crate::daemon_lock::claim(&conn, &heartbeat)?;
    let mut snapshot = match load_runtime_snapshot(&mut conn, data_dir, clock)? {
        Some(snapshot) if runnable(&snapshot) => snapshot,
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

fn runnable(snapshot: &TaskSnapshot) -> bool {
    matches!(snapshot.task.state, TaskState::Open | TaskState::Waiting)
}

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
            tag_check_evidence(conn, &mut next, &mut commands, &decision.id)?;
            if let Err(error) = persist_tool_admissions(conn, &decision, &commands, &now) {
                let settled = settle_effect_error(conn, &snapshot, &work, error.clone(), &now)?;
                record_recovery_fact(
                    conn,
                    &decision.case_id,
                    &decision.id,
                    "admission",
                    &error,
                    0,
                    &now,
                )?;
                persist_snapshot_cell(conn, &settled, &now)?;
                settle_runtime_decision(conn, &decision, "admission_error", &now)?;
                return Ok(settled);
            }
            if let Err(error) = dispatch_effects(conn, workspace, &mut next, &commands, &now) {
                let settled = settle_effect_error(conn, &snapshot, &work, error.clone(), &now)?;
                record_recovery_fact(
                    conn,
                    &decision.case_id,
                    &decision.id,
                    "effect",
                    &error,
                    0,
                    &now,
                )?;
                persist_snapshot_cell(conn, &settled, &now)?;
                settle_runtime_decision(conn, &decision, "effect_error", &now)?;
                return Ok(settled);
            }
            persist_observations(conn, &decision, &next, &commands, &now)?;
            commit_turn(conn, &next, &commands, &now).map_err(|error| error.to_string())?;
            record_command_recovery_facts(conn, &next, &commands, &decision.id, &now)?;
            persist_snapshot_cell(conn, &next, &now)?;
            settle_runtime_decision(conn, &decision, "settled", &now)?;
            return Ok(next);
        }
        Work::RunChecks { step_id } => gather_checks(workspace, &snapshot, *step_id)?,
        Work::CloseTask
        | Work::ResolveState
        | Work::RunNativeEffect(_)
        | Work::BlockTask(_)
        | Work::Wait => TurnOutcome::Noop,
    };
    let (mut next, mut commands) = apply_turn(&snapshot, &work, outcome);
    let now = clock.now();
    tag_check_evidence(conn, &mut next, &mut commands, &decision.id)?;
    persist_tool_admissions(conn, &decision, &commands, &now)?;
    if let Err(error) = dispatch_effects(conn, workspace, &mut next, &commands, &now) {
        let settled = settle_effect_error(conn, &snapshot, &work, error.clone(), &now)?;
        record_recovery_fact(
            conn,
            &decision.case_id,
            &decision.id,
            "effect",
            &error,
            0,
            &now,
        )?;
        persist_snapshot_cell(conn, &settled, &now)?;
        settle_runtime_decision(conn, &decision, "effect_error", &now)?;
        return Ok(settled);
    }
    persist_observations(conn, &decision, &next, &commands, &now)?;
    commit_turn(conn, &next, &commands, &now).map_err(|error| error.to_string())?;
    record_command_recovery_facts(conn, &next, &commands, &decision.id, &now)?;
    persist_snapshot_cell(conn, &next, &now)?;
    settle_runtime_decision(conn, &decision, "settled", &now)?;
    Ok(next)
}
