mod support;

use lkjagent_tools::dispatch::dispatch;
use std::path::Path;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn lkjagent_model_rust_request_creates_connected_seed() -> TestResult<()> {
    let workspace = temp_workspace("doc-lkjagent-seed")?;
    let output = scaffold(
        &workspace,
        &[
            ("root", "docs"),
            ("title", "lkjagent qwen rust documentation"),
            ("kind", "documentation-init"),
        ],
    )?;

    assert!(output.contains("profile=LkjagentSemanticSeed"));
    assert!(workspace.join("docs/project/purpose.md").is_file());
    assert!(workspace.join("docs/model-interface/contract.md").is_file());
    assert!(workspace.join("docs/implementation/rust.md").is_file());
    assert!(workspace
        .join("docs/relations/project-model-implementation.md")
        .is_file());
    assert!(!workspace.join("docs/lkjagent.md").exists());
    let catalog = std::fs::read_to_string(workspace.join("docs/catalog.toml"))?;
    assert!(!catalog.contains("qwen"));

    let audit = audit(&workspace, "docs")?;
    assert!(audit.contains("document audit passed"), "{audit}");
    Ok(())
}

#[test]
fn multi_topic_request_preserves_domain_seed() -> TestResult<()> {
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

    assert!(output.contains("profile=LkjagentSemanticSeed"));
    assert!(workspace.join("docs/project/lkjagent.md").is_file());
    assert!(workspace
        .join("docs/model-interface/model-endpoint.md")
        .is_file());
    assert!(workspace
        .join("docs/domain-examples/asia-foods.md")
        .is_file());
    assert!(workspace
        .join("docs/domain-examples/minecraft.md")
        .is_file());
    assert!(workspace.join("docs/domain-examples/factorio.md").is_file());
    assert!(workspace
        .join("docs/relations/project-model-domain-examples.md")
        .is_file());
    assert!(!workspace.join("docs/architecture/README.md").exists());
    assert!(!workspace.join("docs/guides/README.md").exists());

    let audit = audit(&workspace, "docs")?;
    assert!(audit.contains("document audit passed"), "{audit}");
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
