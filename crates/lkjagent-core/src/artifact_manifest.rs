use serde::{Deserialize, Serialize};

use crate::runtime_artifact::DEFAULT_UNIT_TARGET_TOKENS;
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
use crate::workspace_manifest::validate_workspace_path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub root: String,
    pub schema_version: u32,
    pub audience: String,
    pub objectives: Vec<String>,
    pub units: Vec<ArtifactManifestUnit>,
    pub source_records: Vec<String>,
    pub checks: Vec<String>,
    pub status: String,
    pub layout_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifestUnit {
    pub id: String,
    pub parent_id: Option<String>,
    pub output_path: String,
    pub dependencies: Vec<String>,
    pub required_source_refs: Vec<String>,
    pub target_tokens: u32,
    pub target_words: Option<usize>,
    pub checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactIssue {
    pub unit_id: String,
    pub message: String,
}

impl ArtifactManifest {
    pub fn new(id: &str, kind: &str, title: &str, root: &str) -> Self {
        Self {
            id: id.to_string(),
            kind: kind.to_string(),
            title: title.to_string(),
            root: root.to_string(),
            schema_version: 1,
            audience: "owner".to_string(),
            objectives: Vec::new(),
            units: Vec::new(),
            source_records: Vec::new(),
            checks: vec!["unit-complete".to_string(), "no-placeholders".to_string()],
            status: "planned".to_string(),
            layout_rules: vec!["nested-paths-allowed".to_string()],
        }
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}

impl ArtifactManifestUnit {
    pub fn new(id: &str, output_path: &str) -> Self {
        Self {
            id: id.to_string(),
            parent_id: None,
            output_path: output_path.to_string(),
            dependencies: Vec::new(),
            required_source_refs: Vec::new(),
            target_tokens: DEFAULT_UNIT_TARGET_TOKENS,
            target_words: None,
            checks: vec!["complete".to_string(), "no-placeholders".to_string()],
        }
    }
}

pub fn validate_manifest(manifest: &ArtifactManifest) -> Vec<ArtifactIssue> {
    let mut issues = Vec::new();
    if manifest.units.is_empty() {
        issues.push(issue("manifest", "requires at least one unit"));
    }
    for unit in &manifest.units {
        if unit.id.trim().is_empty() {
            issues.push(issue("manifest", "unit id missing"));
        }
        if let Err(error) = validate_workspace_path(&unit.output_path) {
            issues.push(issue(&unit.id, &format!("output path {error}")));
        }
        if unit.required_source_refs.is_empty() && manifest.source_records.is_empty() {
            issues.push(issue(&unit.id, "source refs missing"));
        }
        if unit.checks.is_empty() {
            issues.push(issue(&unit.id, "checks missing"));
        }
        if placeholder(&unit.output_path) {
            issues.push(issue(&unit.id, "placeholder path"));
        }
    }
    issues
}

pub fn nested_unit_paths(manifest: &ArtifactManifest) -> Vec<String> {
    manifest
        .units
        .iter()
        .filter(|unit| unit.output_path.contains('/'))
        .map(|unit| unit.output_path.clone())
        .collect()
}

fn placeholder(value: &str) -> bool {
    let upper = value.to_ascii_uppercase();
    upper.contains("TODO") || upper.contains("FIELD_VALUE") || upper.contains("PLACEHOLDER")
}

fn issue(unit_id: &str, message: &str) -> ArtifactIssue {
    ArtifactIssue {
        unit_id: unit_id.to_string(),
        message: message.to_string(),
    }
}
