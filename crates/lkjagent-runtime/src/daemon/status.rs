use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::task::TaskState;

impl ResidentDaemon {
    pub(super) fn write_observable(&self, conn: &Connection) -> RuntimeResult<()> {
        store_state::set(conn, "turn", &self.state.turn.to_string())?;
        self.write_context_observable(conn)?;
        match &self.state.task {
            TaskState::Open { .. } => self.write_working(conn),
            TaskState::Waiting { question } => {
                store_state::set(conn, "daemon state", "waiting")?;
                store_state::set(conn, "daemon question", question)?;
                store_state::delete(conn, "daemon error")?;
                Ok(())
            }
            TaskState::Paused { reason } => {
                store_state::set(conn, "daemon state", "error")?;
                store_state::set(conn, "daemon error", reason)?;
                Ok(())
            }
            TaskState::Idle | TaskState::Closed { .. } => {
                if let Some(cycle) = &self.state.maintenance {
                    store_state::set(conn, "daemon state", "working")?;
                    store_state::set(
                        conn,
                        "open task",
                        &format!("maintenance: {}", cycle.directive.as_str()),
                    )?;
                    store_state::delete(conn, "daemon question")?;
                    store_state::delete(conn, "daemon error")?;
                    return Ok(());
                }
                store_state::set(conn, "daemon state", "idle")?;
                store_state::set(conn, "open task", "none")?;
                store_state::delete(conn, "daemon question")?;
                store_state::delete(conn, "daemon error")?;
                Ok(())
            }
        }
    }

    pub(super) fn event_turn(&self) -> Option<i64> {
        if self.state.turn > 0 {
            Some(self.state.turn)
        } else {
            None
        }
    }

    fn write_working(&self, conn: &Connection) -> RuntimeResult<()> {
        store_state::set(conn, "daemon state", "working")?;
        if store_state::get(conn, "open task")?.as_deref() == Some("none") {
            store_state::set(conn, "open task", "active")?;
        }
        store_state::delete(conn, "daemon question")?;
        store_state::delete(conn, "daemon error")?;
        Ok(())
    }
}
