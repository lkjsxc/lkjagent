use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
pub use crate::runtime_context_plan::{
    default_context_pipeline, select_context_plan, ContextFramePlan, ContextLanePlan,
    ContextPipelineStage, ContextPlanEntry,
};

#[rustfmt::skip]
pub const CONTEXT_REGIONS: &[&str] = &["identity-honesty", "phase-fault", "workspace-operation", "tools-grammar-example", "evidence", "owner-message"];
#[rustfmt::skip]
pub const CONTEXT_LANES: &[&str] = &["objective-constraints", "file-evidence", "memory-history", "recovery-diagnosis", "output-reserve"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustClass {
    Owner,
    Measured,
    Memory,
    Model,
    External,
    Recovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StalenessClass {
    Current,
    Stale,
    Superseded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContaminationClass {
    Clean,
    Stale,
    Superseded,
    UnverifiedModelClaim,
    FailedModelOutput,
    RefusedAction,
    RawToolLog,
    ExternalRaw,
    RecoveryOnly,
    SensitiveOwnerData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextItem {
    pub id: String,
    pub semantic_key: String,
    pub body: String,
    pub source_type: String,
    pub source_id: String,
    pub source_fingerprint: String,
    pub trust: TrustClass,
    pub staleness: StalenessClass,
    pub contamination: ContaminationClass,
    pub artifact_refs: Vec<String>,
    pub decision_id: Option<String>,
    pub created_at: String,
}

impl ContextItem {
    pub fn clean_fact(
        id: impl Into<String>,
        semantic_key: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            semantic_key: semantic_key.into(),
            body: body.into(),
            source_type: "test".to_string(),
            source_id: String::new(),
            source_fingerprint: String::new(),
            trust: TrustClass::Measured,
            staleness: StalenessClass::Current,
            contamination: ContaminationClass::Clean,
            artifact_refs: Vec::new(),
            decision_id: None,
            created_at: String::new(),
        }
    }

    pub fn is_normal_prompt_candidate(&self) -> bool {
        self.staleness == StalenessClass::Current && self.contamination == ContaminationClass::Clean
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextConflict {
    pub semantic_key: String,
    pub item_ids: Vec<String>,
}

pub fn select_normal_context(items: &[ContextItem]) -> Vec<ContextItem> {
    let plan = select_context_plan(items, &[]);
    let included = plan
        .included
        .iter()
        .map(|entry| entry.item_id.as_str())
        .collect::<BTreeSet<_>>();
    items
        .iter()
        .filter(|item| included.contains(item.id.as_str()))
        .cloned()
        .collect()
}

pub fn redact_sensitive_owner_data(body: &str) -> String {
    if has_sensitive_owner_data(body) {
        "[sensitive owner data redacted]".to_string()
    } else {
        body.to_string()
    }
}

pub fn contamination_for_observation(
    effect_name: &str,
    status: &str,
    body: &str,
) -> ContaminationClass {
    if status != "ok" {
        return ContaminationClass::RecoveryOnly;
    }
    if has_sensitive_owner_data(body) {
        return ContaminationClass::SensitiveOwnerData;
    }
    match effect_name {
        "shell.run" => ContaminationClass::ExternalRaw,
        "raw-tool-log" => ContaminationClass::RawToolLog,
        _ => ContaminationClass::Clean,
    }
}

fn has_sensitive_owner_data(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    [
        "password=",
        "password:",
        "secret=",
        "secret:",
        "token=",
        "token:",
        "api_key=",
        "api_key:",
        "apikey=",
        "apikey:",
        "\"password\"",
        "\"secret\"",
        "\"token\"",
        "\"api_key\"",
        "\"apikey\"",
        "\"authorization\"",
        "authorization: bearer",
        "authorization=bearer",
    ]
    .iter()
    .any(|pattern| lower.contains(pattern))
}

pub fn detect_contradictions(items: &[ContextItem]) -> Vec<ContextConflict> {
    let mut bodies_by_key: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();
    for item in items
        .iter()
        .filter(|item| item.is_normal_prompt_candidate())
    {
        bodies_by_key
            .entry(item.semantic_key.clone())
            .or_default()
            .entry(normalized_body(&item.body))
            .or_default()
            .insert(item.id.clone());
    }
    bodies_by_key
        .into_iter()
        .filter_map(|(semantic_key, bodies)| conflict_from_bodies(semantic_key, bodies))
        .collect()
}

#[rustfmt::skip]
pub(crate) fn normalized_body(body: &str) -> String {
    body.chars().map(|value| if value.is_alphanumeric() { value.to_ascii_lowercase() } else { ' ' })
        .collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

fn conflict_from_bodies(
    semantic_key: String,
    bodies: BTreeMap<String, BTreeSet<String>>,
) -> Option<ContextConflict> {
    if bodies.len() < 2 {
        return None;
    }
    let item_ids = bodies
        .into_values()
        .flat_map(|ids| ids.into_iter())
        .collect();
    Some(ContextConflict {
        semantic_key,
        item_ids,
    })
}
