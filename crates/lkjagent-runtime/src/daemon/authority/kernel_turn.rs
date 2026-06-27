#[path = "kernel_turn_input.rs"]
mod input;
#[path = "kernel_turn_persist.rs"]
mod persist;
#[path = "kernel_turn_policy.rs"]
mod policy;

use rusqlite::Connection;

use super::authority::RuntimeAuthoritySnapshot;
use crate::error::{RuntimeError, RuntimeResult};
use crate::kernel::{
    render_prompt_frame, RuntimeDecision, RuntimeDecisionKind, RuntimeEffectCommand,
};
use crate::kernel_driver::{run_kernel_turn, KernelTurnInput, KernelTurnRecord};
use crate::mode::{
    completion_policy_for, render_mode_policy, EndpointDecision, RuntimeMission, TurnAuthority,
};
use input::{adapter_input, event_for, mode_snapshot};
use persist::persist_state;
use policy::policy_from_decision;

pub(super) fn decide_kernel_authority(
    conn: &Connection,
    snapshot: &RuntimeAuthoritySnapshot,
    now: &str,
    endpoint_retry_pending: bool,
) -> RuntimeResult<TurnAuthority> {
    let input = KernelTurnInput {
        snapshot: adapter_input(conn, snapshot)?,
        event: event_for(snapshot),
        case_scope: case_scope(snapshot).to_string(),
        created_at: now.to_string(),
    };
    let record = run_kernel_turn(conn, input).map_err(kernel_error)?;
    persist_state(conn, snapshot, &record, endpoint_retry_pending)?;
    Ok(turn_authority(snapshot, &record, endpoint_retry_pending))
}

fn turn_authority(
    snapshot: &RuntimeAuthoritySnapshot,
    record: &KernelTurnRecord,
    endpoint_retry_pending: bool,
) -> TurnAuthority {
    let decision = &record.decision;
    let mission = RuntimeMission::from_kernel(decision.mission);
    let mode = mission.active_mode();
    let policy = policy_from_decision(mode, decision);
    let endpoint_decision = endpoint_decision(snapshot, decision, endpoint_retry_pending);
    let prompt_card = prompt_card(decision);
    let dispatch_card = render_mode_policy(&policy);
    let input = authority_input(snapshot, record);
    TurnAuthority {
        mission,
        mode,
        input,
        snapshot: mode_snapshot(snapshot, mode),
        effective_policy: policy,
        completion_policy: completion_policy_for(mode),
        endpoint_decision,
        prompt_card,
        dispatch_card,
        valid_example: decision
            .admission_view
            .exact_next_action
            .clone()
            .unwrap_or_else(|| "none".to_string()),
    }
}

fn authority_input(
    snapshot: &RuntimeAuthoritySnapshot,
    record: &KernelTurnRecord,
) -> crate::mode::TurnAuthorityInput {
    let mut input: crate::mode::TurnAuthorityInput = snapshot.clone().into();
    input.latest_decision_id = Some(record.decision_id.to_string());
    input.prompt_frame_id = record.prompt_frame_id.map(|id| id.to_string());
    input.staleness_fingerprint = Some(record.decision.staleness_fingerprint.as_str().to_string());
    input
}

fn endpoint_decision(
    snapshot: &RuntimeAuthoritySnapshot,
    decision: &RuntimeDecision,
    endpoint_retry_pending: bool,
) -> EndpointDecision {
    if matches!(
        decision.runtime_effect,
        Some(RuntimeEffectCommand::CompactNow)
    ) {
        return EndpointDecision::RuntimeCompact;
    }
    if snapshot.maintenance_active
        && (snapshot.active_owner_case || snapshot.pending_owner_rows > 0)
    {
        return EndpointDecision::DeferMaintenance;
    }
    if snapshot.pending_owner_rows > 0 {
        return EndpointDecision::DeliverOwner;
    }
    if endpoint_retry_pending {
        return EndpointDecision::WaitForRetry;
    }
    if matches!(decision.kind, RuntimeDecisionKind::ClosedIdle)
        || matches!(
            decision.runtime_effect,
            Some(RuntimeEffectCommand::WaitClosedIdle)
        )
    {
        return EndpointDecision::ClosedIdle;
    }
    EndpointDecision::CallModel
}

fn prompt_card(decision: &RuntimeDecision) -> String {
    render_prompt_frame(decision)
        .unwrap_or_else(|_| format!("Runtime Authority\nmission={}", decision.mission.as_str()))
}

fn case_scope(snapshot: &RuntimeAuthoritySnapshot) -> &'static str {
    if snapshot.case_id.is_some() {
        "case"
    } else {
        "none"
    }
}

fn kernel_error(error: impl std::fmt::Debug) -> RuntimeError {
    RuntimeError::Store(format!("kernel turn failed: {error:?}"))
}
