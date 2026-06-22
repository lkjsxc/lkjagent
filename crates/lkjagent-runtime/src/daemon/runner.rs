use std::path::PathBuf;

use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_llm::client::ClientConfig;
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{DispatchState, ToolRuntime};
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};
use crate::mode::TurnAuthority;
use crate::task::{RuntimeState, TaskState, DEFAULT_TURN_BUDGET};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonTick {
    Idle,
    Working,
    Waiting,
    Done,
    EndpointError,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentRuntime {
    pub holder: String,
    pub client: ClientConfig,
    pub tools: ToolRuntime,
    pub budget: ContextBudgetPolicy,
    pub task_turn_budget: u16,
    pub model_log_path: Option<PathBuf>,
}

impl ResidentRuntime {
    pub fn new(holder: String, client: ClientConfig, workspace: PathBuf, now: &str) -> Self {
        Self {
            holder,
            client,
            tools: ToolRuntime::new(workspace, now),
            budget: ContextBudgetPolicy::default(),
            task_turn_budget: DEFAULT_TURN_BUDGET,
            model_log_path: None,
        }
    }

    pub fn with_budget(mut self, budget: ContextBudgetPolicy) -> Self {
        self.budget = budget;
        self
    }

    pub fn with_task_turn_budget(mut self, turn_budget: u16) -> Self {
        self.task_turn_budget = turn_budget.max(1);
        self
    }

    pub fn with_model_log_path(mut self, path: PathBuf) -> Self {
        self.model_log_path = Some(path);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentDaemon {
    pub state: RuntimeState,
    pub runtime: ResidentRuntime,
    pub dispatch_state: DispatchState,
    pub endpoint_attempt: u32,
    pub endpoint_retry_at: Option<String>,
    pub turn_authority: Option<TurnAuthority>,
}

impl ResidentDaemon {
    pub fn new(state: RuntimeState, runtime: ResidentRuntime) -> Self {
        Self {
            state,
            runtime,
            dispatch_state: DispatchState::default(),
            endpoint_attempt: 0,
            endpoint_retry_at: None,
            turn_authority: None,
        }
    }

    pub fn poll_once(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        self.runtime.tools.now = now.to_string();
        let tick = self.poll_once_inner(conn, now)?;
        self.write_model_log(conn, now)?;
        Ok(tick)
    }

    fn poll_once_inner(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        self.heartbeat(conn, now)?;
        if let Some(tick) = self.compact_before_owner(conn, now)? {
            return Ok(tick);
        }
        self.deliver_owner(conn, now)?;
        if self.state.maintenance.is_some()
            && matches!(self.state.task, TaskState::Idle | TaskState::Closed { .. })
        {
            return self.endpoint_turn(conn, now);
        }
        match &self.state.task {
            TaskState::Open { .. } => self.endpoint_turn(conn, now),
            TaskState::Waiting { .. } => {
                self.write_observable(conn)?;
                Ok(DaemonTick::Waiting)
            }
            TaskState::Paused { .. } => {
                self.write_observable(conn)?;
                Ok(DaemonTick::Paused)
            }
            TaskState::Idle | TaskState::Closed { .. } => {
                if let Some(tick) = self.open_idle_maintenance(conn, now)? {
                    return Ok(tick);
                }
                self.write_observable(conn)?;
                Ok(DaemonTick::Idle)
            }
        }
    }

    fn write_model_log(&self, conn: &Connection, now: &str) -> RuntimeResult<()> {
        if let Some(path) = &self.runtime.model_log_path {
            crate::model_log::write_current_log(conn, path, now, self.runtime.budget)?;
        }
        Ok(())
    }

    fn heartbeat(&self, conn: &Connection, now: &str) -> RuntimeResult<()> {
        if store_state::heartbeat_lock(conn, &self.runtime.holder, now)? {
            Ok(())
        } else {
            Err(RuntimeError::Store("daemon lock lost".to_string()))
        }
    }
}
