use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

pub const DEFAULT_UNIT_TARGET_TOKENS: u32 = 512;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactUnit {
    pub id: String,
    pub target_path: String,
    pub ordinal: u32,
    pub target_tokens: u32,
    pub target_words: Option<usize>,
    pub source_context_keys: Vec<String>,
    pub previous_tail_ref: Option<String>,
    pub content: String,
    pub check_passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssembledArtifact {
    pub path: String,
    pub content: String,
    pub fingerprint: String,
    pub word_count: usize,
    pub unit_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactError {
    pub message: String,
}

impl ArtifactUnit {
    pub fn new(id: impl Into<String>, path: impl Into<String>, ordinal: u32) -> Self {
        Self {
            id: id.into(),
            target_path: path.into(),
            ordinal,
            target_tokens: DEFAULT_UNIT_TARGET_TOKENS,
            target_words: None,
            source_context_keys: Vec::new(),
            previous_tail_ref: None,
            content: String::new(),
            check_passed: false,
        }
    }
}

pub fn assemble_checked_units(
    path: &str,
    units: &[ArtifactUnit],
) -> Result<AssembledArtifact, ArtifactError> {
    if units.is_empty() {
        return Err(error("artifact assembly requires at least one unit"));
    }
    let mut ordered = units.to_vec();
    ordered.sort_by_key(|unit| unit.ordinal);
    for unit in &ordered {
        if unit.target_path != path {
            return Err(error("artifact unit target path mismatch"));
        }
        if !unit.check_passed {
            return Err(error("artifact unit check is not passing"));
        }
    }
    let content = ordered
        .iter()
        .map(|unit| unit.content.trim())
        .collect::<Vec<_>>()
        .join("\n\n");
    let fingerprint = artifact_fingerprint(path, &content).map_err(fingerprint_error)?;
    Ok(AssembledArtifact {
        path: path.to_string(),
        word_count: count_words(&content),
        content,
        fingerprint,
        unit_ids: ordered.into_iter().map(|unit| unit.id).collect(),
    })
}

pub fn artifact_fingerprint(path: &str, content: &str) -> Result<String, FingerprintError> {
    stable_fingerprint(&serde_json::json!({
        "path": path,
        "bytes": content.len(),
        "content": content,
    }))
}

fn error(message: &str) -> ArtifactError {
    ArtifactError {
        message: message.to_string(),
    }
}

pub fn count_words(text: &str) -> usize {
    let latin = text
        .split_whitespace()
        .filter(|token| token.chars().any(|c| c.is_ascii_alphanumeric()))
        .count();
    let cjk = text
        .chars()
        .filter(|c| {
            matches!(
        *c as u32, 0x3400..=0x9fff | 0x3040..=0x30ff | 0xf900..=0xfaff)
        })
        .count();
    latin + cjk
}

fn fingerprint_error(error: FingerprintError) -> ArtifactError {
    ArtifactError {
        message: error.message,
    }
}
