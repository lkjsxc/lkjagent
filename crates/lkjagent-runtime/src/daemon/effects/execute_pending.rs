#[path = "pending_observation.rs"]
mod pending_observation;

use lkjagent_tools::dispatch::{dispatch_with_text, DispatchOutput};
use rusqlite::Connection;

use super::authority_admission::{
    install_authority_view, record_authority_admission, record_authority_refusal,
};
use super::pending_staleness::{persisted_action_refusal, stale_action_refusal};
use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::{RuntimeError, RuntimeResult};
use crate::mode::EndpointDecision;
use crate::step::{step, StepInput};
use pending_observation::{notice_output, record_authority_observation};

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
        let cached = self.turn_authority.clone();
        let current = if cached
            .as_ref()
            .is_some_and(|authority| pending_matches_authority(&pending, authority))
        {
            cached
                .clone()
                .ok_or_else(|| RuntimeError::Store("cached authority disappeared".to_string()))?
        } else {
            self.decide_authority(conn, now, false)?
        };
        if let Some(message) = persisted_action_refusal(&pending, &current, &pending.action.tool)
            .or_else(|| stale_action_refusal(cached.as_ref(), &current, &pending.action.tool))
        {
            self.sync_effective_dispatch_policy(conn, &current.effective_policy)?;
            install_authority_view(conn, &mut self.dispatch_state, &current)?;
            if current.endpoint_decision == EndpointDecision::DeferMaintenance {
                self.state.maintenance = None;
            }
            let admission_id = record_authority_refusal(
                conn,
                now,
                &pending.action.tool,
                &message,
                &current.valid_example,
            )?;
            crate::model_log::record_provider_admission(
                conn,
                &pending.action.tool,
                false,
                &message,
                &current.valid_example,
            )?;
            let output = notice_output(&mut self.dispatch_state, action_text, message);
            return self.finish_pending_output(conn, now, output, false, admission_id);
        }
        let authority = cached.unwrap_or(current);
        let mode_policy = authority.effective_policy.clone();
        let maintenance_ask = self.maintenance_ask_pending(conn, pending.action.tool.as_str())?;
        self.sync_effective_dispatch_policy(conn, &mode_policy)?;
        install_authority_view(conn, &mut self.dispatch_state, &authority)?;
        let admission_id = record_authority_admission(
            conn,
            now,
            &self.dispatch_state,
            &pending.action.tool,
            &authority.valid_example,
        )?;
        crate::model_log::record_provider_admission(
            conn,
            &pending.action.tool,
            true,
            "admitted",
            &authority.valid_example,
        )?;
        let output = dispatch_with_text(
            &pending.action,
            action_text,
            &self.runtime.tools,
            conn,
            &mut self.dispatch_state,
        );
        self.finish_pending_output(conn, now, output, maintenance_ask, admission_id)
    }

    fn finish_pending_output(
        &mut self,
        conn: &mut Connection,
        now: &str,
        output: DispatchOutput,
        maintenance_ask: bool,
        admission_id: Option<i64>,
    ) -> RuntimeResult<Option<DaemonTick>> {
        crate::model_log::record_provider_observation(conn, &output.rendered)?;
        record_authority_observation(conn, now, admission_id, &output)?;
        let result = step(self.state.clone(), StepInput::ToolOutput(output));
        let tick = self.apply_step_result(conn, now, result, false)?;
        self.turn_authority = None;
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

fn pending_matches_authority(
    pending: &crate::task::PendingAction,
    authority: &crate::mode::TurnAuthority,
) -> bool {
    let persisted = pending.authority_decision_id.is_some()
        || pending.prompt_frame_id.is_some()
        || pending.staleness_fingerprint.is_some();
    persisted
        && pending.authority_decision_id == authority.input.latest_decision_id
        && optional_id_eq(&pending.prompt_frame_id, &authority.input.prompt_frame_id)
        && pending.staleness_fingerprint == authority.input.staleness_fingerprint
}

fn optional_id_eq(left: &Option<String>, right: &Option<String>) -> bool {
    normalize_optional_id(left) == normalize_optional_id(right)
}

fn normalize_optional_id(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|text| *text != "none")
}
