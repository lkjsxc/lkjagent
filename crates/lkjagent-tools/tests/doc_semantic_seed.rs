mod support;

use lkjagent_tools::dispatch::dispatch;
use std::path::Path;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_scaffold_creates_no_lkjagent_seed() -> TestResult<()> {
    let workspace = temp_workspace("doc-lkjagent-seed")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "docs"),
            ("title", "lkjagent qwen rust documentation"),
            ("kind", "documentation-init"),
        ],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs/project/purpose.md").exists());
    assert!(!workspace.join("docs/lkjagent.md").exists());
    Ok(())
}

#[test]
fn removed_scaffold_creates_no_multi_topic_seed() -> TestResult<()> {
    let workspace = temp_workspace("doc-multi-topic-seed")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "docs"),
            (
                "title",
                "lkjagent model endpoint Asia foods Minecraft Factorio documentation",
            ),
            ("kind", "documentation-init"),
        ],
    )?;

    assert!(output.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("docs/project/lkjagent.md").exists());
    assert!(!workspace.join("docs/domain-examples/minecraft.md").exists());
    Ok(())
}

#[test]
fn audit_rejects_markdown_suffix_directory() -> TestResult<()> {
    let workspace = temp_workspace("doc-md-dir")?;
    std::fs::create_dir_all(workspace.join("docs/lkjagent.md"))?;
    std::fs::write(
        workspace.join("docs/README.md"),
        "# Docs\n\n## Purpose\n\nDocs.\n\n## Table of Contents\n\n- [lkjagent.md/](lkjagent.md/README.md): bad.\n",
    )?;
    std::fs::write(
        workspace.join("docs/lkjagent.md/README.md"),
        "# Bad\n\n## Purpose\n\nBad.\n\n## Table of Contents\n\n- [topic.md](topic.md): topic.\n- [other.md](other.md): other.\n",
    )?;
    std::fs::write(
        workspace.join("docs/lkjagent.md/topic.md"),
        "# Topic\n\n## Purpose\n\nTopic.\n",
    )?;
    std::fs::write(
        workspace.join("docs/lkjagent.md/other.md"),
        "# Other\n\n## Purpose\n\nOther.\n",
    )?;

    let audit = audit(&workspace, "docs")?;
    assert!(audit.contains("markdown_suffix_directory: lkjagent.md"));
    assert!(audit.contains("document audit failed"));
    Ok(())
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
    let mut state = state();
    Ok(dispatch(&action(tool, params), &runtime, &mut conn, &mut state).content)
}
