mod support;

use std::fs;

use lkjagent_runtime::daemon::build_prefix_from_store;
use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, dispatch_state, store, temp_workspace, tool_runtime, TestResult};

#[test]
fn recursive_structure_seed_loads_and_drives_indexed_tree() -> TestResult<()> {
    let workspace = temp_workspace("recursive-structure")?;
    fs::write(
        workspace.join("AGENTS.md"),
        "# AGENTS.md\n\n## Purpose\n\nTest workspace rules.\n",
    )?;
    let runtime = tool_runtime(workspace.clone())?;

    let mut conn = store()?;
    let prefix = build_prefix_from_store(&conn, &runtime.skill_library, &workspace)?;
    assert!(prefix
        .iter()
        .any(|frame| frame.content.contains("recursive-structure: A task asks")));

    let mut state = dispatch_state();
    let skill = dispatch(
        &action("skill.use", &[("name", "recursive-structure")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(skill.kind, OutputKind::Skill { .. }));
    assert!(skill
        .content
        .contains("Put a `README.md` in every new directory"));

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
            "shell.run",
            &[(
                "command",
                "find docs -type d ! -exec test -f '{}/README.md' ';' -print",
            )],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(check.content.trim().ends_with("exit_code=0"));
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
