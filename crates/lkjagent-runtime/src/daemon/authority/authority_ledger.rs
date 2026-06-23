use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use lkjagent_store::runtime_authority::{
    record_decision, record_event, AuthorityDecisionInput, AuthorityEventInput,
};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::mode::{ActiveMode, EndpointDecision, TurnAuthority};

pub(super) struct AuthorityGraphView<'a> {
    pub case_id: &'a str,
    pub node: &'a str,
    pub evidence_gaps: &'a str,
    pub recovery_route: &'a str,
}

pub(super) fn persist_authority_ledger(
    daemon: &ResidentDaemon,
    conn: &Connection,
    authority: &TurnAuthority,
    graph: AuthorityGraphView<'_>,
) -> RuntimeResult<()> {
    let case_id = parse_case_id(graph.case_id);
    let created_at = daemon.runtime.tools.now.as_str();
    let event_payload = event_payload(authority, &graph);
    let event_id = record_event(
        conn,
        &AuthorityEventInput {
            case_id,
            event_kind: event_kind(authority),
            event_payload: &event_payload,
            created_at,
        },
    )?;
    let admitted_tools = strings(&authority.effective_policy.allowed_tools);
    let blocked_tools = strings(&authority.effective_policy.blocked_tools);
    let missing_evidence = missing_evidence(graph.evidence_gaps);
    let active_mode = format!("{:?}", authority.mode);
    let fingerprint = authority_fingerprint(authority, &graph, &admitted_tools, &missing_evidence);
    let completion_refusal = completion_refusal(authority, graph.evidence_gaps);
    let recovery_route = optional(graph.recovery_route);
    let decision_id = record_decision(
        conn,
        &AuthorityDecisionInput {
            case_id,
            event_id,
            mission: authority.mission.as_str(),
            active_mode: &active_mode,
            active_node: graph.node,
            admitted_tools: &admitted_tools,
            blocked_tools: &blocked_tools,
            missing_evidence: &missing_evidence,
            forced_next_action: authority.effective_policy.preferred_next_action,
            exact_valid_example: Some(authority.valid_example.as_str()),
            completion_allowed: completion_allowed(authority, graph.evidence_gaps),
            completion_refusal,
            recovery_route,
            compaction_required: authority.mode == ActiveMode::Compaction,
            maintenance_allowed: authority.mode == ActiveMode::Maintenance,
            authority_fingerprint: &fingerprint,
            created_at,
        },
    )?;
    store_state::set(conn, "authority decision id", &decision_id.to_string())?;
    store_state::set(conn, "authority fingerprint", &fingerprint)?;
    Ok(())
}

fn event_kind(authority: &TurnAuthority) -> &'static str {
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

fn event_payload(authority: &TurnAuthority, graph: &AuthorityGraphView<'_>) -> String {
    format!(
        "endpoint={:?};node={};gaps={};recovery={}",
        authority.endpoint_decision, graph.node, graph.evidence_gaps, graph.recovery_route
    )
}

fn completion_allowed(authority: &TurnAuthority, evidence_gaps: &str) -> bool {
    authority.mode.allows_completion()
        && evidence_gaps == "none"
        && authority.endpoint_decision != EndpointDecision::RuntimeCompact
}

fn completion_refusal<'a>(authority: &TurnAuthority, evidence_gaps: &'a str) -> Option<&'a str> {
    if completion_allowed(authority, evidence_gaps) || evidence_gaps == "none" {
        None
    } else {
        Some(evidence_gaps)
    }
}

fn missing_evidence(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .map(|item| item.trim().to_string())
            .collect()
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn optional(value: &str) -> Option<&str> {
    if value == "none" {
        None
    } else {
        Some(value)
    }
}

fn parse_case_id(value: &str) -> i64 {
    value.parse::<i64>().ok().unwrap_or(0)
}

fn authority_fingerprint(
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
