use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::runtime_context::{ContaminationClass, ContextConflict, ContextItem, StalenessClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPlanEntry {
    pub item_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextFramePlan {
    pub included: Vec<ContextPlanEntry>,
    pub excluded: Vec<ContextPlanEntry>,
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
    };
    for item in items {
        match suppression_reason(item, &conflict_keys) {
            Some(reason) => plan.excluded.push(plan_entry(item, &reason)),
            None => plan.included.push(plan_entry(item, "clean-current")),
        }
    }
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

fn plan_entry(item: &ContextItem, reason: &str) -> ContextPlanEntry {
    ContextPlanEntry {
        item_id: item.id.clone(),
        reason: reason.to_string(),
    }
}
