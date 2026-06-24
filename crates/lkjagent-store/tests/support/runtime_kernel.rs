use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::runtime_authority::{
    record_decision, record_event, record_snapshot, AuthorityDecisionInput, AuthorityEventInput,
    AuthoritySnapshotInput,
};
use rusqlite::Connection;

use super::TestResult;

pub struct SeededDecision {
    pub snapshot_id: i64,
    pub event_id: i64,
    pub decision_id: i64,
}

pub fn seed_decision(conn: &Connection) -> TestResult<SeededDecision> {
    let missing = vec!["artifact-readiness".to_string()];
    let admitted = vec!["artifact.next".to_string()];
    let blocked = vec!["agent.done".to_string()];
    let snapshot_id = record_snapshot(conn, &snapshot_input(&missing))?;
    let event_id = record_event(conn, &event_input(snapshot_id))?;
    let decision_id = record_decision(
        conn,
        &AuthorityDecisionInput {
            snapshot_id: Some(snapshot_id),
            case_scope: "case",
            case_id: Some(17),
            event_id,
            mission: "owner_execution",
            active_mode: "OwnerTask",
            active_node: "artifact-next",
            admitted_tools: &admitted,
            blocked_tools: &blocked,
            missing_evidence: &missing,
            forced_next_action: "run artifact.next",
            exact_valid_example: Some("<act>artifact.next</act>"),
            completion_allowed: false,
            completion_refusal: Some("missing artifact readiness"),
            recovery_route: None,
            compaction_required: false,
            maintenance_allowed: false,
            authority_fingerprint: "authority-fp-1",
            staleness_fingerprint: "stale-fp-1",
            created_at: "2026-01-01T00:00:02Z",
        },
    )?;
    Ok(SeededDecision {
        snapshot_id,
        event_id,
        decision_id,
    })
}

pub fn temp_store_path() -> TestResult<PathBuf> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "lkjagent-store-test-{}-{nanos}.db",
        std::process::id()
    )))
}

pub fn remove_temp_store(path: PathBuf) -> TestResult<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(Box::new(error)),
    }
}

fn snapshot_input<'a>(missing: &'a [String]) -> AuthoritySnapshotInput<'a> {
    AuthoritySnapshotInput {
        case_scope: "case",
        case_id: Some(17),
        queue_head: Some(1),
        queue_pending_count: 1,
        owner_objective: "finish artifact",
        active_mode: "OwnerTask",
        active_node: "artifact-next",
        missing_evidence: missing,
        artifact_head: Some("artifact-17"),
        fault_head: None,
        compaction_head: None,
        maintenance_state: "inactive",
        prompt_frame_id: None,
        context_frame_id: None,
        staleness_fingerprint: "stale-fp-1",
        created_at: "2026-01-01T00:00:00Z",
    }
}

fn event_input(snapshot_id: i64) -> AuthorityEventInput<'static> {
    AuthorityEventInput {
        snapshot_id: Some(snapshot_id),
        case_scope: "case",
        case_id: Some(17),
        event_kind: "completion_requested",
        event_payload: "{missing:[artifact-readiness]}",
        created_at: "2026-01-01T00:00:01Z",
    }
}
