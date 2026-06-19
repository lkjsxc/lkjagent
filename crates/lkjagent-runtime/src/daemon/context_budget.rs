use lkjagent_context::budget::{ContextPressure, LOG_OBSERVATION};
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::{queue, state as store_state};
use rusqlite::Connection;

use super::pressure::pressure_name;
use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;
use crate::task::TaskState;

impl ResidentDaemon {
    pub(super) fn compact_before_owner(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let owner = next_pending_owner_tokens(conn)?;
        if self.state.compaction.is_some() {
            return if owner == 0 {
                Ok(None)
            } else {
                self.finish_compaction(conn, now, "owner delivery")
                    .map(Some)
            };
        }
        if owner == 0 {
            return Ok(None);
        }
        self.guard_pressure(
            conn,
            now,
            owner.saturating_add(LOG_OBSERVATION),
            "owner delivery",
        )
    }

    pub(super) fn compact_before_endpoint(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        if self.state.compaction.is_some() {
            return self.continue_compaction_before_endpoint(conn, now);
        }
        self.guard_pressure(conn, now, LOG_OBSERVATION, "endpoint call")
    }

    pub(super) fn compact_after_observation(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        if self.state.compaction.is_some() {
            return Ok(None);
        }
        self.guard_pressure(conn, now, 0, "observation append")
    }

    pub(super) fn write_context_observable(&self, conn: &Connection) -> RuntimeResult<()> {
        let policy = self.runtime.budget;
        let used = self.state.context.used_tokens();
        store_state::set(conn, "context window", &policy.window.to_string())?;
        store_state::set(conn, "context reserve", &policy.reserve.to_string())?;
        store_state::set(conn, "context used tokens", &used.to_string())?;
        store_state::set(
            conn,
            "context soft trigger",
            &policy.soft_trigger.to_string(),
        )?;
        store_state::set(
            conn,
            "context hard trigger",
            &policy.hard_trigger.to_string(),
        )?;
        store_state::set(
            conn,
            "context post compaction target",
            &policy.post_compaction_target.to_string(),
        )?;
        store_state::set(
            conn,
            "context pressure",
            pressure_name(policy.pressure(used, 0)),
        )?;
        Ok(())
    }

    fn guard_pressure(
        &mut self,
        conn: &mut Connection,
        now: &str,
        predicted: usize,
        reason: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let policy = self.runtime.budget;
        let pressure = policy.pressure(self.state.context.used_tokens(), predicted);
        match pressure {
            ContextPressure::BlackInvalid => self.pause_context(conn, now, reason),
            ContextPressure::Red => self.finish_compaction(conn, now, reason).map(Some),
            ContextPressure::Orange => self.start_compaction_distillation(conn, now, reason),
            ContextPressure::Green | ContextPressure::Yellow => Ok(None),
        }
    }

    fn pause_context(
        &mut self,
        conn: &Connection,
        now: &str,
        reason: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let message = format!("context budget invalid before {reason}");
        self.state.task = TaskState::Paused {
            reason: message.clone(),
        };
        append_event(conn, self.event_turn(), EventKind::Error, &message, 16, now)?;
        self.write_observable(conn)?;
        Ok(Some(DaemonTick::Paused))
    }
}

fn next_pending_owner_tokens(conn: &Connection) -> RuntimeResult<usize> {
    let rows = queue::list(conn)?;
    Ok(rows
        .iter()
        .find(|row| row.status == "pending")
        .map_or(0, |row| {
            token_estimate(&lkjagent_protocol::render_owner(&row.content))
        }))
}
