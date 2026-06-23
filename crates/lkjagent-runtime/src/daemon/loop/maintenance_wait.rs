use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::task::TaskState;

impl ResidentDaemon {
    pub(super) fn maintenance_ask_pending(
        &self,
        conn: &Connection,
        tool: &str,
    ) -> RuntimeResult<bool> {
        if tool != "agent.ask" {
            return Ok(false);
        }
        Ok(self.state.maintenance.is_some() || open_task_is_maintenance(conn)?)
    }

    pub(super) fn close_maintenance_ask(&mut self, conn: &Connection) -> RuntimeResult<()> {
        self.dispatch_state.control.question_outstanding = false;
        self.state.maintenance = None;
        self.state.task = TaskState::Idle;
        self.write_observable(conn)
    }
}

fn open_task_is_maintenance(conn: &Connection) -> RuntimeResult<bool> {
    Ok(store_state::get(conn, "open task")?
        .as_deref()
        .is_some_and(|task| task.starts_with("maintenance:")))
}
