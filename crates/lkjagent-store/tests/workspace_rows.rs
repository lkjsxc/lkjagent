use lkjagent_core::workspace_manifest::{RebalanceMove, WorkspaceManifest};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::workspace_rows::{
    insert_alias, insert_rebalance_audit, operation_revisions, prepare_or_load_operation,
    resolve_alias, upsert_manifest, OperationDraft, OperationPreparation, OperationRevision,
    PathAliasRow,
};
use rusqlite::Connection;

#[test]
fn workspace_manifest_aliases_and_rebalance_audit_round_trip(
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    upsert_manifest(&conn, &WorkspaceManifest::default_workspace(), "now")?;
    let version: i64 = conn.query_row(
        "SELECT schema_version FROM workspace_manifest WHERE id = 'default'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(version, 1);

    insert_alias(
        &conn,
        &PathAliasRow {
            old_path: "records/note/old.md".to_string(),
            entity_id: "rec_1".to_string(),
            entity_kind: "record".to_string(),
            new_path: "records/note/rec_1.md".to_string(),
            decision_id: "decision-1".to_string(),
            created_at: "now".to_string(),
        },
    )?;
    assert_eq!(
        resolve_alias(&conn, "records/note/old.md")?.map(|row| row.entity_id),
        Some("rec_1".to_string())
    );

    insert_rebalance_audit(
        &conn,
        "rebalance-1",
        &RebalanceMove {
            entity_id: "rec_1".to_string(),
            entity_kind: "record".to_string(),
            old_path: "records/note/old.md".to_string(),
            new_path: "records/note/rec_1.md".to_string(),
            decision_id: "decision-1".to_string(),
            reason: "canonical".to_string(),
            validation: vec!["ok".to_string()],
        },
        "now",
    )?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_rebalance_audit",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 1);
    Ok(())
}

#[test]
fn operation_revisions_are_exact_and_idempotent() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let revisions = vec![
        OperationRevision {
            role: "prior".to_string(),
            path: "records/note/a.md".to_string(),
            bytes: vec![0, 255, 10],
            fingerprint: "prior-fingerprint".to_string(),
        },
        OperationRevision {
            role: "intended".to_string(),
            path: "archive/note/a.md".to_string(),
            bytes: b"intended".to_vec(),
            fingerprint: "intended-fingerprint".to_string(),
        },
    ];
    let first = prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: "operation-1",
            key: "key-1",
            kind: "archive",
            preimage: "{}",
            intended: "{}",
            revisions: &revisions,
            now: "now",
        },
    )?;
    let second = prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: "operation-2",
            key: "key-1",
            kind: "archive",
            preimage: "{}",
            intended: "{}",
            revisions: &revisions,
            now: "later",
        },
    )?;
    assert!(matches!(first, OperationPreparation::Prepared(_)));
    assert!(matches!(second, OperationPreparation::Existing(_)));
    assert_eq!(operation_revisions(&conn, "operation-1")?, revisions);
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM workspace_operations", [], |row| {
        row.get(0)
    })?;
    assert_eq!(count, 1);
    Ok(())
}
