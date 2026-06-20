use lkjagent_graph::{source_graph, GraphNodeId};

#[test]
fn recover_params_exposes_only_safe_schema_tools() {
    let graph = source_graph();
    let node = graph
        .nodes
        .iter()
        .find(|node| node.id == GraphNodeId("recover-params"))
        .expect("recover-params node");

    assert_eq!(
        node.allowed_actions,
        &["graph.state", "fs.list", "workspace.summary", "agent.ask"]
    );
}
