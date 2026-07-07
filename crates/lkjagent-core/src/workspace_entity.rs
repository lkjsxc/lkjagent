use serde::{Deserialize, Serialize};

use crate::workspace_manifest::validate_workspace_path;
use crate::workspace_record::WorkspaceRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceEntityKind {
    Record,
    Artifact,
    Project,
    Repository,
    Index,
    System,
    ExternalReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceVisibility {
    Private,
    Project,
    Public,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceRetention {
    Active,
    Archive,
    Evidence,
    Ephemeral,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEntity {
    pub id: String,
    pub kind: WorkspaceEntityKind,
    pub path: String,
    pub title: String,
    pub visibility: WorkspaceVisibility,
    pub retention: WorkspaceRetention,
    pub tags: Vec<String>,
    pub ledger_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEntityIssue {
    pub code: String,
    pub message: String,
}

impl WorkspaceEntity {
    pub fn record(record: &WorkspaceRecord, path: impl Into<String>) -> Self {
        Self {
            id: record.id.clone(),
            kind: WorkspaceEntityKind::Record,
            path: path.into(),
            title: record.title.clone(),
            visibility: WorkspaceVisibility::Private,
            retention: WorkspaceRetention::Active,
            tags: record.tags.clone(),
            ledger_refs: record.state_keys.clone(),
        }
    }

    pub fn moved_to(&self, path: impl Into<String>) -> Self {
        let mut moved = self.clone();
        moved.path = path.into();
        moved
    }
}

pub fn validate_entity(entity: &WorkspaceEntity) -> Vec<WorkspaceEntityIssue> {
    let mut issues = Vec::new();
    if entity.id.trim().is_empty() {
        issues.push(issue("id-missing", "entity id is required"));
    }
    if let Err(error) = validate_workspace_path(&entity.path) {
        issues.push(issue("path-invalid", error));
    }
    if entity.title.trim().is_empty() {
        issues.push(issue("title-missing", "entity title is required"));
    }
    if entity.kind == WorkspaceEntityKind::Index
        && entity.retention != WorkspaceRetention::Ephemeral
    {
        issues.push(issue(
            "index-retention",
            "derived indexes must be ephemeral workspace entities",
        ));
    }
    if entity.kind == WorkspaceEntityKind::System
        && entity.visibility != WorkspaceVisibility::System
        && entity.visibility != WorkspaceVisibility::Private
    {
        issues.push(issue(
            "system-visibility",
            "system entities cannot be public or project visible",
        ));
    }
    issues
}

pub fn preserve_identity_after_move(before: &WorkspaceEntity, after: &WorkspaceEntity) -> bool {
    before.id == after.id && before.kind == after.kind && before.path != after.path
}

fn issue(code: impl Into<String>, message: impl Into<String>) -> WorkspaceEntityIssue {
    WorkspaceEntityIssue {
        code: code.into(),
        message: message.into(),
    }
}
