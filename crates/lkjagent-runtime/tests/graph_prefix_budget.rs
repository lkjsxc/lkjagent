mod support;

use lkjagent_context::budget::PREFIX_GRAPH_STATE;
use lkjagent_context::model::{FrameKind, PrefixSection};
use lkjagent_runtime::daemon::build_prefix_from_store;
use lkjagent_runtime::graph_state::open_owner_case;
use lkjagent_store::state;
use support::{store, temp_workspace, TestResult};

#[test]
fn startup_graph_prefix_keeps_guard_inside_section_budget() -> TestResult<()> {
    let conn = store()?;
    let owner = "Create a structured Chronos Fracture story bible with request, project, setting, characters, plot, continuity, style, manuscript, relations, and checks directories. ".repeat(10);
    open_owner_case(&conn, &owner, "2026-06-25T00:00:00Z")?;
    state::set(&conn, "completion guard", "file-count-about:100")?;
    let workspace = temp_workspace("graph-prefix-budget")?;

    let prefix = build_prefix_from_store(&conn, &workspace)?;
    let graph = prefix
        .iter()
        .find(|frame| frame.kind == FrameKind::Prefix(PrefixSection::GraphState))
        .ok_or("missing graph state frame")?;

    assert!(graph.tokens.0 <= PREFIX_GRAPH_STATE);
    assert!(graph
        .content
        .contains("completion_guard=file-count-about:100"));
    Ok(())
}
