mod support;

use lkjagent_store::runtime_authority::{
    dense_packet_for_decision, record_decision, record_dense_runtime_row, record_event,
    record_snapshot, AuthorityDecisionInput, AuthorityEventInput, AuthoritySnapshotInput,
    DenseRuntimeRowInput,
};
use support::{memory_store, TestResult};

#[test]
fn dense_runtime_packet_reopens_for_decision() -> TestResult<()> {
    let conn = memory_store()?;
    let missing = vec!["artifact-readiness".to_string()];
    let snapshot_id = record_snapshot(&conn, &snapshot_input(&missing))?;
    let event_id = record_event(&conn, &event_input(snapshot_id))?;
    let decision_id = record_decision(&conn, &decision_input(event_id, &missing))?;

    for (kind, subject, predicate, object) in [
        ("fact", "root", "status", "stories/iwanna"),
        ("obligation", "artifact-readiness", "required_by", "audit"),
        (
            "resolver_plan",
            "selected",
            "label",
            "rule=audit-artifact-audit plan=audit:artifact.audit",
        ),
        ("resolver_rule", "selected", "id", "audit-artifact-audit"),
        ("progress", "selected", "key", "root=stories/iwanna"),
        (
            "completion_input",
            "completion",
            "input",
            "artifact_ready=false",
        ),
    ] {
        record_dense_runtime_row(
            &conn,
            &DenseRuntimeRowInput {
                decision_id,
                row_kind: kind,
                subject,
                predicate,
                object,
                created_at: "2026-01-01T00:00:03Z",
            },
        )?;
    }

    let packet = dense_packet_for_decision(&conn, decision_id)?;

    assert_eq!(packet.facts.len(), 1);
    assert_eq!(packet.obligations[0].subject, "artifact-readiness");
    assert_eq!(
        packet.resolver_plans[0].object,
        "rule=audit-artifact-audit plan=audit:artifact.audit"
    );
    assert_eq!(packet.resolver_rules[0].object, "audit-artifact-audit");
    assert_eq!(packet.progress[0].object, "root=stories/iwanna");
    assert_eq!(packet.completion_inputs[0].object, "artifact_ready=false");
    Ok(())
}

fn snapshot_input<'a>(missing: &'a [String]) -> AuthoritySnapshotInput<'a> {
    AuthoritySnapshotInput {
        case_scope: "case",
        case_id: Some(9),
        queue_head: None,
        queue_pending_count: 0,
        owner_objective: "finish artifact",
        active_mode: "OwnerTask",
        active_node: "artifact-audit",
        missing_evidence: missing,
        artifact_head: Some("stories/iwanna"),
        fault_head: None,
        compaction_head: None,
        maintenance_state: "inactive",
        prompt_frame_id: None,
        context_frame_id: None,
        staleness_fingerprint: "stale-9",
        created_at: "2026-01-01T00:00:00Z",
    }
}

fn event_input(snapshot_id: i64) -> AuthorityEventInput<'static> {
    AuthorityEventInput {
        snapshot_id: Some(snapshot_id),
        case_scope: "case",
        case_id: Some(9),
        event_kind: "observation_recorded",
        event_payload: "{}",
        created_at: "2026-01-01T00:00:01Z",
    }
}

fn decision_input<'a>(event_id: i64, missing: &'a [String]) -> AuthorityDecisionInput<'a> {
    AuthorityDecisionInput {
        snapshot_id: None,
        case_scope: "case",
        case_id: Some(9),
        event_id,
        mission: "owner_execution",
        active_mode: "OwnerTask",
        active_node: "artifact-audit",
        admitted_tools: &[],
        blocked_tools: &[],
        missing_evidence: missing,
        forced_next_action: "artifact.audit",
        exact_valid_example: None,
        completion_allowed: false,
        completion_refusal: Some("artifact-readiness"),
        recovery_route: None,
        compaction_required: false,
        maintenance_allowed: false,
        authority_fingerprint: "fp-9",
        staleness_fingerprint: "stale-9",
        created_at: "2026-01-01T00:00:02Z",
    }
}
