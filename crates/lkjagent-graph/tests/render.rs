use lkjagent_graph::{initial_state, render_graph_slice, source_graph};

#[test]
fn render_graph_slice_names_allowed_and_blocked_tools() {
    let state = initial_state("write docs", None);
    let rendered = render_graph_slice(source_graph(), &state, 512);

    assert!(rendered.contains("phase: planning"));
    assert!(rendered.contains("Missing evidence: plan"));
    assert!(rendered.contains("Allowed tools now:"));
    assert!(rendered.contains("Blocked tools now:"));
    assert!(rendered.contains("graph.plan"));
    assert!(rendered.contains("fs.write"));
    assert!(rendered.contains("Legal transitions:"));
}

#[test]
fn graph_slice_does_not_render_objective_counter_prefix() {
    let state = initial_state("write a bread cookbook", Some(10));
    let rendered = render_graph_slice(source_graph(), &state, 512);

    assert!(rendered.contains("Objective:"));
    assert!(!rendered.contains("Objective: v1"));
    assert!(!rendered.contains("Objective: v10"));
}
