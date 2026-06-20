use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::guards::Guard;
use crate::model::{GraphDefinition, GraphNodeId, NodeKind};
use crate::validate_tools::collect_tools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub violations: Vec<String>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}
pub fn validate_graph(graph: GraphDefinition) -> ValidationReport {
    let mut violations = Vec::new();
    let nodes = collect_nodes(&graph, &mut violations);
    collect_edges(&graph, &nodes, &mut violations);
    collect_packages(&graph, &mut violations);
    collect_tools(&graph, &mut violations);
    collect_reachability(&graph, &mut violations);
    collect_shell_policy(&graph, &nodes, &mut violations);
    collect_completion_gates(&graph, &mut violations);
    ValidationReport { violations }
}

fn collect_nodes(graph: &GraphDefinition, violations: &mut Vec<String>) -> BTreeSet<GraphNodeId> {
    let mut nodes = BTreeSet::new();
    for node in &graph.nodes {
        if !nodes.insert(node.id) {
            violations.push(format!("duplicate node id: {}", node.id.0));
        }
    }
    nodes
}

fn collect_edges(
    graph: &GraphDefinition,
    nodes: &BTreeSet<GraphNodeId>,
    violations: &mut Vec<String>,
) {
    let mut edges = BTreeSet::new();
    for edge in &graph.edges {
        if !edges.insert(edge.id) {
            violations.push(format!("duplicate edge id: {}", edge.id.0));
        }
        if edge.guards.is_empty() {
            violations.push(format!("edge {} has no guards", edge.id.0));
        }
        if !nodes.contains(&edge.from) {
            violations.push(format!("edge {} missing source {}", edge.id.0, edge.from.0));
        }
        if !nodes.contains(&edge.to) {
            violations.push(format!("edge {} missing target {}", edge.id.0, edge.to.0));
        }
        if let Some(target) = edge.policy.recovery_target {
            if !nodes.contains(&target) {
                violations.push(format!(
                    "edge {} missing recovery target {}",
                    edge.id.0, target.0
                ));
            }
        }
    }
}

fn collect_packages(graph: &GraphDefinition, violations: &mut Vec<String>) {
    let mut packages = BTreeSet::new();
    for package in &graph.packages {
        if !packages.insert(package.id.0) {
            violations.push(format!("duplicate package: {}", package.id.0));
        }
    }
    for node in &graph.nodes {
        for package in node.packages {
            if !packages.contains(package) {
                violations.push(format!("node {} missing package {}", node.id.0, package));
            }
        }
    }
}

fn collect_reachability(graph: &GraphDefinition, violations: &mut Vec<String>) {
    let mut starts = reachable_from(graph, GraphNodeId("intake"));
    starts.extend(reachable_from(graph, GraphNodeId("maintain")));
    for node in &graph.nodes {
        if !starts.contains(&node.id) {
            violations.push(format!("unreachable node: {}", node.id.0));
        }
    }
}

fn reachable_from(graph: &GraphDefinition, start: GraphNodeId) -> BTreeSet<GraphNodeId> {
    let mut adjacency: BTreeMap<GraphNodeId, Vec<GraphNodeId>> = BTreeMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.from).or_default().push(edge.to);
    }
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(node) = queue.pop_front() {
        if !seen.insert(node) {
            continue;
        }
        if let Some(next) = adjacency.get(&node) {
            queue.extend(next.iter().copied());
        }
    }
    seen
}

fn collect_shell_policy(
    graph: &GraphDefinition,
    nodes: &BTreeSet<GraphNodeId>,
    violations: &mut Vec<String>,
) {
    for allowed in graph.policy.shell_allowed_nodes {
        if !nodes.contains(&GraphNodeId(allowed)) {
            violations.push(format!("unknown shell-admitted node: {allowed}"));
        }
    }
    for node in &graph.nodes {
        if node.allowed_actions.contains(&"shell.run")
            && !graph.policy.shell_allowed_nodes.contains(&node.id.0)
        {
            violations.push(format!(
                "node {} exposes shell.run without shell policy",
                node.id.0
            ));
        }
    }
}

fn collect_completion_gates(graph: &GraphDefinition, violations: &mut Vec<String>) {
    for node in &graph.nodes {
        if node.kind != NodeKind::Completion {
            continue;
        }
        let gated = graph.edges.iter().any(|edge| {
            edge.to == node.id
                && edge
                    .guards
                    .iter()
                    .any(|guard| matches!(guard, Guard::CompletionReady))
        });
        if !gated && node.id != GraphNodeId("docs-code-consistency") {
            violations.push(format!(
                "completion node lacks completion gate: {}",
                node.id.0
            ));
        }
    }
}
