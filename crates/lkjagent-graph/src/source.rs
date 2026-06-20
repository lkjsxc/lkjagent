use crate::model::{ContextPackage, GraphDefinition, GraphEdge, GraphNode};
use crate::policy::DEFAULT_POLICY;

pub fn source_graph() -> GraphDefinition {
    GraphDefinition {
        nodes: all_nodes(),
        edges: crate::source_edges::EDGES.to_vec(),
        packages: crate::source_packages::PACKAGES.to_vec(),
        policy: DEFAULT_POLICY,
    }
}

fn all_nodes() -> Vec<GraphNode> {
    let mut nodes = Vec::new();
    extend(&mut nodes, crate::source_core::NODES);
    extend(&mut nodes, crate::source_intake::NODES);
    extend(&mut nodes, crate::source_context::NODES);
    extend(&mut nodes, crate::source_planning::NODES);
    extend(&mut nodes, crate::source_code::NODES);
    extend(&mut nodes, crate::source_execution::NODES);
    extend(&mut nodes, crate::source_docs::NODES);
    extend(&mut nodes, crate::source_document::NODES);
    extend(&mut nodes, crate::source_verification::NODES);
    extend(&mut nodes, crate::source_compaction::NODES);
    extend(&mut nodes, crate::source_recovery::NODES);
    extend(&mut nodes, crate::source_recovery_extra::NODES);
    extend(&mut nodes, crate::source_completion::NODES);
    extend(&mut nodes, crate::source_maintenance::NODES);
    nodes
}

fn extend<T: Copy>(out: &mut Vec<T>, values: &[T]) {
    out.extend_from_slice(values);
}

#[allow(dead_code)]
fn _type_checks(_: GraphEdge, _: ContextPackage) {}
