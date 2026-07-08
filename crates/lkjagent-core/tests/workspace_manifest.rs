use lkjagent_core::workspace_manifest::{
    canonical_record_path, validate_rebalance_move, RebalanceMove, WorkspaceManifest,
};

#[test]
fn manifest_has_versioned_roots_and_path_validation() -> Result<(), String> {
    let manifest = WorkspaceManifest::default_workspace();
    assert_eq!(manifest.schema_version, 1);
    assert!(manifest.directories.contains(&"system".to_string()));
    assert!(manifest
        .fingerprint()
        .map_err(|error| error.message)?
        .starts_with("fnv1a64:"));
    assert_eq!(
        canonical_record_path("todo", "rec_1"),
        "records/life/todo/open/rec_1.md"
    );

    let ok = RebalanceMove {
        entity_id: "rec_1".to_string(),
        entity_kind: "record".to_string(),
        old_path: "records/knowledge/notes/old.md".to_string(),
        new_path: "records/life/todo/open/rec_1.md".to_string(),
        decision_id: "decision-1".to_string(),
        reason: "canonical record path".to_string(),
        validation: Vec::new(),
    };
    assert!(validate_rebalance_move(&ok).is_empty());

    let mut bad = ok;
    bad.new_path = "../secret".to_string();
    assert_eq!(
        validate_rebalance_move(&bad),
        vec!["new_path:workspace path escapes root"]
    );
    Ok(())
}
