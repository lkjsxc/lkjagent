use lkjagent_store::runtime_authority::{
    record_effect, record_runtime_observation, RuntimeEffectInput, RuntimeObservationInput,
};
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{dispatch_with_text, DispatchOutput};
use lkjagent_tools::observe::{self, OutputKind};
use rusqlite::Connection;

use super::authority_admission::{
    install_authority_view, record_authority_admission, record_authority_refusal,
};
use super::pending_staleness::{persisted_action_refusal, stale_action_refusal};
use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::mode::EndpointDecision;
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
        let cached = self.turn_authority.clone();
        let current = self.decide_authority(conn, now, false)?;
        if let Some(message) = persisted_action_refusal(&pending, &current, &pending.action.tool)
            .or_else(|| stale_action_refusal(cached.as_ref(), &current, &pending.action.tool))
        {
            self.sync_effective_dispatch_policy(conn, &current.effective_policy);
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
        self.sync_effective_dispatch_policy(conn, &mode_policy);
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

fn record_authority_observation(
    conn: &Connection,
    now: &str,
    admission_id: Option<i64>,
    output: &DispatchOutput,
) -> RuntimeResult<()> {
    let Some(decision_id) = numeric_state(conn, "authority decision id")? else {
        return Ok(());
    };
    let summary = one_line(&output.rendered);
    let effect_id = record_effect(
        conn,
        &RuntimeEffectInput {
            decision_id,
            admission_id,
            effect_kind: effect_kind(&output.kind),
            effect_summary: &summary,
            observation_event_id: None,
            created_at: now,
        },
    )?;
    let (observation_kind, status) = observation_shape(&output.kind);
    record_runtime_observation(
        conn,
        &RuntimeObservationInput {
            decision_id,
            admission_id,
            effect_id: Some(effect_id),
            observation_event_id: None,
            observation_kind,
            status,
            summary: &summary,
            created_at: now,
        },
    )?;
    Ok(())
}

fn effect_kind(kind: &OutputKind) -> &'static str {
    match kind {
        OutputKind::Observation { .. } => "tool.dispatch",
        OutputKind::Notice { .. } => "tool.refusal",
    }
}

fn observation_shape(kind: &OutputKind) -> (&'static str, &str) {
    match kind {
        OutputKind::Observation { status } => ("observation", status.as_str()),
        OutputKind::Notice { kind } => ("notice", kind.as_str()),
    }
}

fn numeric_state(conn: &Connection, key: &str) -> RuntimeResult<Option<i64>> {
    let Some(value) = store_state::get(conn, key)? else {
        return Ok(None);
    };
    Ok(value.parse::<i64>().ok())
}

fn one_line(value: &str) -> String {
    value
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.chars().take(160).collect())
        .unwrap_or_else(|| "none".to_string())
}

fn notice_output(
    state: &mut lkjagent_tools::dispatch::DispatchState,
    action_text: &str,
    message: String,
) -> DispatchOutput {
    let frame = observe::notice("error", message);
    let frame_ref = state.next_frame_ref;
    state.next_frame_ref = state.next_frame_ref.saturating_add(1);
    state.last_action_text = Some(action_text.to_string());
    state.last_frame_ref = Some(frame_ref);
    DispatchOutput {
        frame_ref,
        kind: frame.kind,
        content: frame.content,
        rendered: frame.rendered,
    }
}
