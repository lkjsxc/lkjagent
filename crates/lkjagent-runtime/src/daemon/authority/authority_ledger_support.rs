use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::mode::{ActiveMode, EndpointDecision, TurnAuthority};

use super::authority_ledger::AuthorityGraphView;

pub(super) struct CaseRef {
    pub scope: &'static str,
    pub id: Option<i64>,
}

pub(super) fn event_kind(authority: &TurnAuthority) -> &'static str {
    if authority.mode == ActiveMode::Compaction {
        "context_pressure_detected"
    } else if authority.input.pending_owner_rows > 0 || authority.input.active_owner_case {
        "owner_message_received"
    } else if authority.input.recoverable_owner_case {
        "tool_error"
    } else if authority.mode == ActiveMode::Maintenance {
        "maintenance_tick"
    } else {
        "turn_budget_checkpoint"
    }
}

pub(super) fn event_payload(authority: &TurnAuthority, graph: &AuthorityGraphView<'_>) -> String {
    format!(
        "endpoint={:?};node={};gaps={};recovery={}",
        authority.endpoint_decision, graph.node, graph.evidence_gaps, graph.recovery_route
    )
}

pub(super) fn completion_allowed(authority: &TurnAuthority, evidence_gaps: &str) -> bool {
    authority.mode.allows_completion()
        && evidence_gaps == "none"
        && authority.endpoint_decision != EndpointDecision::RuntimeCompact
}

pub(super) fn completion_refusal<'a>(
    authority: &TurnAuthority,
    evidence_gaps: &'a str,
) -> Option<&'a str> {
    if completion_allowed(authority, evidence_gaps) || evidence_gaps == "none" {
        None
    } else {
        Some(evidence_gaps)
    }
}

pub(super) fn missing_evidence(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .map(|item| item.trim().to_string())
            .collect()
    }
}

pub(super) fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

pub(super) fn optional(value: &str) -> Option<&str> {
    if value == "none" {
        None
    } else {
        Some(value)
    }
}

pub(super) fn compaction_head(authority: &TurnAuthority) -> Option<&'static str> {
    if authority.mode == ActiveMode::Compaction {
        Some("required")
    } else {
        None
    }
}

pub(super) fn maintenance_state(authority: &TurnAuthority) -> &'static str {
    if authority.mode == ActiveMode::Maintenance {
        "active"
    } else {
        "inactive"
    }
}

pub(super) fn case_ref(value: &str) -> CaseRef {
    if value == "none" {
        return CaseRef {
            scope: "none",
            id: None,
        };
    }
    match value.parse::<i64>() {
        Ok(id) => CaseRef {
            scope: "case",
            id: Some(id),
        },
        Err(_) => CaseRef {
            scope: "external",
            id: None,
        },
    }
}

pub(super) fn authority_fingerprint(
    authority: &TurnAuthority,
    graph: &AuthorityGraphView<'_>,
    tools: &[String],
    missing: &[String],
) -> String {
    let mut hasher = DefaultHasher::new();
    authority.mission.as_str().hash(&mut hasher);
    graph.case_id.hash(&mut hasher);
    graph.node.hash(&mut hasher);
    tools.hash(&mut hasher);
    missing.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
