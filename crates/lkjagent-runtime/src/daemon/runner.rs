use std::path::PathBuf;

use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_graph::TaskFamily;
use lkjagent_llm::client::ClientConfig;
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{DispatchState, ToolRuntime};
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};
use crate::graph_state::open_owner_case_with_guard;
use crate::intake;
use crate::step::{step, StepInput};
use crate::task::{RuntimeState, TaskState, DEFAULT_TURN_BUDGET};

use super::persisted::{next_owner_tokens, owner_preview};

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
}

impl ResidentRuntime {
    pub fn new(holder: String, client: ClientConfig, workspace: PathBuf, now: &str) -> Self {
        Self {
            holder,
            client,
            tools: ToolRuntime::new(workspace, now),
            budget: ContextBudgetPolicy::default(),
            task_turn_budget: DEFAULT_TURN_BUDGET,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentDaemon {
    pub state: RuntimeState,
    pub runtime: ResidentRuntime,
    pub dispatch_state: DispatchState,
    pub endpoint_attempt: u32,
    pub endpoint_retry_at: Option<String>,
}

impl ResidentDaemon {
    pub fn new(state: RuntimeState, runtime: ResidentRuntime) -> Self {
        Self {
            state,
            runtime,
            dispatch_state: DispatchState::default(),
            endpoint_attempt: 0,
            endpoint_retry_at: None,
        }
    }

    pub fn poll_once(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        self.runtime.tools.now = now.to_string();
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
        let owner_content = owner.content.clone();
        self.endpoint_retry_at = None;
        self.dispatch_state.reset_repeat_tracking();
        let starting_task = !matches!(
            self.state.task,
            TaskState::Open { .. } | TaskState::Waiting { .. }
        );
        let visible_task = store_state::get(conn, "open task")?;
        let visible_maintenance = visible_task
            .as_deref()
            .is_some_and(|task| task.starts_with("maintenance:"));
        if starting_task || visible_maintenance {
            store_state::set(conn, "open task", &owner_preview(&owner.content))?;
        }
        let previous_guard = self.dispatch_state.control.guard;
        if starting_task {
            self.dispatch_state.control.start_task(&owner.content);
        } else {
            self.dispatch_state.control.resume_task_with(&owner.content);
        }
        if starting_task || previous_guard != self.dispatch_state.control.guard {
            let guard = self.dispatch_state.control.guard.as_state_value();
            store_state::set(conn, "completion guard", &guard)?;
        }
        let scaffold_docs = starting_task
            && self.dispatch_state.control.guard.is_recursive()
            && Self::recursive_docs_requested(&owner.content);
        let benchmark_target = self
            .dispatch_state
            .control
            .guard
            .markdown_target()
            .filter(|_| Self::benchmark_docs_requested(&owner.content));
        let graph = if starting_task || visible_maintenance {
            Some(open_owner_case_with_guard(
                conn,
                &owner.content,
                now,
                self.dispatch_state.control.guard,
            )?)
        } else {
            None
        };
        let counted_document_guard = graph.as_ref().and_then(|graph| {
            (starting_task
                && benchmark_target.is_none()
                && graph.family == TaskFamily::Documentation
                && !self.dispatch_state.control.guard.is_recursive())
            .then(|| self.dispatch_state.control.guard.count_guard())
            .flatten()
        });
        let scaffold_profile = self.scaffold_profile();
        let result = step(
            self.state.clone(),
            StepInput::Owner {
                content: owner.content,
                tokens: owner.tokens,
                graph,
                turn_budget: self.runtime.task_turn_budget,
            },
        );
        self.apply_step_result(conn, now, result, true)?;
        if starting_task && self.dispatch_state.control.guard.is_recursive() && scaffold_docs {
            self.auto_scaffold_recursive_docs(conn, now, scaffold_profile)?;
        }
        if let Some(target) = benchmark_target {
            self.auto_scaffold_markdown_corpus(conn, now, target)?;
        }
        if let Some(guard) = counted_document_guard {
            self.auto_scaffold_counted_documents(conn, now, guard, &owner_content)?;
        }
        Ok(())
    }
}
