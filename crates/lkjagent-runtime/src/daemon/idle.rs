use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::maintenance::{prepare_idle_cycle, BoundaryDecision};
use crate::step::{step, StepInput};

use super::runner::{DaemonTick, ResidentDaemon};

impl ResidentDaemon {
    pub(super) fn open_idle_maintenance(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        match prepare_idle_cycle(conn, &self.state, now)? {
            BoundaryDecision::StartCycle { directive, budget } => {
                self.dispatch_state.control.resume_task();
                let result = step(
                    self.state.clone(),
                    StepInput::StartMaintenance { directive, budget },
                );
                self.apply_step_result(conn, now, result, false).map(Some)
            }
            _ => Ok(None),
        }
    }
}
