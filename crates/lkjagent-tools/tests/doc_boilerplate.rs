mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_scaffold_writer_emits_no_boilerplate_files() -> TestResult<()> {
    let workspace = temp_workspace("doc-boilerplate")?;
    let output = run_tool(
        &workspace,
        "doc.scaffold",
        &[("root", "docs"), ("title", "Project Docs")],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs").exists());
    Ok(())
}

#[test]
fn old_boilerplate_fails_content_readiness() -> TestResult<()> {
    let workspace = temp_workspace("doc-boilerplate-old")?;
    let root = workspace.join("docs");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"documentation\"\n")?;
    fs::write(
        root.join("README.md"),
        "# Docs\n\n## Purpose\n\nDocs.\n\n## Table of Contents\n\n- [a.md](a.md): A.\n- [b.md](b.md): B.\n- [catalog.toml](catalog.toml): Catalog.\n",
    )?;
    fs::write(root.join("a.md"), old_body("A"))?;
    fs::write(root.join("b.md"), old_body("B"))?;

    let audit = audit(&workspace, "docs")?;

    assert!(audit.contains("document audit failed"));
    assert!(audit.contains("content_readiness=failed"));
    assert!(audit.contains("scaffold_only_content"));
    Ok(())
}

fn old_body(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis file records the {title} role for the generated documentation tree.\n\n## Contract\n\n- Keep this file semantic and linked from its local README.\n- Record concrete facts, decisions, and verification evidence.\n\n## Status\n\nscaffolded\n"
    )
}

fn audit(workspace: &Path, root: &str) -> TestResult<String> {
    run_tool(workspace, "doc.audit", &[("root", root)])
}

fn run_tool(workspace: &Path, tool: &str, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action(tool, params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}
