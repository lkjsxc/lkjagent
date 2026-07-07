use serde::{Deserialize, Serialize};

use crate::runtime_admission::workspace_relative_path;
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
use crate::workspace_record::{record_path, WorkspaceRecord};

pub const WORKSPACE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub schema_version: u32,
    pub root_policy: WorkspaceRootPolicy,
    pub directories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRootPolicy {
    pub root: String,
    pub archive_root: String,
    pub system_root: String,
    pub allow_rebalance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebalanceMove {
    pub entity_id: String,
    pub entity_kind: String,
    pub old_path: String,
    pub new_path: String,
    pub decision_id: String,
    pub reason: String,
    pub validation: Vec<String>,
}

impl WorkspaceManifest {
    pub fn default_workspace() -> Self {
        Self {
            schema_version: WORKSPACE_SCHEMA_VERSION,
            root_policy: WorkspaceRootPolicy {
                root: "workspace".to_string(),
                archive_root: "archive".to_string(),
                system_root: "system".to_string(),
                allow_rebalance: true,
            },
            directories: [
                "records",
                "artifacts",
                "projects",
                "repos",
                "indexes",
                "archive",
                "system",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        }
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}

pub fn canonical_record_path(kind: &str, id: &str) -> String {
    record_path(kind, id).unwrap_or_else(|_| format!("records/knowledge/notes/{kind}/{id}.md"))
}

pub fn validate_workspace_path(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("workspace path is empty".to_string());
    }
    if !workspace_relative_path(path) {
        return Err("workspace path escapes root".to_string());
    }
    Ok(())
}

pub fn validate_rebalance_move(item: &RebalanceMove) -> Vec<String> {
    let mut errors = Vec::new();
    for (label, path) in [("old", &item.old_path), ("new", &item.new_path)] {
        if let Err(error) = validate_workspace_path(path) {
            errors.push(format!("{label}_path:{error}"));
        }
    }
    if item.entity_id.trim().is_empty() {
        errors.push("entity_id:missing".to_string());
    }
    if item.old_path == item.new_path {
        errors.push("path:unchanged".to_string());
    }
    errors
}

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
