mod support;

use lkjagent_store::runtime_authority::{
    latest_decision, record_decision, record_event, record_tool_admission, AuthorityDecisionInput,
    AuthorityEventInput, ToolAdmissionInput,
};
use support::{memory_store, TestResult};

#[test]
fn runtime_authority_tables_persist_decision_and_admission_history() -> TestResult<()> {
    let conn = memory_store()?;
    let event_id = record_event(
        &conn,
        &AuthorityEventInput {
            case_id: 7,
            event_kind: "completion_requested",
            event_payload: "{missing:[artifact-readiness]}",
            created_at: "2026-01-01T00:00:00Z",
        },
    )?;
    let admitted = vec!["artifact.next".to_string(), "fs.batch_write".to_string()];
    let blocked = vec!["agent.done".to_string()];
    let missing = vec!["artifact-readiness".to_string()];
    let decision_id = record_decision(
        &conn,
        &AuthorityDecisionInput {
            case_id: 7,
            event_id,
            mission: "owner_execution",
            active_mode: "OwnerTask",
            active_node: "artifact-audit",
            admitted_tools: &admitted,
            blocked_tools: &blocked,
            missing_evidence: &missing,
            forced_next_action: "run artifact.next",
            exact_valid_example: Some("<act>artifact.next</act>"),
            completion_allowed: false,
            completion_refusal: Some("completion missing required evidence"),
            recovery_route: None,
            compaction_required: false,
            maintenance_allowed: false,
            authority_fingerprint: "fp-1",
            created_at: "2026-01-01T00:00:01Z",
        },
    )?;
    record_tool_admission(
        &conn,
        &ToolAdmissionInput {
            decision_id,
            case_id: 7,
            requested_tool: "agent.done",
            admitted: false,
            refusal_reason: "completion missing required evidence",
            exact_valid_example: Some("<act>artifact.next</act>"),
            created_at: "2026-01-01T00:00:02Z",
        },
    )?;

    let latest = latest_decision(&conn, 7)?.ok_or("missing authority decision")?;
    assert_eq!(latest.id, decision_id);
    assert_eq!(latest.event_id, event_id);
    assert_eq!(latest.mission, "owner_execution");
    assert_eq!(latest.admitted_tools, "artifact.next,fs.batch_write");
    assert_eq!(latest.blocked_tools, "agent.done");
    assert_eq!(latest.missing_evidence, "artifact-readiness");
    assert!(!latest.completion_allowed);
    assert_eq!(latest.authority_fingerprint, "fp-1");
    let admission_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM runtime_tool_admissions WHERE decision_id = ?1",
        [decision_id],
        |row| row.get(0),
    )?;
    assert_eq!(admission_count, 1);
    Ok(())
}
