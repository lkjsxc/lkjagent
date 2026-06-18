use std::path::PathBuf;

use lkjagent_llm::client::ClientConfig;
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{DispatchState, ToolRuntime};
use rusqlite::Connection;

use crate::daemon::endpoint_complete;
use crate::error::{RuntimeError, RuntimeResult};
use crate::intake;
use crate::prompt::token_estimate;
use crate::step::{step, StepInput};
use crate::task::{RuntimeState, TaskState};

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
}

impl ResidentRuntime {
    pub fn new(
        holder: String,
        client: ClientConfig,
        workspace: PathBuf,
        skill_library: PathBuf,
        now: &str,
    ) -> Self {
        Self {
            holder,
            client,
            tools: ToolRuntime::new(workspace, skill_library, now),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentDaemon {
    pub state: RuntimeState,
    pub runtime: ResidentRuntime,
    pub dispatch_state: DispatchState,
    pub endpoint_attempt: u32,
}

impl ResidentDaemon {
    pub fn new(state: RuntimeState, runtime: ResidentRuntime) -> Self {
        Self {
            state,
            runtime,
            dispatch_state: DispatchState::default(),
            endpoint_attempt: 0,
        }
    }

    pub fn poll_once(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        self.runtime.tools.now = now.to_string();
        self.heartbeat(conn, now)?;
        self.deliver_owner(conn, now)?;
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
                self.write_observable(conn)?;
                Ok(DaemonTick::Idle)
            }
        }
    }

    fn heartbeat(&self, conn: &Connection, now: &str) -> RuntimeResult<()> {
        if store_state::heartbeat_lock(conn, &self.runtime.holder, now)? {
            Ok(())
        } else {
            Err(RuntimeError::Store("daemon lock lost".to_string()))
        }
    }

    fn deliver_owner(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<()> {
        let tokens = next_owner_tokens(conn)?;
        let turn = self.state.turn.saturating_add(1);
        let Some(owner) = intake::deliver_next(conn, turn, tokens as i64, now)? else {
            return Ok(());
        };
        let starting_task = !matches!(
            self.state.task,
            TaskState::Open { .. } | TaskState::Waiting { .. }
        );
        if starting_task {
            store_state::set(conn, "open task", &preview(&owner.content))?;
        }
        self.dispatch_state.control.work_open = true;
        self.dispatch_state.control.question_outstanding = false;
        let result = step(
            self.state.clone(),
            StepInput::Owner {
                content: owner.content,
                tokens: owner.tokens,
            },
        );
        self.apply_step_result(conn, now, result, true)?;
        Ok(())
    }

    fn endpoint_turn(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        match endpoint_complete(
            &self.runtime.client,
            &self.state.context,
            self.endpoint_attempt,
        ) {
            Ok(completion) => {
                self.endpoint_attempt = 0;
                let result = step(
                    self.state.clone(),
                    StepInput::Completion {
                        content: completion.content,
                        tokens: completion.usage.completion_tokens as usize,
                    },
                );
                self.apply_step_result(conn, now, result, false)
            }
            Err(error) => {
                self.endpoint_attempt = self.endpoint_attempt.saturating_add(1);
                self.record_endpoint_error(conn, now, &error.to_string())?;
                Ok(DaemonTick::EndpointError)
            }
        }
    }
}

fn next_owner_tokens(conn: &Connection) -> RuntimeResult<usize> {
    let rows = lkjagent_store::queue::list(conn)?;
    let tokens = rows
        .iter()
        .find(|row| row.status == "pending")
        .map_or(0, |row| {
            token_estimate(&lkjagent_protocol::render_owner(&row.content))
        });
    Ok(tokens)
}

fn preview(content: &str) -> String {
    let first = content
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or("active");
    first.chars().take(80).collect()
}
