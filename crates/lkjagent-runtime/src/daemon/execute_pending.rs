use lkjagent_tools::dispatch::dispatch_with_text;
use rusqlite::Connection;

use super::maintenance_gate::{blocked_maintenance_output, maintenance_allows};
use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::step::{step, StepInput};

impl ResidentDaemon {
    pub(super) fn execute_pending(
        &mut self,
        conn: &mut Connection,
        now: &str,
        action_text: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let Some(pending) = self.state.pending_action.clone() else {
            return Ok(None);
        };
        let maintenance_ask = self.maintenance_ask_pending(conn, pending.action.tool.as_str())?;
        let output =
            if self.state.maintenance.is_some() && !maintenance_allows(&pending.action.tool) {
                blocked_maintenance_output(&mut self.dispatch_state, action_text)
            } else {
                self.sync_graph_dispatch_state();
                dispatch_with_text(
                    &pending.action,
                    action_text,
                    &self.runtime.tools,
                    conn,
                    &mut self.dispatch_state,
                )
            };
        let result = step(self.state.clone(), StepInput::ToolOutput(output));
        let tick = self.apply_step_result(conn, now, result, false)?;
        if maintenance_ask {
            self.close_maintenance_ask(conn)?;
            return Ok(Some(DaemonTick::Done));
        }
        if let Some(next) = self.compact_after_observation(conn, now)? {
            return Ok(Some(next));
        }
        Ok(Some(tick))
    }
}
