use lkjagent_context::assemble::assemble_messages;
use lkjagent_context::model::{ContextState, Frame, FrameKind, NoticeKind};
use lkjagent_llm::client::{complete, ClientConfig};
use lkjagent_llm::wire::Completion;
use lkjagent_protocol::Action;
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::state::{take_lock, LockDecision};
use lkjagent_tools::dispatch::{dispatch, DispatchOutput, DispatchState, ToolRuntime};
use rusqlite::Connection;
use std::time::Duration;

use crate::error::RuntimeResult;
use crate::prompt::token_estimate;
use crate::task::{RuntimeState, TaskState};

mod authority;
mod authority_ledger;
mod authority_store;
mod compaction;
mod compaction_support;
mod context_budget;
mod count_scaffold;
mod count_scaffold_gate;
mod effects;
mod effects_graph;
mod endpoint;
mod execute_pending;
mod graph_sync;
mod idle;
mod maintenance_wait;
mod owner_delivery;
mod persisted;
mod pressure;
mod record;
mod runner;
mod scaffold;
mod scaffold_evidence;
mod startup;
mod status;
mod task_summary;

pub use persisted::restore_completion_guard;
pub use runner::{DaemonTick, ResidentDaemon, ResidentRuntime};
pub use startup::{build_prefix_from_store, startup_summary};

pub type EndpointClientConfig = ClientConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupLock {
    Taken,
    Refused { holder: String },
    Reclaimed { previous: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Interrupt,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownState {
    pub stop_requested: bool,
    pub in_flight: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownDecision {
    Continue,
    FinishTurnThenExit,
    ExitNow,
}

pub fn take_daemon_lock(
    conn: &Connection,
    holder: &str,
    started_at: &str,
    stale_before: &str,
) -> RuntimeResult<StartupLock> {
    match take_lock(conn, holder, started_at, stale_before)? {
        LockDecision::Taken => Ok(StartupLock::Taken),
        LockDecision::Refused { holder } => Ok(StartupLock::Refused { holder }),
        LockDecision::Reclaimed { previous } => {
            append_event(
                conn,
                None,
                EventKind::Notice,
                &format!("reclaimed stale daemon lock from {previous}"),
                32,
                started_at,
            )?;
            Ok(StartupLock::Reclaimed { previous })
        }
    }
}

pub fn startup_state(prefix: Vec<Frame>, task_summary: Option<String>) -> RuntimeState {
    startup_state_with_budget(prefix, task_summary, crate::task::DEFAULT_TURN_BUDGET)
}

pub fn startup_state_with_budget(
    prefix: Vec<Frame>,
    task_summary: Option<String>,
    task_turn_budget: u16,
) -> RuntimeState {
    let mut state = RuntimeState::new(ContextState::new(prefix, Vec::new()));
    state.continuation_epoch.checkpoint_turns = task_turn_budget.max(1);
    if let Some(summary) = task_summary {
        let frame = Frame::new(
            FrameKind::Notice(NoticeKind::Compaction),
            summary.clone(),
            token_estimate(&summary),
        );
        state.context.log.push(frame);
        state.task = TaskState::Open {
            turns_remaining: task_turn_budget.max(1),
        };
    }
    state
}

pub fn request_shutdown(
    state: ShutdownState,
    _signal: Signal,
) -> (ShutdownState, ShutdownDecision) {
    let next = ShutdownState {
        stop_requested: true,
        in_flight: state.in_flight,
    };
    let decision = if state.in_flight {
        ShutdownDecision::FinishTurnThenExit
    } else {
        ShutdownDecision::ExitNow
    };
    (next, decision)
}

pub fn endpoint_complete(
    config: &ClientConfig,
    context: &ContextState,
    attempt: u32,
) -> RuntimeResult<Completion> {
    let messages = assemble_messages(context);
    Ok(complete(config, &messages, attempt)?)
}

pub fn client_config(
    base_url: &str,
    model: &str,
    api_key: Option<String>,
    timeout_seconds: u64,
    max_tokens: u16,
) -> ClientConfig {
    let mut config = ClientConfig::new(base_url, model);
    config.api_key = api_key;
    config.timeout = Duration::from_secs(timeout_seconds);
    config.max_tokens = max_tokens;
    config
}

pub fn execute_tool(
    action: &Action,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    dispatch(action, runtime, conn, state)
}
