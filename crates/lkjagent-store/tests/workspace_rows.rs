use lkjagent_core::workspace_manifest::{RebalanceMove, WorkspaceManifest};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::workspace_rows::{
    insert_alias, insert_rebalance_audit, resolve_alias, upsert_manifest, PathAliasRow,
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
