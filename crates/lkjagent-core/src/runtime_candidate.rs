use serde_json::Value;

use crate::runtime_decision::OutputEnvelope;
use crate::runtime_operation::RuntimeOperation;
use crate::runtime_selector as selector;
use crate::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

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
    let mut candidates = snapshot
        .active_cells()
        .into_iter()
        .filter_map(cell_candidate)
        .map(|candidate| selector::apply_edge_blocks(snapshot, candidate))
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        candidates.push(idle_candidate());
    }
    candidates.sort_by(|left, right| left.score.cmp(&right.score));
    candidates
}

pub fn selected_candidate(snapshot: &RuntimeSnapshot) -> SelectorCandidate {
    selector_candidates(snapshot)
        .into_iter()
        .find(|candidate| candidate.blocked_by.is_empty())
        .unwrap_or_else(idle_candidate)
}

fn cell_candidate(cell: &StateCell) -> Option<SelectorCandidate> {
    let body = selector::payload_value(cell);
    if cell.cooldown_until.is_some() {
        return Some(candidate(cell, RuntimeOperation::idle(), 98, "cooldown"));
    }
    if let Some(operation) = operation_from_payload(&body) {
        return Some(candidate(
            cell,
            operation,
            selector::payload_tier(&body, 80),
            "payload",
        ));
    }
    match (cell.key.namespace.as_str(), cell.key.name.as_str()) {
        ("case", "owner-intake") => Some(model_free(cell, "owner.intake", 10, "owner intake")),
        ("case", "waiting-answer") => Some(model_free(cell, "owner.answer", 20, "owner answer")),
        ("completion", "blocked") => Some(model_free(
            cell,
            "completion.blocked",
            65,
            "completion blocked",
        )),
        ("completion", "close-candidate") => Some(model_free(
            cell,
            "completion.close",
            70,
            "completion candidate",
        )),
        _ => namespace_candidate(cell, &body),
    }
}

fn namespace_candidate(cell: &StateCell, body: &Value) -> Option<SelectorCandidate> {
    match cell.key.namespace.as_str() {
        "recovery" => Some(model_free(
            cell,
            &format!("recovery.handle/{}", cell.key.name),
            30,
            "recovery",
        )),
        "effect" => Some(model_free(
            cell,
            &format!("effect.run/{}", cell.key.name),
            40,
            "effect",
        )),
        "model" => Some(candidate(
            cell,
            RuntimeOperation::model_call(
                format!("model.call/{}", cell.key.name),
                selector::payload_envelope(body),
                selector::payload_tool_view(body),
                selector::payload_number(body, "model_budget_tokens"),
                evidence(cell),
            ),
            50,
            "model",
        )),
        "check" => Some(model_free(
            cell,
            &format!("check.run/{}", cell.key.name),
            60,
            "check",
        )),
        _ => workspace_candidate(cell),
    }
}

fn workspace_candidate(cell: &StateCell) -> Option<SelectorCandidate> {
    let (operation, tier, reason) =
        crate::runtime_workspace_family::operation(&cell.key.namespace)?;
    Some(model_free(
        cell,
        &format!("{operation}/{}", cell.key.name),
        tier,
        reason,
    ))
}

fn operation_from_payload(body: &Value) -> Option<RuntimeOperation> {
    let key = selector::payload_text(body, "operation_key")?;
    let expected = selector::payload_envelope(body);
    if expected == OutputEnvelope::None {
        Some(RuntimeOperation::model_free(
            key,
            selector::payload_evidence_requirements(body),
        ))
    } else {
        Some(RuntimeOperation::model_call(
            key,
            expected,
            selector::payload_tool_view(body),
            selector::payload_number(body, "model_budget_tokens"),
            selector::payload_evidence_requirements(body),
        ))
    }
}

fn model_free(cell: &StateCell, key: &str, tier: u8, reason: &str) -> SelectorCandidate {
    candidate(
        cell,
        RuntimeOperation::model_free(key, evidence(cell)),
        tier,
        reason,
    )
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
    SelectorCandidate {
        operation: RuntimeOperation::idle(),
        state_key: None,
        reason: "no executable state candidate".to_string(),
        blocked_by: Vec::new(),
        score: CandidateScore {
            tier: 100,
            priority_rank: 0,
            deadline: "~".to_string(),
            key: "runtime:idle".to_string(),
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
