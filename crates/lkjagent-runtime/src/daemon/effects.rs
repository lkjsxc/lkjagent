use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::dispatch_with_text;
use rusqlite::Connection;

use super::compaction_gate::blocked_compaction_output;
use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;
use crate::step::{step, Effect, StepInput, StepResult};
use crate::task::StopReason;

impl ResidentDaemon {
    pub(super) fn apply_step_result(
        &mut self,
        conn: &mut Connection,
        now: &str,
        result: StepResult,
        skip_owner_record: bool,
    ) -> RuntimeResult<DaemonTick> {
        let mut tick = tick_for_stop(result.stop_reason);
        self.state = result.state;
        for effect in result.effects {
            if let Some(next) = self.apply_effect(conn, now, effect, skip_owner_record)? {
                tick = next;
            }
        }
        self.write_observable(conn)?;
        Ok(tick)
    }

    pub(super) fn record_endpoint_error(
        &self,
        conn: &Connection,
        now: &str,
        message: &str,
    ) -> RuntimeResult<()> {
        append_event(
            conn,
            self.event_turn(),
            EventKind::Error,
            message,
            token_estimate(message) as i64,
            now,
        )?;
        store_state::set(conn, "daemon state", "error")?;
        store_state::set(conn, "daemon error", message)?;
        Ok(())
    }

    fn apply_effect(
        &mut self,
        conn: &mut Connection,
        now: &str,
        effect: Effect,
        skip_owner_record: bool,
    ) -> RuntimeResult<Option<DaemonTick>> {
        match effect {
            Effect::RecordEvent {
                kind: EventKind::Owner,
                ..
            } if skip_owner_record => Ok(None),
            Effect::RecordEvent {
                kind,
                content,
                tokens,
            } => {
                append_event(conn, self.event_turn(), kind, &content, tokens, now)?;
                Ok(None)
            }
            Effect::ExecuteTool { action_text } => self.execute_pending(conn, now, &action_text),
            Effect::DistillTask { summary, .. } => {
                self.save_task_summary(conn, now, &summary)?;
                Ok(None)
            }
            Effect::Pause { reason } => {
                store_state::set(conn, "daemon error", &reason)?;
                Ok(Some(DaemonTick::Paused))
            }
            Effect::CompactionRecorded {
                before_tokens,
                after_tokens,
                memory_ids,
                policy,
            } => self.record_compaction(conn, now, before_tokens, after_tokens, memory_ids, policy),
        }
    }
}

impl ResidentDaemon {
    fn execute_pending(
        &mut self,
        conn: &mut Connection,
        now: &str,
        action_text: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let Some(pending) = self.state.pending_action.clone() else {
            return Ok(None);
        };
        let maintenance_ask = self.maintenance_ask_pending(conn, pending.action.tool.as_str())?;
        let output = if self.state.compaction.is_some() && pending.action.tool != "memory.save" {
            blocked_compaction_output(&mut self.dispatch_state, action_text)
        } else {
            dispatch_with_text(
                &pending.action,
                action_text,
                &self.runtime.tools,
                conn,
                &mut self.dispatch_state,
            )
        };
        let compaction_output = output.clone();
        let result = step(self.state.clone(), StepInput::ToolOutput(output));
        let tick = self.apply_step_result(conn, now, result, false)?;
        if maintenance_ask {
            self.close_maintenance_ask(conn)?;
            return Ok(Some(DaemonTick::Done));
        }
        if let Some(next) =
            self.advance_compaction_after_output(conn, now, &pending.action, &compaction_output)?
        {
            return Ok(Some(next));
        }
        if let Some(next) = self.compact_after_observation(conn, now)? {
            return Ok(Some(next));
        }
        Ok(Some(tick))
    }

    fn save_task_summary(
        &mut self,
        conn: &mut Connection,
        now: &str,
        summary: &str,
    ) -> RuntimeResult<()> {
        let memory_id = lkjagent_store::memory::save(
            conn,
            lkjagent_store::memory::MemoryKind::TaskSummary,
            &summary_title(summary),
            "task",
            summary,
            token_estimate(summary) as i64,
            now,
        )?;
        store_state::set(conn, "last task summary id", &memory_id.to_string())?;
        store_state::set(conn, "open task", "none")?;
        store_state::delete(conn, "completion guard")?;
        let content = format!("task-summary memory_id={memory_id}\nsummary={summary}");
        append_event(
            conn,
            self.event_turn(),
            EventKind::Notice,
            &content,
            token_estimate(&content) as i64,
            now,
        )?;
        Ok(())
    }
}

fn tick_for_stop(stop: Option<StopReason>) -> DaemonTick {
    match stop {
        Some(StopReason::Done) => DaemonTick::Done,
        Some(StopReason::Ask) => DaemonTick::Waiting,
        Some(StopReason::EndpointError) => DaemonTick::EndpointError,
        Some(StopReason::Acted | StopReason::Compaction | StopReason::Maintenance) => {
            DaemonTick::Working
        }
        Some(StopReason::InvalidAction | StopReason::UnknownTool) => DaemonTick::Working,
        Some(StopReason::BadParams | StopReason::BudgetNotice) => DaemonTick::Working,
        Some(StopReason::ToolError | StopReason::RepeatAction) => DaemonTick::Working,
        None => DaemonTick::Working,
    }
}

fn summary_title(summary: &str) -> String {
    let first = summary
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or("closed task");
    first.chars().take(80).collect()
}
