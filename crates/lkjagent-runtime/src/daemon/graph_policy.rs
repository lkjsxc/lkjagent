use rusqlite::Connection;

use crate::mode::{ActiveMode, ActiveModePolicy};
use lkjagent_graph::{admitted_targets, source_graph, TaskGraphState, TransitionDecision};
use lkjagent_tools::dispatch::{EffectivePolicy, GraphDispatchPolicy};

pub(super) fn completion_decision(conn: &Connection, graph: &TaskGraphState) -> TransitionDecision {
    let graph_decision = lkjagent_graph::completion_decision(graph);
    if unresolved_artifact_ledger(conn, graph) {
        return TransitionDecision::Defer {
            missing: vec!["artifact-readiness".to_string()],
        };
    }
    graph_decision
}

pub(super) fn effective_policy(
    mode_policy: &ActiveModePolicy,
    graph_policy: Option<&GraphDispatchPolicy>,
) -> EffectivePolicy {
    if mode_policy.graph_policy_applies {
        if let Some(graph) = graph_policy {
            let allowed_tools = effective_allowed_tools(mode_policy, graph);
            let blocked_tools = effective_blocked_tools(graph, &allowed_tools);
            return EffectivePolicy {
                mode: format!("{:?}", mode_policy.mode),
                allowed_tools,
                blocked_tools,
                shell_allowed: graph.shell_allowed,
                completion_allowed: graph.completion_ready,
                reason: graph
                    .blocked_reason
                    .clone()
                    .unwrap_or_else(|| "tool is not admitted by the active graph node".to_string()),
                preferred_next_action: mode_policy.preferred_next_action.to_string(),
            };
        }
    }
    EffectivePolicy {
        mode: format!("{:?}", mode_policy.mode),
        allowed_tools: strings(&mode_policy.allowed_tools),
        blocked_tools: strings(&mode_policy.blocked_tools),
        shell_allowed: mode_policy.allowed_tools.contains(&"shell.run"),
        completion_allowed: mode_policy.mode.allows_completion(),
        reason: format!("tool is not admitted by {:?} active mode", mode_policy.mode),
        preferred_next_action: mode_policy.preferred_next_action.to_string(),
    }
}

pub(super) fn policy_for(graph: &TaskGraphState) -> GraphDispatchPolicy {
    let source = source_graph();
    let node = source
        .nodes
        .iter()
        .find(|node| node.id == graph.active_node);
    let owner_question = graph.open_questions.iter().any(|question| {
        question.owner_required && question.status == lkjagent_graph::case_fields::FieldStatus::Open
    });
    let allowed = node.map_or_else(Vec::new, |node| {
        node.allowed_actions
            .iter()
            .filter(|tool| **tool != "agent.ask" || owner_question)
            .map(|tool| (*tool).to_string())
            .collect()
    });
    let allowed_packages = node.map_or_else(Vec::new, |node| {
        node.packages.iter().map(|id| (*id).to_string()).collect()
    });
    let blocked = source
        .nodes
        .iter()
        .flat_map(|node| node.allowed_actions.iter().copied())
        .filter(|tool| !allowed.iter().any(|allowed| allowed == tool))
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    GraphDispatchPolicy {
        active_node: graph.active_node.0.to_string(),
        phase: graph.phase.as_str().to_string(),
        allowed_tools: allowed,
        blocked_tools: blocked,
        allowed_packages,
        legal_transitions: admitted_targets(&source, graph)
            .iter()
            .map(|node| node.0.to_string())
            .collect(),
        evidence_requirements: graph.evidence.requirement_ids(),
        blocked_reason: graph.completion.refusal_reason.clone(),
        plan_ready: graph.plan.ready,
        completion_ready: graph.completion.ready,
        shell_allowed: source
            .policy
            .shell_allowed_nodes
            .contains(&graph.active_node.0),
    }
}

fn unresolved_artifact_ledger(conn: &Connection, graph: &TaskGraphState) -> bool {
    if !graph.evidence.knows_requirement("artifact-readiness") {
        return false;
    }
    if !graph.evidence.has("artifact-readiness") {
        return false;
    }
    let Some(case_id) = graph.case_id else {
        return true;
    };
    match lkjagent_store::artifact_ledger::latest_for_case(conn, case_id) {
        Ok(Some(row)) => row.readiness_status != "passed" || row.weak_path_count != 0,
        _ => true,
    }
}

fn effective_allowed_tools(
    mode_policy: &ActiveModePolicy,
    graph: &GraphDispatchPolicy,
) -> Vec<String> {
    let mut allowed = graph.allowed_tools.clone();
    for tool in authority_escape_tools(mode_policy.mode, graph) {
        if !allowed.iter().any(|existing| existing == tool) {
            allowed.push((*tool).to_string());
        }
    }
    allowed
}

fn effective_blocked_tools(graph: &GraphDispatchPolicy, allowed: &[String]) -> Vec<String> {
    graph
        .blocked_tools
        .iter()
        .filter(|tool| !allowed.iter().any(|allowed| allowed == *tool))
        .cloned()
        .collect()
}

fn authority_escape_tools(
    mode: ActiveMode,
    graph: &GraphDispatchPolicy,
) -> &'static [&'static str] {
    match mode {
        ActiveMode::Recovery => &[
            "graph.recover",
            "graph.transition",
            "artifact.next",
            "artifact.audit",
            "doc.audit",
            "fs.read",
            "fs.list",
            "fs.stat",
            "fs.batch_write",
            "workspace.summary",
        ],
        ActiveMode::OwnerTask if !graph.completion_ready => &[
            "graph.plan",
            "fs.read",
            "fs.list",
            "fs.stat",
            "artifact.audit",
            "artifact.next",
            "doc.audit",
            "fs.batch_write",
            "graph.evidence",
            "workspace.summary",
        ],
        _ => &[],
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
