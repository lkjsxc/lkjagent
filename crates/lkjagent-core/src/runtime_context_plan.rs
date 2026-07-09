use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::runtime_context::{
    ContaminationClass, ContextConflict, ContextItem, StalenessClass, TrustClass,
};
use crate::runtime_context_pipeline::{default_context_pipeline, ContextPipelineStage};
use crate::runtime_fingerprint::stable_fingerprint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPlanEntry {
    pub item_id: String,
    pub reason: String,
    pub rank: i32,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextLanePlan {
    pub name: String,
    pub budget_tokens: u32,
    pub source_refs: Vec<String>,
    pub included_item_ids: Vec<String>,
    pub excluded_item_ids: Vec<String>,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextFramePlan {
    pub included: Vec<ContextPlanEntry>,
    pub excluded: Vec<ContextPlanEntry>,
    #[serde(default)]
    pub lanes: Vec<ContextLanePlan>,
    #[serde(default)]
    pub pipeline: Vec<ContextPipelineStage>,
}

pub fn select_context_plan(
    items: &[ContextItem],
    conflicts: &[ContextConflict],
) -> ContextFramePlan {
    let conflict_keys = conflicts
        .iter()
        .map(|conflict| conflict.semantic_key.as_str())
        .collect::<BTreeSet<_>>();
    let mut plan = ContextFramePlan {
        included: Vec::new(),
        excluded: Vec::new(),
        lanes: Vec::new(),
        pipeline: default_context_pipeline(),
    };
    let mut seen = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        match suppression_reason(item, &conflict_keys) {
            Some(reason) => plan.excluded.push(plan_entry(item, &reason, index)),
            None if !seen.insert(dedup_key(item)) => {
                plan.excluded
                    .push(plan_entry(item, "duplicate-context", index));
            }
            None => plan.included.push(plan_entry(item, "clean-current", index)),
        }
    }
    plan.lanes = build_lanes(items, &plan);
    plan
}

fn suppression_reason(item: &ContextItem, conflict_keys: &BTreeSet<&str>) -> Option<String> {
    if item.staleness != StalenessClass::Current {
        return Some(format!("staleness:{:?}", item.staleness));
    }
    if item.contamination != ContaminationClass::Clean {
        return Some(format!("contamination:{:?}", item.contamination));
    }
    if conflict_keys.contains(item.semantic_key.as_str()) {
        return Some("unresolved-conflict".to_string());
    }
    None
}

fn build_lanes(items: &[ContextItem], plan: &ContextFramePlan) -> Vec<ContextLanePlan> {
    vec![
        lane("relevant-records", 1_200, items, &plan.included),
        lane("excluded-context-notes", 300, items, &plan.excluded),
    ]
}

fn lane(
    name: &str,
    budget_tokens: u32,
    items: &[ContextItem],
    entries: &[ContextPlanEntry],
) -> ContextLanePlan {
    let ids: Vec<String> = entries.iter().map(|entry| entry.item_id.clone()).collect();
    let source_refs = entries
        .iter()
        .filter_map(|entry| items.iter().find(|item| item.id == entry.item_id))
        .map(|item| format!("{}:{}", item.source_type, item.source_id))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let excluded_lane = name.starts_with("excluded");
    let mut plan = ContextLanePlan {
        name: name.to_string(),
        budget_tokens,
        source_refs,
        included_item_ids: if excluded_lane {
            Vec::new()
        } else {
            ids.clone()
        },
        excluded_item_ids: if excluded_lane { ids } else { Vec::new() },
        fingerprint: String::new(),
    };
    plan.fingerprint = lane_fingerprint(&plan);
    plan
}

fn lane_fingerprint(plan: &ContextLanePlan) -> String {
    stable_fingerprint(plan).unwrap_or_else(|error| format!("fingerprint-error:{}", error.message))
}

fn dedup_key(item: &ContextItem) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        item.semantic_key, item.body, item.source_type, item.source_fingerprint
    )
}

fn plan_entry(item: &ContextItem, reason: &str, index: usize) -> ContextPlanEntry {
    ContextPlanEntry {
        item_id: item.id.clone(),
        reason: reason.to_string(),
        rank: item_rank(item, index),
        source_ref: source_ref(item),
    }
}

fn item_rank(item: &ContextItem, index: usize) -> i32 {
    let trust = match item.trust {
        TrustClass::Owner => 500,
        TrustClass::Measured => 400,
        TrustClass::Memory => 300,
        TrustClass::Recovery => 250,
        TrustClass::Model => 100,
        TrustClass::External => 50,
    };
    let freshness = i32::from(item.staleness == StalenessClass::Current) * 100;
    let cleanliness = i32::from(item.contamination == ContaminationClass::Clean) * 50;
    trust + freshness + cleanliness - index as i32
}

fn source_ref(item: &ContextItem) -> String {
    let id = if item.source_id.is_empty() {
        "none"
    } else {
        &item.source_id
    };
    let fp = if item.source_fingerprint.is_empty() {
        "none"
    } else {
        &item.source_fingerprint
    };
    format!("{}:{id}@{fp}", item.source_type)
}
