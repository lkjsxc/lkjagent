mod support;

use lkjagent_store::artifact_ledger::{
    latest_for_case, record_weak_path, upsert_artifact, weak_paths, ArtifactLedgerInput,
    WeakPathInput,
};
use support::{memory_store, TestResult};

#[test]
fn artifact_ledger_persists_identity_and_weak_paths() -> TestResult<()> {
    let conn = memory_store()?;
    let artifact_id = upsert_artifact(
        &conn,
        &ArtifactLedgerInput {
            case_id: 42,
            artifact_id: "42:cookbook:japanese-home:small",
            root: "cookbook/japanese-home",
            kind: "cookbook",
            normalized_topic: "japanese-home",
            requested_scale: "small",
            profile: "cookbook",
            lifecycle_state: "content-partial",
            topology_status: "passed",
            readiness_status: "failed",
            objective_match_status: "passed",
            latest_audit_id: Some("audit-1"),
            weak_path_count: 1,
        },
        "2026-01-01T00:00:00Z",
    )?;
    let missing = vec![
        "ingredients-with-quantities".to_string(),
        "method-steps".to_string(),
    ];
    let weak = vec!["scaffold-like".to_string()];
    record_weak_path(
        &conn,
        &WeakPathInput {
            artifact_ledger_id: artifact_id,
            path: "cookbook/japanese-home/miso-soup.md",
            role: "recipe",
            missing_requirements: &missing,
            weak_signals: &weak,
            semantic_mismatch: "none",
            retry_count: 0,
            updated_at: "2026-01-01T00:00:01Z",
        },
    )?;

    let latest = latest_for_case(&conn, 42)?.ok_or("missing artifact ledger")?;
    assert_eq!(latest.id, artifact_id);
    assert_eq!(latest.artifact_id, "42:cookbook:japanese-home:small");
    assert_eq!(latest.readiness_status, "failed");
    assert_eq!(latest.weak_path_count, 1);
    let weak_paths = weak_paths(&conn, artifact_id)?;
    assert_eq!(weak_paths.len(), 1);
    assert_eq!(weak_paths[0].role, "recipe");
    assert!(weak_paths[0]
        .missing_requirements
        .contains("ingredients-with-quantities"));
    Ok(())
}

#[test]
fn artifact_ledger_upsert_preserves_identity_and_updates_readiness() -> TestResult<()> {
    let conn = memory_store()?;
    let first = upsert_artifact(&conn, &artifact("failed", 2), "2026-01-01T00:00:00Z")?;
    let second = upsert_artifact(&conn, &artifact("passed", 0), "2026-01-01T00:00:02Z")?;

    assert_eq!(first, second);
    let latest = latest_for_case(&conn, 7)?.ok_or("missing artifact ledger")?;
    assert_eq!(latest.readiness_status, "passed");
    assert_eq!(latest.weak_path_count, 0);
    Ok(())
}

fn artifact(readiness_status: &str, weak_path_count: i64) -> ArtifactLedgerInput<'_> {
    ArtifactLedgerInput {
        case_id: 7,
        artifact_id: "7:dictionary:bread:large",
        root: "dictionary/bread",
        kind: "dictionary",
        normalized_topic: "bread",
        requested_scale: "large",
        profile: "dictionary",
        lifecycle_state: "audit-passed",
        topology_status: "passed",
        readiness_status,
        objective_match_status: "passed",
        latest_audit_id: Some("audit-2"),
        weak_path_count,
    }
}
