use serde_json::Value;

use crate::runtime_decision::OutputEnvelope;
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_selector as selector;
use crate::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[rustfmt::skip]
const WORKSPACE_FAMILIES: &[(&str, &str, u8, &str)] = &[("todo", "todo.review", 35, "todo"), ("calendar", "calendar.review", 36, "calendar"), ("routine", "routine.run", 37, "routine"), ("index", "index.rebuild", 38, "index"), ("proof", "proof.collect", 39, "proof"), ("dev", "dev.review", 40, "dev"), ("project", "project.advance", 41, "project"), ("finance", "finance.review", 42, "finance")];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorCandidate {
    pub operation: RuntimeOperation,
    pub state_key: Option<StateKey>,
    pub reason: String,
    pub blocked_by: Vec<String>,
    score: CandidateScore,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CandidateScore {
    tier: u8,
    priority_rank: i32,
    deadline: String,
    key: String,
}

pub fn selector_candidates(snapshot: &RuntimeSnapshot) -> Vec<SelectorCandidate> {
    selector_candidates_for(snapshot, None)
}

pub fn selector_candidates_at(snapshot: &RuntimeSnapshot, now: &str) -> Vec<SelectorCandidate> {
    selector_candidates_for(snapshot, Some(now))
}

#[rustfmt::skip]
fn selector_candidates_for(snapshot: &RuntimeSnapshot, now: Option<&str>) -> Vec<SelectorCandidate> {
    let mut candidates = snapshot.active_cells().into_iter()
        .filter(|cell| crate::runtime_eligibility::cell_is_due(cell, now)).filter_map(cell_candidate)
        .map(|item| selector::apply_edge_blocks(snapshot, item)).collect::<Vec<_>>();
    if candidates.is_empty() {
        let fallback = if crate::runtime_eligibility::has_invalid_cooldown(snapshot) { blocked_candidate(&[]) }
            else { now.and_then(|value| crate::runtime_eligibility::next_wake(snapshot, value)).map_or_else(idle_candidate, wait_candidate) };
        candidates.push(fallback);
    }
    candidates.sort_by(|left, right| left.score.cmp(&right.score)); candidates
}

pub fn selected_candidate(snapshot: &RuntimeSnapshot) -> SelectorCandidate {
    select_candidate(selector_candidates(snapshot))
}

pub fn selected_candidate_at(snapshot: &RuntimeSnapshot, now: &str) -> SelectorCandidate {
    select_candidate(selector_candidates_at(snapshot, now))
}

#[rustfmt::skip]
fn select_candidate(candidates: Vec<SelectorCandidate>) -> SelectorCandidate {
    candidates.iter().find(|item| item.blocked_by.is_empty()).cloned()
        .unwrap_or_else(|| blocked_candidate(&candidates))
}

#[rustfmt::skip]
fn cell_candidate(cell: &StateCell) -> Option<SelectorCandidate> {
    let body = selector::payload_value(cell);
    if let Some(operation) = operation_from_payload(&body) {
        return Some(candidate(cell, operation, selector::payload_tier(&body, 80), "payload"));
    }
    match (cell.key.namespace.as_str(), cell.key.name.as_str()) {
        ("case", "owner-intake") => Some(model_free(cell, "owner.intake", 10, "owner intake")),
        ("case", "waiting-answer") => Some(model_free(cell, "owner.answer", 20, "owner answer")),
        ("completion", "blocked") => Some(model_free(cell, "completion.blocked", 65, "completion blocked")),
        ("completion", "close-candidate") => Some(model_free(cell, "completion.close", 70, "completion candidate")),
        _ => namespace_candidate(cell, &body),
    }
}

#[rustfmt::skip]
fn namespace_candidate(cell: &StateCell, body: &Value) -> Option<SelectorCandidate> {
    match cell.key.namespace.as_str() {
        "recovery" => Some(model_free(cell, &format!("recovery.handle/{}", cell.key.name), 30, "recovery")),
        "effect" => Some(model_free(cell, &format!("effect.run/{}", cell.key.name), 40, "effect")),
        "model" => Some(candidate(cell, RuntimeOperation::model_call(format!("model.call/{}", cell.key.name),
            selector::payload_envelope(body), selector::payload_tool_view(body),
            selector::payload_number(body, "model_budget_tokens"), evidence(cell)), 50, "model")),
        "check" => Some(model_free(cell, &format!("check.run/{}", cell.key.name), 60, "check")),
        _ => workspace_family_operation(&cell.key.namespace)
            .map(|(operation, tier, reason)| model_free(cell, &format!("{operation}/{}", cell.key.name), tier, reason)),
    }
}

#[rustfmt::skip]
fn workspace_family_operation(namespace: &str) -> Option<(&'static str, u8, &'static str)> {
    WORKSPACE_FAMILIES.iter().find(|(name, _, _, _)| *name == namespace).map(|(_, operation, tier, reason)| (*operation, *tier, *reason))
}

#[rustfmt::skip]
fn operation_from_payload(body: &Value) -> Option<RuntimeOperation> {
    let key = selector::payload_text(body, "operation_key")?; let expected = selector::payload_envelope(body);
    let mut operation = if expected == OutputEnvelope::None {
        RuntimeOperation::model_free_effect(key, selector::payload_evidence_requirements(body), selector::payload_effect_command(body))
    } else {
        RuntimeOperation::model_call(key, expected, selector::payload_tool_view(body),
            selector::payload_number(body, "model_budget_tokens"), selector::payload_evidence_requirements(body))
    };
    if let Some(policy) = selector::payload_text(body, "recovery_policy") { operation.recovery_policy = policy.to_string(); }
    Some(operation)
}

#[rustfmt::skip]
fn model_free(cell: &StateCell, key: &str, tier: u8, reason: &str) -> SelectorCandidate {
    candidate(cell, RuntimeOperation::model_free(key, evidence(cell)), tier, reason)
}

fn candidate(
    cell: &StateCell,
    operation: RuntimeOperation,
    tier: u8,
    reason: &str,
) -> SelectorCandidate {
    let body = selector::payload_value(cell);
    SelectorCandidate {
        operation,
        state_key: Some(cell.key.clone()),
        reason: reason.to_string(),
        blocked_by: Vec::new(),
        score: CandidateScore {
            tier,
            priority_rank: -cell.priority,
            deadline: selector::payload_text(&body, "deadline_at")
                .unwrap_or("~")
                .to_string(),
            key: cell.key.as_label(),
        },
    }
}

fn idle_candidate() -> SelectorCandidate {
    synthetic(
        RuntimeOperation::idle(),
        "no executable state candidate",
        "runtime:idle",
        100,
    )
}

fn wait_candidate(due: &str) -> SelectorCandidate {
    synthetic(
        RuntimeOperation::model_free("runtime.wait", vec![format!("wake:{due}")]),
        &format!("waiting until {due}"),
        "runtime:wait",
        99,
    )
}

fn blocked_candidate(items: &[SelectorCandidate]) -> SelectorCandidate {
    let evidence = items
        .iter()
        .flat_map(|item| item.blocked_by.clone())
        .collect();
    synthetic(
        RuntimeOperation::model_free("completion.blocked", evidence),
        "all state candidates are blocked",
        "completion:blocked",
        65,
    )
}

fn synthetic(operation: RuntimeOperation, reason: &str, key: &str, tier: u8) -> SelectorCandidate {
    SelectorCandidate {
        operation,
        state_key: None,
        reason: reason.to_string(),
        blocked_by: Vec::new(),
        score: CandidateScore {
            tier,
            priority_rank: 0,
            deadline: "~".to_string(),
            key: key.to_string(),
        },
    }
}

fn evidence(cell: &StateCell) -> Vec<String> {
    if cell.evidence_refs.is_empty() {
        return vec![cell.key.as_label()];
    }
    cell.evidence_refs
        .iter()
        .map(|item| format!("{}:{}", item.source_type, item.source_id))
        .collect()
}
