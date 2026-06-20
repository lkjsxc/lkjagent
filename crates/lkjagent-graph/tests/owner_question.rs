use lkjagent_graph::case_fields::{FieldStatus, QuestionRecord};
use lkjagent_graph::{initial_state, render_graph_slice, source_graph, GraphNodeId};

#[test]
fn graph_slice_hides_agent_ask_without_owner_question() {
    let mut state = initial_state("Recover without owner input.", Some(1));
    state.active_node = GraphNodeId("recover-params");

    let rendered = render_graph_slice(source_graph(), &state, 4096);
    let allowed = rendered
        .lines()
        .find(|line| line.starts_with("Allowed tools now:"))
        .unwrap_or("");

    assert!(allowed.contains("graph.state, fs.list"));
    assert!(!allowed.contains("agent.ask"));
}

#[test]
fn graph_slice_allows_agent_ask_with_owner_required_question() {
    let mut state = initial_state("Recover with owner input.", Some(2));
    state.active_node = GraphNodeId("recover-params");
    state.open_questions.push(QuestionRecord {
        question: "Which target should be edited?".to_string(),
        status: FieldStatus::Open,
        owner_required: true,
    });

    let rendered = render_graph_slice(source_graph(), &state, 4096);

    assert!(rendered.contains("agent.ask"));
}
