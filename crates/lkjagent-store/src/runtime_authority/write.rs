use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::codec::encode_list;
use super::model::{
    AuthorityDecisionInput, AuthorityEventInput, AuthoritySnapshotInput, RuntimeEffectInput,
    RuntimeTransitionInput, ToolAdmissionInput,
};

pub fn record_snapshot(conn: &Connection, input: &AuthoritySnapshotInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_snapshots
         (case_scope, case_id, queue_head, queue_pending_count, owner_objective,
          active_mode, active_node, missing_evidence, artifact_head, fault_head,
          compaction_head, maintenance_state, prompt_frame_id, context_frame_id,
          staleness_fingerprint, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![
            input.case_scope,
            input.case_id,
            input.queue_head,
            input.queue_pending_count,
            input.owner_objective,
            input.active_mode,
            input.active_node,
            encode_list(input.missing_evidence),
            input.artifact_head,
            input.fault_head,
            input.compaction_head,
            input.maintenance_state,
            input.prompt_frame_id,
            input.context_frame_id,
            input.staleness_fingerprint,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_event(conn: &Connection, input: &AuthorityEventInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_authority_events
         (snapshot_id, case_scope, case_id, event_kind, event_payload, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            input.snapshot_id,
            input.case_scope,
            input.case_id,
            input.event_kind,
            input.event_payload,
            input.created_at
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_decision(conn: &Connection, input: &AuthorityDecisionInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_authority_decisions
         (snapshot_id, case_scope, case_id, event_id, mission, active_mode, active_node,
          admitted_tools, blocked_tools, missing_evidence, forced_next_action,
          exact_valid_example, completion_allowed, completion_refusal, recovery_route,
          compaction_required, maintenance_allowed, authority_fingerprint,
          staleness_fingerprint, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
          ?15, ?16, ?17, ?18, ?19, ?20)",
        params![
            input.snapshot_id,
            input.case_scope,
            input.case_id,
            input.event_id,
            input.mission,
            input.active_mode,
            input.active_node,
            encode_list(input.admitted_tools),
            encode_list(input.blocked_tools),
            encode_list(input.missing_evidence),
            input.forced_next_action,
            input.exact_valid_example,
            as_i64(input.completion_allowed),
            input.completion_refusal,
            input.recovery_route,
            as_i64(input.compaction_required),
            as_i64(input.maintenance_allowed),
            input.authority_fingerprint,
            input.staleness_fingerprint,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_tool_admission(
    conn: &Connection,
    input: &ToolAdmissionInput<'_>,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_tool_admissions
         (decision_id, case_scope, case_id, requested_tool, admitted, refusal_reason,
          exact_valid_example, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.decision_id,
            input.case_scope,
            input.case_id,
            input.requested_tool,
            as_i64(input.admitted),
            input.refusal_reason,
            input.exact_valid_example,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_transition(
    conn: &Connection,
    input: &RuntimeTransitionInput<'_>,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_transitions
         (snapshot_id, event_id, decision_id, case_scope, case_id, from_node,
          to_node, transition_kind, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            input.snapshot_id,
            input.event_id,
            input.decision_id,
            input.case_scope,
            input.case_id,
            input.from_node,
            input.to_node,
            input.transition_kind,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_effect(conn: &Connection, input: &RuntimeEffectInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_effects
         (decision_id, admission_id, effect_kind, effect_summary, observation_event_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            input.decision_id,
            input.admission_id,
            input.effect_kind,
            input.effect_summary,
            input.observation_event_id,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

fn as_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}
