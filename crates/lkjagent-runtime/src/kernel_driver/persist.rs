use lkjagent_store::error::StoreResult;
use lkjagent_store::runtime_authority::{
    record_decision, record_decision_detail, record_effect, record_event, record_prompt_frame,
    record_snapshot, record_snapshot_detail, AuthorityDecisionInput, AuthorityEventInput,
    AuthoritySnapshotInput, PromptFrameInput, RuntimeEffectInput,
};
use rusqlite::Connection;

use crate::kernel::{RuntimeDecision, RuntimeSnapshot};
use crate::kernel_driver::input::KernelTurnInput;
use crate::kernel_driver::persist_map::{
    decision_detail, maintenance_state, next_action_class, queue_head_i64, snapshot_detail,
    tool_names,
};

pub fn persist_snapshot(
    conn: &Connection,
    input: &KernelTurnInput,
    snapshot: &RuntimeSnapshot,
) -> StoreResult<i64> {
    let fault = snapshot.latest_fault.map(|fault| format!("{fault:?}"));
    let snapshot_id = record_snapshot(
        conn,
        &AuthoritySnapshotInput {
            case_scope: &input.case_scope,
            case_id: input.case_id_i64(),
            queue_head: queue_head_i64(snapshot),
            queue_pending_count: snapshot.queue.pending_owner_count as i64,
            owner_objective: snapshot.case.owner_objective.as_deref().unwrap_or(""),
            active_mode: snapshot.active_mode.as_str(),
            active_node: snapshot.graph.node.as_deref().unwrap_or("none"),
            missing_evidence: &snapshot.evidence.missing,
            artifact_head: snapshot.artifact.root.as_deref(),
            fault_head: fault.as_deref(),
            compaction_head: snapshot.context.compaction_head.as_deref(),
            maintenance_state: maintenance_state(snapshot),
            prompt_frame_id: snapshot.prompt_frame_fingerprint.as_deref(),
            context_frame_id: None,
            staleness_fingerprint: snapshot.staleness_fingerprint.as_str(),
            created_at: &input.created_at,
        },
    )?;
    record_snapshot_detail(conn, &snapshot_detail(snapshot_id, snapshot))?;
    Ok(snapshot_id)
}

pub fn persist_event(
    conn: &Connection,
    input: &KernelTurnInput,
    snapshot_id: i64,
) -> StoreResult<i64> {
    record_event(
        conn,
        &AuthorityEventInput {
            snapshot_id: Some(snapshot_id),
            case_scope: &input.case_scope,
            case_id: input.case_id_i64(),
            event_kind: input.event.kind().as_str(),
            event_payload: &format!("{:?}", input.event),
            created_at: &input.created_at,
        },
    )
}

pub fn persist_decision(
    conn: &Connection,
    input: &KernelTurnInput,
    snapshot_id: i64,
    event_id: i64,
    snapshot: &RuntimeSnapshot,
    decision: &RuntimeDecision,
) -> StoreResult<i64> {
    let admitted = tool_names(&decision.admission_view.admitted_tools);
    let blocked = tool_names(&decision.admission_view.blocked_tools);
    let decision_id = record_decision(
        conn,
        &AuthorityDecisionInput {
            snapshot_id: Some(snapshot_id),
            case_scope: &input.case_scope,
            case_id: input.case_id_i64(),
            event_id,
            mission: decision.mission.as_str(),
            active_mode: decision.active_mode.as_str(),
            active_node: decision.graph_node.as_deref().unwrap_or("none"),
            admitted_tools: &admitted,
            blocked_tools: &blocked,
            missing_evidence: &decision.missing_evidence,
            forced_next_action: next_action_class(decision),
            exact_valid_example: decision.admission_view.exact_next_action.as_deref(),
            completion_allowed: decision.completion_allowed,
            completion_refusal: decision.completion_refusal.as_deref(),
            recovery_route: decision.recovery_plan.as_deref(),
            compaction_required: decision.compaction_plan.is_some(),
            maintenance_allowed: decision.maintenance_plan.is_some(),
            authority_fingerprint: decision.authority_fingerprint.as_str(),
            staleness_fingerprint: decision.staleness_fingerprint.as_str(),
            created_at: &input.created_at,
        },
    )?;
    record_decision_detail(conn, &decision_detail(decision_id, snapshot, decision))?;
    Ok(decision_id)
}

pub fn persist_prompt_frame(
    conn: &Connection,
    input: &KernelTurnInput,
    decision_id: i64,
    rendered: &str,
) -> StoreResult<i64> {
    record_prompt_frame(
        conn,
        &PromptFrameInput {
            decision_id,
            case_scope: &input.case_scope,
            case_id: input.case_id_i64(),
            frame_kind: "model-call",
            prompt_fingerprint: "kernel-driver-prompt",
            context_package_ids: &[],
            rendered_summary: rendered,
            created_at: &input.created_at,
        },
    )
}

pub fn persist_runtime_effect(
    conn: &Connection,
    decision_id: i64,
    summary: &str,
    created_at: &str,
) -> StoreResult<i64> {
    record_effect(
        conn,
        &RuntimeEffectInput {
            decision_id,
            admission_id: None,
            effect_kind: summary,
            effect_summary: summary,
            observation_event_id: None,
            created_at,
        },
    )
}
