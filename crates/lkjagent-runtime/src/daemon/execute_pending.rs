use lkjagent_tools::dispatch::dispatch_with_text;
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::mode::{policy_for_mode, select_active_mode, ActiveModeInput};
use crate::step::{step, StepInput};
use crate::task::TaskState;

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
        let mode = select_active_mode(self.mode_input_for_pending());
        let mode_policy = policy_for_mode(mode);
        let maintenance_ask = self.maintenance_ask_pending(conn, pending.action.tool.as_str())?;
        self.sync_effective_dispatch_policy(&mode_policy);
        let output = {
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

    fn mode_input_for_pending(&self) -> ActiveModeInput {
        let active_owner_case = matches!(
            self.state.task,
            TaskState::Open { .. } | TaskState::Waiting { .. } | TaskState::Paused { .. }
        ) && self.state.maintenance.is_none();
        ActiveModeInput {
            pending_owner_rows: 0,
            active_owner_case,
            recoverable_owner_case: active_owner_case
                && (self.state.parse_faults > 0
                    || self.state.repeat_faults > 0
                    || self.state.tool_faults > 0),
            compaction_required: self.state.compaction.is_some(),
            maintenance_due: false,
            maintenance_active: self.state.maintenance.is_some(),
        }
    }
}
