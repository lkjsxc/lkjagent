#[path = "graph_snapshot.rs"]
mod graph_snapshot;
use lkjagent_context::budget::{ContextPressure, LOG_OBSERVATION};
use lkjagent_context::model::{Frame, FrameKind};
use rusqlite::Connection;

use super::kernel_turn::decide_kernel_authority;
use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::mode::{render_turn_authority, TurnAuthority, TurnAuthorityInput};
use crate::prompt::token_estimate;
use crate::task::TaskState;
use graph_snapshot::graph_snapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAuthoritySnapshot {
    pub pending_owner_rows: usize,
    pub active_owner_case: bool,
    pub recoverable_owner_case: bool,
    pub compaction_required: bool,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
    pub endpoint_retry_pending: bool,
    pub case_id: Option<i64>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub artifact_root: Option<String>,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub latest_decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
}

impl ResidentDaemon {
    pub(super) fn decide_authority(
        &self,
        conn: &Connection,
        now: &str,
        endpoint_retry_pending: bool,
    ) -> RuntimeResult<TurnAuthority> {
        let snapshot = self.authority_snapshot(conn, now, endpoint_retry_pending)?;
        decide_kernel_authority(conn, &snapshot, now, endpoint_retry_pending)
    }

    pub(super) fn refresh_authority_card(
        &mut self,
        conn: &Connection,
        authority: &TurnAuthority,
    ) -> RuntimeResult<()> {
        self.state.context.log.retain(|frame| {
            !frame.content.starts_with("Active Mode:\n")
                && !frame.content.starts_with("Runtime Authority\n")
        });
        let rendered = persisted_authority_card(conn, authority)?;
        let prompt_frame_id = lkjagent_store::state::get(conn, "authority prompt frame id")?;
        let staleness = lkjagent_store::state::get(conn, "kernel staleness fingerprint")?;
        if let Some(cached) = self.turn_authority.as_mut() {
            cached.input.prompt_frame_id = prompt_frame_id;
            cached.input.staleness_fingerprint = staleness;
        }
        self.state.context.log.push(Frame::new(
            FrameKind::GraphNotice,
            rendered.clone(),
            token_estimate(&rendered),
        ));
        Ok(())
    }

    fn authority_snapshot(
        &self,
        conn: &Connection,
        now: &str,
        endpoint_retry_pending: bool,
    ) -> RuntimeResult<RuntimeAuthoritySnapshot> {
        let active_owner_case = self.active_owner_case();
        let graph = graph_snapshot(conn)?;
        Ok(RuntimeAuthoritySnapshot {
            pending_owner_rows: lkjagent_store::queue::pending_count(conn)?,
            active_owner_case,
            recoverable_owner_case: active_owner_case && self.recoverable_fault_active(),
            compaction_required: self.compaction_required(),
            maintenance_due: crate::maintenance::maintenance_due(conn, &self.state, now)?,
            maintenance_active: self.state.maintenance.is_some(),
            endpoint_retry_pending,
            case_id: graph.case_id,
            graph_node: graph.node,
            graph_phase: graph.phase,
            artifact_root: graph.artifact_root,
            required_evidence: graph.required_evidence,
            missing_evidence: graph.missing_evidence,
            latest_decision_id: lkjagent_store::state::get(conn, "authority decision id")?,
            prompt_frame_id: lkjagent_store::state::get(conn, "authority prompt frame id")?,
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

fn persisted_authority_card(conn: &Connection, authority: &TurnAuthority) -> RuntimeResult<String> {
    let mut rendered = render_turn_authority(authority);
    if let Some(decision_id) = lkjagent_store::state::get(conn, "authority decision id")? {
        rendered.push_str(&format!("\nauthority_decision_id={decision_id}"));
    }
    if let Some(fingerprint) = lkjagent_store::state::get(conn, "authority fingerprint")? {
        rendered.push_str(&format!("\nauthority_fingerprint={fingerprint}"));
    }
    Ok(rendered)
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
            case_id: snapshot.case_id,
            graph_node: snapshot.graph_node,
            graph_phase: snapshot.graph_phase,
            artifact_root: snapshot.artifact_root,
            required_evidence: snapshot.required_evidence,
            missing_evidence: snapshot.missing_evidence,
            latest_decision_id: snapshot.latest_decision_id,
            prompt_frame_id: snapshot.prompt_frame_id,
            staleness_fingerprint: None,
        }
    }
}
