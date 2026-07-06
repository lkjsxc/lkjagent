use serde::{Deserialize, Serialize};

use crate::runtime_admission::workspace_relative_path;
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

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
    format!("records/{kind}/{id}.md")
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
