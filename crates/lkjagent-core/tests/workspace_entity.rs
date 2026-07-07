use lkjagent_core::workspace_entity::{
    preserve_identity_after_move, validate_entity, WorkspaceEntity, WorkspaceEntityKind,
    WorkspaceRetention, WorkspaceVisibility,
};
use lkjagent_core::workspace_record::WorkspaceRecord;

#[test]
fn record_entity_preserves_stable_id_across_move() {
    let mut record = WorkspaceRecord::new("rec-1", "todo", "Pay bill", "now");
    record.tags = vec!["finance".into()];
    record.state_keys = vec!["todo:open/rec-1".into()];
    let entity = WorkspaceEntity::record(&record, "records/todo/rec-1.md");
    let mut moved = entity.clone();
    moved.path = "archive/todo/rec-1.md".into();

    assert!(validate_entity(&entity).is_empty());
    assert!(preserve_identity_after_move(&entity, &moved));
    assert_eq!(moved.id, "rec-1");
    assert_eq!(moved.ledger_refs, vec!["todo:open/rec-1"]);
}

#[test]
fn validation_rejects_missing_identity_and_path_escape() {
    let entity = WorkspaceEntity {
        id: "".into(),
        kind: WorkspaceEntityKind::Record,
        path: "../outside.md".into(),
        title: "".into(),
        visibility: WorkspaceVisibility::Private,
        retention: WorkspaceRetention::Active,
        tags: Vec::new(),
        ledger_refs: Vec::new(),
    };

    let codes = validate_entity(&entity)
        .into_iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert_eq!(codes, vec!["id-missing", "path-invalid", "title-missing"]);
}

#[test]
fn derived_indexes_are_not_source_truth_entities() {
    let entity = WorkspaceEntity {
        id: "index-records".into(),
        kind: WorkspaceEntityKind::Index,
        path: "indexes/records.json".into(),
        title: "Records Index".into(),
        visibility: WorkspaceVisibility::System,
        retention: WorkspaceRetention::Active,
        tags: Vec::new(),
        ledger_refs: Vec::new(),
    };

    let issues = validate_entity(&entity);

    assert_eq!(issues[0].code, "index-retention");
}

#[test]
fn system_entities_are_not_public() {
    let entity = WorkspaceEntity {
        id: "manifest".into(),
        kind: WorkspaceEntityKind::System,
        path: "system/workspace-manifest.json".into(),
        title: "Workspace Manifest".into(),
        visibility: WorkspaceVisibility::Public,
        retention: WorkspaceRetention::Evidence,
        tags: Vec::new(),
        ledger_refs: Vec::new(),
    };

    let issues = validate_entity(&entity);

    assert_eq!(issues[0].code, "system-visibility");
}
