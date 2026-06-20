use std::collections::BTreeSet;

use lkjagent_graph::{source_graph, GraphNodeId};

const REQUIRED_RECOVERY_NODES: &[GraphNodeId] = &[
    GraphNodeId("recover"),
    GraphNodeId("recover-parse"),
    GraphNodeId("recover-params"),
    GraphNodeId("recover-tool"),
    GraphNodeId("recover-repeat"),
    GraphNodeId("recover-endpoint"),
    GraphNodeId("recover-budget"),
    GraphNodeId("recover-verification"),
    GraphNodeId("recover-context"),
    GraphNodeId("recover-by-state-inspection"),
    GraphNodeId("recover-by-alternate-tool"),
    GraphNodeId("recover-by-smaller-scope"),
    GraphNodeId("recover-by-artifact-plan"),
    GraphNodeId("recover-by-bounded-write"),
    GraphNodeId("recover-by-shell-escape"),
    GraphNodeId("repair-step"),
    GraphNodeId("owner-question"),
    GraphNodeId("compact-soft"),
    GraphNodeId("compact-hard"),
    GraphNodeId("compact-boundary"),
    GraphNodeId("rebuild-context"),
    GraphNodeId("resume-after-compact"),
];

#[test]
fn all_edge_targets_have_nodes() {
    let graph = source_graph();
    let nodes = node_ids();

    for edge in graph.edges {
        assert!(nodes.contains(&edge.from), "missing source {}", edge.from.0);
        assert!(nodes.contains(&edge.to), "missing target {}", edge.to.0);
    }
}

#[test]
fn required_recovery_nodes_exist() {
    let nodes = node_ids();

    for id in REQUIRED_RECOVERY_NODES {
        assert!(nodes.contains(id), "missing node {}", id.0);
    }
}

#[test]
fn recovery_nodes_have_recovery_package() {
    let graph = source_graph();

    for node in graph
        .nodes
        .iter()
        .filter(|node| recovery_ladder_node(node.id))
    {
        assert!(
            node.packages.contains(&"recovery-policy"),
            "node {} lacks recovery-policy",
            node.id.0
        );
    }
}

fn recovery_ladder_node(id: GraphNodeId) -> bool {
    id.0.starts_with("recover") || id == GraphNodeId("owner-question")
}

#[test]
fn compaction_nodes_have_compaction_package() {
    let graph = source_graph();

    for id in [
        GraphNodeId("compact-soft"),
        GraphNodeId("compact-hard"),
        GraphNodeId("compact-boundary"),
        GraphNodeId("rebuild-context"),
        GraphNodeId("resume-after-compact"),
    ] {
        let node = graph.nodes.iter().find(|node| node.id == id);
        assert!(
            node.is_some_and(|item| item.packages.contains(&"compaction-preserve")),
            "node {} lacks compaction-preserve",
            id.0
        );
    }
}

#[test]
fn recovery_packages_apply_to_owner_question() {
    let graph = source_graph();
    let package = graph
        .packages
        .iter()
        .find(|package| package.id.0 == "recovery-policy");

    assert!(
        package.is_some_and(|item| { item.applies_to.contains(&GraphNodeId("owner-question")) })
    );
}

#[test]
fn artifact_recovery_admits_bounded_artifact_tools() {
    let graph = source_graph();
    let node = graph
        .nodes
        .iter()
        .find(|node| node.id == GraphNodeId("recover-by-artifact-plan"));
    assert!(node.is_some(), "missing artifact recovery node");
    if let Some(node) = node {
        for tool in [
            "artifact.plan",
            "artifact.apply",
            "artifact.next",
            "doc.scaffold",
            "fs.batch_write",
        ] {
            assert!(node.allowed_actions.contains(&tool), "missing {tool}");
        }
        assert!(!node.allowed_actions.contains(&"fs.write"));
    }
}

fn node_ids() -> BTreeSet<GraphNodeId> {
    source_graph()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect()
}
