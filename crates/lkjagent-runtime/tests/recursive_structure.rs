mod support;

use std::fs;

use lkjagent_runtime::daemon::build_prefix_from_store;
use lkjagent_runtime::graph_state::{open_owner_case, render_state};
use lkjagent_tools::dispatch::dispatch;
use support::{action, dispatch_state, store, temp_workspace, tool_runtime, TestResult};

#[test]
fn graph_case_drives_indexed_tree_workflow() -> TestResult<()> {
    let workspace = temp_workspace("recursive-structure")?;
    fs::write(
        workspace.join("AGENTS.md"),
        "# AGENTS.md\n\n## Purpose\n\nTest workspace rules.\n",
    )?;
    let runtime = tool_runtime(workspace.clone())?;

    let mut conn = store()?;
    let graph = open_owner_case(&conn, "build a recursive docs structure", "101")?;
    let prefix = build_prefix_from_store(&conn, &workspace)?;
    assert!(prefix
        .iter()
        .any(|frame| frame.content.contains("phase: planning")));

    let mut state = dispatch_state();
    state.graph_state = Some(render_state(&graph));
    let graph_state = dispatch(&action("graph.state", &[]), &runtime, &mut conn, &mut state);
    assert!(graph_state.content.contains("Required evidence: plan"));

    write_file(
        &runtime,
        &mut conn,
        &mut state,
        "docs/README.md",
        ROOT_README,
    );
    write_file(
        &runtime,
        &mut conn,
        &mut state,
        "docs/system/README.md",
        SYSTEM_README,
    );
    write_file(
        &runtime,
        &mut conn,
        &mut state,
        "docs/system/overview.md",
        OVERVIEW,
    );

    let check = dispatch(
        &action(
            "fs.list",
            &[("path", "docs"), ("depth", "3"), ("kind", "dir")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(check.content.contains("dir docs/system"));
    Ok(())
}

fn write_file(
    runtime: &lkjagent_tools::dispatch::ToolRuntime,
    conn: &mut rusqlite::Connection,
    state: &mut lkjagent_tools::dispatch::DispatchState,
    path: &str,
    content: &str,
) {
    let output = dispatch(
        &action("fs.write", &[("path", path), ("content", content)]),
        runtime,
        conn,
        state,
    );
    assert!(output.content.contains(&format!("path={path}")));
}

const ROOT_README: &str = "# Docs\n\n## Purpose\n\nRoot index.\n\n## Table of Contents\n\n- [system/](system/README.md): system contracts.\n";

const SYSTEM_README: &str = "# System\n\n## Purpose\n\nSystem index.\n\n## Table of Contents\n\n- [overview.md](overview.md): system overview.\n";

const OVERVIEW: &str =
    "# System Overview\n\n## Purpose\n\nLeaf contract.\n\n## Status\n\nimplemented.\n";
