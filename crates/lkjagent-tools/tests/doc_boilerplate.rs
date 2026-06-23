mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn scaffold_profiles_do_not_emit_old_boilerplate() -> TestResult<()> {
    let workspace = temp_workspace("doc-boilerplate")?;
    for (index, params) in scaffold_cases().iter().enumerate() {
        let root = format!("docs-{index}");
        let mut case = vec![("root", root.as_str())];
        case.extend(params.iter().copied());
        scaffold(&workspace, &case)?;
        assert_no_banned(&workspace.join(root))?;
    }
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

fn scaffold_cases() -> Vec<Vec<(&'static str, &'static str)>> {
    vec![
        vec![("title", "Project Docs")],
        vec![("title", "Knowledge Encyclopedia")],
        vec![("title", "Implementation Plan")],
        vec![("title", "Research Report")],
        vec![("title", "User Guide")],
        vec![("title", "Operations Runbook")],
        vec![("title", "Long SF Story")],
        vec![("title", "Japanese Cookbook"), ("kind", "cookbook")],
        vec![("title", "Bread Cookbook"), ("kind", "content-artifact")],
        vec![(
            "title",
            "Model Endpoint, Minecraft, Windows, Japan, United States",
        )],
    ]
}

fn assert_no_banned(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            assert_no_banned(&path)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            let text = fs::read_to_string(&path)?;
            for phrase in banned_phrases() {
                assert!(
                    !text.contains(phrase),
                    "{} contains {phrase}",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn banned_phrases() -> [&'static str; 6] {
    [
        "Keep this file semantic",
        "Record concrete facts",
        "Implementation Hooks",
        "Failure Modes",
        "scaffolded",
        "generated documentation tree",
    ]
}

fn old_body(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis file records the {title} role for the generated documentation tree.\n\n## Contract\n\n- Keep this file semantic and linked from its local README.\n- Record concrete facts, decisions, and verification evidence.\n\n## Implementation Hooks\n\n- Source: `crates/lkjagent-tools/src/doc.rs`\n\n## Failure Modes\n\n- The file is unlinked from its directory README.\n\n## Status\n\nscaffolded\n"
    )
}

fn scaffold(workspace: &Path, params: &[(&str, &str)]) -> TestResult<String> {
    run_tool(workspace, "doc.scaffold", params)
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
