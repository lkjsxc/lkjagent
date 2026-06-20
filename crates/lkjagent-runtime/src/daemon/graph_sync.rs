use super::runner::ResidentDaemon;
use crate::graph_state::render_state;
use lkjagent_graph::{admitted_targets, source_graph};
use lkjagent_tools::dispatch::GraphDispatchPolicy;

impl ResidentDaemon {
    pub(super) fn sync_graph_dispatch_state(&mut self) {
        let Some(graph) = self.state.graph.as_ref() else {
            self.dispatch_state.graph_state = None;
            self.dispatch_state.graph_completion_ready = true;
            self.dispatch_state.graph_missing.clear();
            self.dispatch_state.graph_policy = None;
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
