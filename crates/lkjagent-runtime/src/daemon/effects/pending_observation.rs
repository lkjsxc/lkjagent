use lkjagent_store::runtime_authority::{
    record_effect, record_runtime_observation, RuntimeEffectInput, RuntimeObservationInput,
};
use lkjagent_store::state as store_state;
use lkjagent_tools::dispatch::{DispatchOutput, DispatchState};
use lkjagent_tools::observe::{self, OutputKind};
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};

pub(super) fn record_authority_observation(
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
    )
    .map_err(|error| RuntimeError::Store(format!("record runtime effect: {error}")))?;
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
    )
    .map_err(|error| RuntimeError::Store(format!("record runtime observation: {error}")))?;
    Ok(())
}

pub(super) fn notice_output(
    state: &mut DispatchState,
    action_text: &str,
    message: String,
) -> DispatchOutput {
    let frame = observe::notice("error", message);
    let frame_ref = state.next_frame_ref;
    state.next_frame_ref = state.next_frame_ref.saturating_add(1);
    state.last_action_text = Some(action_text.to_string());
    state.last_frame_ref = Some(frame_ref);
    state.last_output_kind = Some(frame.kind.clone());
    DispatchOutput {
        frame_ref,
        kind: frame.kind,
        content: frame.content,
        rendered: frame.rendered,
    }
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
