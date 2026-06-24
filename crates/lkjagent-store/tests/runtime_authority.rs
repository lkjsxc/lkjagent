mod support;

use lkjagent_store::runtime_authority::{
    admission_for_decision_and_tool, latest_decision, latest_snapshot_for_case,
    latest_transition_for_case, record_decision, record_effect, record_event, record_snapshot,
    record_tool_admission, record_transition, AuthorityDecisionInput, AuthorityEventInput,
    AuthoritySnapshotInput, RuntimeEffectInput, RuntimeTransitionInput, ToolAdmissionInput,
};
use support::{memory_store, TestResult};

#[test]
fn runtime_authority_tables_persist_replayable_history() -> TestResult<()> {
    let conn = memory_store()?;
    let missing = vec!["artifact-readiness".to_string()];
    let snapshot_id = record_snapshot(
        &conn,
        &AuthoritySnapshotInput {
            case_scope: "case",
            case_id: Some(7),
            queue_head: Some(42),
            queue_pending_count: 1,
            owner_objective: "finish artifact",
            active_mode: "OwnerTask",
            active_node: "artifact-audit",
            missing_evidence: &missing,
            artifact_head: Some("artifact-7"),
            fault_head: None,
            compaction_head: None,
            maintenance_state: "inactive",
            prompt_frame_id: Some("prompt-1"),
            context_frame_id: Some("context-1"),
            staleness_fingerprint: "stale-1",
            created_at: "2026-01-01T00:00:00Z",
        },
    )?;
    let event_id = record_event(
        &conn,
        &AuthorityEventInput {
            snapshot_id: Some(snapshot_id),
            case_scope: "case",
            case_id: Some(7),
            event_kind: "completion_requested",
            event_payload: "{missing:[artifact-readiness]}",
            created_at: "2026-01-01T00:00:01Z",
        },
    )?;
    let admitted = vec!["artifact.next".to_string(), "fs.batch_write".to_string()];
    let blocked = vec!["agent.done".to_string()];
    let decision_id = record_decision(
        &conn,
        &AuthorityDecisionInput {
            snapshot_id: Some(snapshot_id),
            case_scope: "case",
            case_id: Some(7),
            event_id,
            mission: "owner_execution",
            active_mode: "OwnerTask",
            active_node: "artifact-audit",
            admitted_tools: &admitted,
            blocked_tools: &blocked,
            missing_evidence: &missing,
            forced_next_action: "run artifact.next",
            exact_valid_example: Some("<action>artifact.next</action>"),
            completion_allowed: false,
            completion_refusal: Some("completion missing required evidence"),
            recovery_route: None,
            compaction_required: false,
            maintenance_allowed: false,
            authority_fingerprint: "fp-1",
            staleness_fingerprint: "stale-1",
            created_at: "2026-01-01T00:00:02Z",
        },
    )?;
    let admission_id = record_tool_admission(
        &conn,
        &ToolAdmissionInput {
            decision_id,
            case_scope: "case",
            case_id: Some(7),
            requested_tool: "agent.done",
            admitted: false,
            refusal_reason: "completion missing required evidence",
            exact_valid_example: Some("<action>artifact.next</action>"),
            created_at: "2026-01-01T00:00:03Z",
        },
    )?;
    record_transition(
        &conn,
        &RuntimeTransitionInput {
            snapshot_id,
            event_id,
            decision_id,
            case_scope: "case",
            case_id: Some(7),
            from_node: "audit",
            to_node: "repair",
            transition_kind: "completion_blocked",
            created_at: "2026-01-01T00:00:04Z",
        },
    )?;
    record_effect(
        &conn,
        &RuntimeEffectInput {
            decision_id,
            admission_id: Some(admission_id),
            effect_kind: "refusal",
            effect_summary: "agent.done refused",
            observation_event_id: None,
            created_at: "2026-01-01T00:00:05Z",
        },
    )?;

    let snapshot = latest_snapshot_for_case(&conn, 7)?.ok_or("missing snapshot")?;
    assert_eq!(snapshot.id, snapshot_id);
    assert_eq!(snapshot.case_scope, "case");
    assert_eq!(snapshot.case_id, Some(7));
    assert_eq!(snapshot.staleness_fingerprint, "stale-1");
    let latest = latest_decision(&conn, 7)?.ok_or("missing authority decision")?;
    assert_eq!(latest.snapshot_id, Some(snapshot_id));
    assert_eq!(latest.event_id, event_id);
    assert_eq!(latest.mission, "owner_execution");
    assert_eq!(latest.admitted_tools, "artifact.next,fs.batch_write");
    assert_eq!(latest.blocked_tools, "agent.done");
    assert_eq!(latest.missing_evidence, "artifact-readiness");
    assert_eq!(latest.staleness_fingerprint, "stale-1");
    let admission = admission_for_decision_and_tool(&conn, decision_id, "agent.done")?
        .ok_or("missing admission")?;
    assert_eq!(admission.id, admission_id);
    assert!(!admission.admitted);
    let transition = latest_transition_for_case(&conn, 7)?.ok_or("missing transition")?;
    assert_eq!(transition.decision_id, decision_id);
    assert_eq!(transition.to_node, "repair");
    Ok(())
}
