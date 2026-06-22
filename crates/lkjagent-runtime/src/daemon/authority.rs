use lkjagent_context::budget::{ContextPressure, LOG_OBSERVATION};
use lkjagent_context::model::{Frame, FrameKind};
use rusqlite::Connection;

use super::authority_store::persist_authority_snapshot;
use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::mode::{
    decide_turn_authority, render_turn_authority, TurnAuthority, TurnAuthorityInput,
};
use crate::prompt::token_estimate;
use crate::task::TaskState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeAuthoritySnapshot {
    pub pending_owner_rows: usize,
    pub active_owner_case: bool,
    pub recoverable_owner_case: bool,
    pub compaction_required: bool,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
    pub endpoint_retry_pending: bool,
}

impl ResidentDaemon {
    pub(super) fn decide_authority(
        &self,
        conn: &Connection,
        now: &str,
        endpoint_retry_pending: bool,
    ) -> RuntimeResult<TurnAuthority> {
        let authority = decide_turn_authority(
            self.authority_snapshot(conn, now, endpoint_retry_pending)?
                .into(),
        );
        persist_authority_snapshot(self, conn, &authority)?;
        Ok(authority)
    }

    pub(super) fn refresh_authority_card(&mut self, authority: &TurnAuthority) {
        self.state
            .context
            .log
            .retain(|frame| !frame.content.starts_with("Active Mode:\n"));
        let rendered = render_turn_authority(authority);
        self.state.context.log.push(Frame::new(
            FrameKind::GraphNotice,
            rendered.clone(),
            token_estimate(&rendered),
        ));
    }

    fn authority_snapshot(
        &self,
        conn: &Connection,
        now: &str,
        endpoint_retry_pending: bool,
    ) -> RuntimeResult<RuntimeAuthoritySnapshot> {
        let active_owner_case = self.active_owner_case();
        Ok(RuntimeAuthoritySnapshot {
            pending_owner_rows: lkjagent_store::queue::pending_count(conn)?,
            active_owner_case,
            recoverable_owner_case: active_owner_case && self.recoverable_fault_active(),
            compaction_required: self.compaction_required(),
            maintenance_due: crate::maintenance::maintenance_due(conn, &self.state, now)?,
            maintenance_active: self.state.maintenance.is_some(),
            endpoint_retry_pending,
        })
    }

    fn active_owner_case(&self) -> bool {
        matches!(
            self.state.task,
            TaskState::Open { .. } | TaskState::Waiting { .. } | TaskState::Paused { .. }
        ) && self.state.maintenance.is_none()
    }

    fn recoverable_fault_active(&self) -> bool {
        self.state.parse_faults > 0 || self.state.repeat_faults > 0 || self.state.tool_faults > 0
    }

    fn compaction_required(&self) -> bool {
        if self.state.compaction.is_some() {
            return true;
        }
        matches!(
            self.runtime
                .budget
                .pressure(self.state.context.used_tokens(), LOG_OBSERVATION),
            ContextPressure::Orange | ContextPressure::Red | ContextPressure::BlackInvalid
        )
    }
}

impl From<RuntimeAuthoritySnapshot> for TurnAuthorityInput {
    fn from(snapshot: RuntimeAuthoritySnapshot) -> Self {
        Self {
            pending_owner_rows: snapshot.pending_owner_rows,
            active_owner_case: snapshot.active_owner_case,
            recoverable_owner_case: snapshot.recoverable_owner_case,
            compaction_required: snapshot.compaction_required,
            maintenance_due: snapshot.maintenance_due,
            maintenance_active: snapshot.maintenance_active,
            endpoint_retry_pending: snapshot.endpoint_retry_pending,
        }
    }
}
