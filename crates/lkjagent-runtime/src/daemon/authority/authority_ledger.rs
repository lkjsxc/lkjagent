use lkjagent_store::runtime_authority::{
    record_decision, record_event, record_snapshot, record_transition, AuthorityDecisionInput,
    AuthorityEventInput, AuthoritySnapshotInput, RuntimeTransitionInput,
};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::authority_ledger_support::{
    authority_fingerprint, case_ref, compaction_head, completion_allowed, completion_refusal,
    event_kind, event_payload, maintenance_state, missing_evidence, optional, strings,
};
use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::mode::{ActiveMode, TurnAuthority};

pub(super) struct AuthorityGraphView<'a> {
    pub case_id: &'a str,
    pub node: &'a str,
    pub evidence_gaps: &'a str,
    pub recovery_route: &'a str,
}

pub(super) fn persist_authority_ledger(
    daemon: &ResidentDaemon,
    conn: &Connection,
    authority: &TurnAuthority,
    graph: AuthorityGraphView<'_>,
) -> RuntimeResult<()> {
    let case_ref = case_ref(graph.case_id);
    let created_at = daemon.runtime.tools.now.as_str();
    let admitted_tools = strings(&authority.effective_policy.allowed_tools);
    let blocked_tools = strings(&authority.effective_policy.blocked_tools);
    let missing_evidence = missing_evidence(graph.evidence_gaps);
    let active_mode = format!("{:?}", authority.mode);
    let fingerprint = authority_fingerprint(authority, &graph, &admitted_tools, &missing_evidence);
    let snapshot_id = record_snapshot(
        conn,
        &AuthoritySnapshotInput {
            case_scope: case_ref.scope,
            case_id: case_ref.id,
            queue_head: None,
            queue_pending_count: authority.input.pending_owner_rows as i64,
            owner_objective: graph.node,
            active_mode: &active_mode,
            active_node: graph.node,
            missing_evidence: &missing_evidence,
            artifact_head: authority.snapshot.active_artifact.as_deref(),
            fault_head: authority.snapshot.last_tool_attempt.as_deref(),
            compaction_head: compaction_head(authority),
            maintenance_state: maintenance_state(authority),
            prompt_frame_id: authority.input.prompt_frame_id.as_deref(),
            context_frame_id: None,
            staleness_fingerprint: &fingerprint,
            created_at,
        },
    )?;
    let event_payload = event_payload(authority, &graph);
    let event_id = record_event(
        conn,
        &AuthorityEventInput {
            snapshot_id: Some(snapshot_id),
            case_scope: case_ref.scope,
            case_id: case_ref.id,
            event_kind: event_kind(authority),
            event_payload: &event_payload,
            created_at,
        },
    )?;
    let completion_refusal = completion_refusal(authority, graph.evidence_gaps);
    let recovery_route = optional(graph.recovery_route);
    let decision_id = record_decision(
        conn,
        &AuthorityDecisionInput {
            snapshot_id: Some(snapshot_id),
            case_scope: case_ref.scope,
            case_id: case_ref.id,
            event_id,
            mission: authority.mission.as_str(),
            active_mode: &active_mode,
            active_node: graph.node,
            admitted_tools: &admitted_tools,
            blocked_tools: &blocked_tools,
            missing_evidence: &missing_evidence,
            forced_next_action: authority.effective_policy.preferred_next_action,
            exact_valid_example: Some(authority.valid_example.as_str()),
            completion_allowed: completion_allowed(authority, graph.evidence_gaps),
            completion_refusal,
            recovery_route,
            compaction_required: authority.mode == ActiveMode::Compaction,
            maintenance_allowed: authority.mode == ActiveMode::Maintenance,
            authority_fingerprint: &fingerprint,
            staleness_fingerprint: &fingerprint,
            created_at,
        },
    )?;
    record_transition(
        conn,
        &RuntimeTransitionInput {
            snapshot_id,
            event_id,
            decision_id,
            case_scope: case_ref.scope,
            case_id: case_ref.id,
            from_node: graph.node,
            to_node: graph.node,
            transition_kind: authority.mission.as_str(),
            created_at,
        },
    )?;
    store_state::set(conn, "authority snapshot id", &snapshot_id.to_string())?;
    store_state::set(conn, "authority decision id", &decision_id.to_string())?;
    store_state::set(conn, "authority fingerprint", &fingerprint)?;
    Ok(())
}
