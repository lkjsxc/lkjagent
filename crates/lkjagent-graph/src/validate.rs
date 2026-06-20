use std::collections::BTreeSet;

use crate::model::{GraphDefinition, GraphNodeId};

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

fn collect_tools(graph: &GraphDefinition, violations: &mut Vec<String>) {
    for node in &graph.nodes {
        for tool in node.allowed_actions {
            if !KNOWN_TOOLS.contains(tool) {
                violations.push(format!("node {} unknown tool {}", node.id.0, tool));
            }
        }
    }
}

const KNOWN_TOOLS: &[&str] = &[
    "fs.read",
    "fs.write",
    "fs.edit",
    "fs.list",
    "fs.search",
    "fs.stat",
    "fs.mkdir",
    "fs.batch_write",
    "shell.run",
    "queue.list",
    "queue.enqueue",
    "queue.edit",
    "queue.delete",
    "queue.redeliver",
    "memory.save",
    "memory.find",
    "graph.state",
    "graph.plan",
    "graph.transition",
    "graph.context",
    "graph.note",
    "graph.evidence",
    "graph.compact",
    "workspace.summary",
    "verify.cargo",
    "verify.xtask",
    "doc.scaffold",
    "doc.audit",
    "agent.done",
    "agent.ask",
];
