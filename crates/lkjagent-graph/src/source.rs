use crate::model::GraphDefinition;
use crate::source_edges::EDGES;
use crate::source_nodes::NODES;
use crate::source_packages::PACKAGES;

pub fn source_graph() -> GraphDefinition {
    GraphDefinition {
        nodes: NODES,
        edges: EDGES,
        packages: PACKAGES,
    }
}
