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
    let nodes = collect_nodes(graph, &mut violations);
    collect_edges(graph, &nodes, &mut violations);
    collect_packages(graph, &mut violations);
    ValidationReport { violations }
}

fn collect_nodes(graph: GraphDefinition, violations: &mut Vec<String>) -> BTreeSet<GraphNodeId> {
    let mut nodes = BTreeSet::new();
    for node in graph.nodes {
        if !nodes.insert(node.id) {
            violations.push(format!("duplicate node id: {}", node.id.0));
        }
    }
    nodes
}

fn collect_edges(
    graph: GraphDefinition,
    nodes: &BTreeSet<GraphNodeId>,
    violations: &mut Vec<String>,
) {
    let mut edges = BTreeSet::new();
    for edge in graph.edges {
        if !edges.insert(edge.id) {
            violations.push(format!("duplicate edge id: {}", edge.id.0));
        }
        if !nodes.contains(&edge.from) {
            violations.push(format!("edge {} missing source {}", edge.id.0, edge.from.0));
        }
        if !nodes.contains(&edge.to) {
            violations.push(format!("edge {} missing target {}", edge.id.0, edge.to.0));
        }
    }
}

fn collect_packages(graph: GraphDefinition, violations: &mut Vec<String>) {
    let mut packages = BTreeSet::new();
    for package in graph.packages {
        if !packages.insert(package.name) {
            violations.push(format!("duplicate package: {}", package.name));
        }
    }
    for node in graph.nodes {
        for package in node.packages {
            if !packages.contains(package) {
                violations.push(format!("node {} missing package {}", node.id.0, package));
            }
        }
    }
}
