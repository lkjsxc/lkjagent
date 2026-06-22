use super::runner::ResidentDaemon;
use crate::graph_state::render_state;
use crate::mode::{ActiveMode, ActiveModePolicy};
use lkjagent_graph::{admitted_targets, source_graph};
use lkjagent_tools::dispatch::{EffectivePolicy, GraphDispatchPolicy};

impl ResidentDaemon {
    pub(super) fn clear_graph_dispatch_state(&mut self) {
        self.dispatch_state.graph_state = None;
        self.dispatch_state.graph_completion_ready = true;
        self.dispatch_state.graph_missing.clear();
        self.dispatch_state.graph_policy = None;
        self.dispatch_state.effective_policy = None;
    }

    pub(super) fn sync_graph_dispatch_state(&mut self) {
        let Some(graph) = self.state.graph.as_ref() else {
            self.clear_graph_dispatch_state();
            return;
        };
        self.dispatch_state.graph_state = Some(render_state(graph));
        self.dispatch_state.graph_policy = Some(policy_for(graph));
        match lkjagent_graph::completion_decision(graph) {
            lkjagent_graph::TransitionDecision::Admit { .. } => {
                self.dispatch_state.graph_completion_ready = true;
                self.dispatch_state.graph_missing.clear();
            }
            lkjagent_graph::TransitionDecision::Defer { missing } => {
                self.dispatch_state.graph_completion_ready = false;
                self.dispatch_state.graph_missing = missing;
            }
            lkjagent_graph::TransitionDecision::Recover { reason, .. }
            | lkjagent_graph::TransitionDecision::Refuse { reason } => {
                self.dispatch_state.graph_completion_ready = false;
                self.dispatch_state.graph_missing = vec![reason];
            }
        }
    }

    pub(super) fn sync_effective_dispatch_policy(&mut self, mode_policy: &ActiveModePolicy) {
        if mode_policy.graph_policy_applies {
            self.sync_graph_dispatch_state();
        } else {
            self.clear_graph_dispatch_state();
        }
        self.dispatch_state.effective_policy = Some(effective_policy(
            mode_policy,
            self.dispatch_state.graph_policy.as_ref(),
        ));
    }
}

fn effective_policy(
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
        allowed_tools: mode_policy
            .allowed_tools
            .iter()
            .map(|tool| (*tool).to_string())
            .collect(),
        blocked_tools: mode_policy
            .blocked_tools
            .iter()
            .map(|tool| (*tool).to_string())
            .collect(),
        shell_allowed: mode_policy.allowed_tools.contains(&"shell.run"),
        completion_allowed: mode_policy.mode.allows_completion(),
        reason: format!("tool is not admitted by {:?} active mode", mode_policy.mode),
        preferred_next_action: mode_policy.preferred_next_action.to_string(),
    }
}

fn effective_allowed_tools(
    mode_policy: &ActiveModePolicy,
    graph: &GraphDispatchPolicy,
) -> Vec<String> {
    let mut allowed = graph.allowed_tools.clone();
    let escape_tools = authority_escape_tools(mode_policy.mode, graph);
    for tool in escape_tools {
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

fn policy_for(graph: &lkjagent_graph::TaskGraphState) -> GraphDispatchPolicy {
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
