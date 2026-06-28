mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_plan_does_not_write_files() -> TestResult<()> {
    let workspace = temp_workspace("artifact-plan-pure")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let output = dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", "stories/memory-market"),
                ("title", "Memory Market"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("document plan created"));
    assert!(output.content.contains("writes=0"));
    assert!(!workspace.join("stories/memory-market").exists());
    Ok(())
}

#[test]
fn removed_artifact_apply_is_not_live() -> TestResult<()> {
    let workspace = temp_workspace("artifact-apply-story")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let output = dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", "stories/memory-market"),
                ("title", "Memory Market"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("unknown tool: artifact.apply"));
    assert!(!workspace.join("stories/memory-market/README.md").exists());
    Ok(())
}

#[test]
fn artifact_audit_rejects_empty_root() -> TestResult<()> {
    let workspace = temp_workspace("artifact-audit-empty")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "cookbooks/bread"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("document audit failed"));
    assert!(output.content.contains("missing_root: cookbooks/bread"));
    Ok(())
}

#[test]
fn artifact_audit_rejects_cookbook_structure_only_content() -> TestResult<()> {
    let workspace = temp_workspace("artifact-audit-cookbook-content")?;
    seed_cookbook(&workspace, "cookbooks/bread-cookbook")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "cookbooks/bread-cookbook"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("audit failed"));
    assert!(output.content.contains("content") || output.content.contains("weak"));
    Ok(())
}

#[test]
fn artifact_audit_rejects_generic_project_docs_for_story() -> TestResult<()> {
    let workspace = temp_workspace("artifact-audit-kind")?;
    let root = workspace.join("docs/not-a-story");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "profile = \"ProjectDocs\"\n")?;
    fs::write(
        root.join("README.md"),
        "# Project Docs\n\n## Purpose\n\nDocs.\n",
    )?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "docs/not-a-story"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("artifact_kind_mismatch"));
    Ok(())
}

fn seed_cookbook(workspace: &Path, root: &str) -> TestResult<()> {
    let root = workspace.join(root);
    fs::create_dir_all(root.join("recipes"))?;
    fs::write(root.join("catalog.toml"), "kind = \"cookbook\"\n")?;
    fs::write(
        root.join("README.md"),
        "# Bread\n\n## Purpose\n\nBread.\n\n## Contents\n\n- [Recipes](recipes/README.md)\n- [Catalog](catalog.toml)\n",
    )?;
    fs::write(
        root.join("recipes/README.md"),
        "# Recipes\n\n## Purpose\n\nRecipes.\n\n## Contents\n\n- [Loaf](loaf.md)\n- [Rolls](rolls.md)\n",
    )?;
    fs::write(
        root.join("recipes/loaf.md"),
        "# Loaf\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;
    fs::write(
        root.join("recipes/rolls.md"),
        "# Rolls\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;
    Ok(())
}
