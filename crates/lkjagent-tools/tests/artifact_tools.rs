mod support;

use std::fs;

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
fn artifact_apply_writes_manifest_and_readmes() -> TestResult<()> {
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

    assert!(output.content.contains("document scaffold created"));
    assert!(workspace.join("stories/memory-market/README.md").is_file());
    assert!(workspace
        .join("stories/memory-market/catalog.toml")
        .is_file());
    Ok(())
}

#[test]
fn artifact_apply_reuses_existing_root_without_duplicate_sections() -> TestResult<()> {
    let workspace = temp_workspace("artifact-apply-reuse")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let params = [
        ("root", "stories/memory-market"),
        ("title", "Memory Market"),
        ("kind", "story"),
    ];

    let first = dispatch(
        &action("artifact.apply", &params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let initial_count = markdown_count(&workspace.join("stories/memory-market"))?;
    dispatch_state.reset_repeat_tracking();
    let second = dispatch(
        &action("artifact.apply", &params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let repaired_count = markdown_count(&workspace.join("stories/memory-market"))?;

    assert!(first.content.contains("document scaffold created"));
    assert!(second.content.contains("document scaffold created"));
    assert_eq!(initial_count, repaired_count);
    assert!(!workspace
        .join("stories/memory-market/chapters/part-001.md")
        .exists());
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
fn artifact_audit_rejects_cookbook_scaffold_without_recipe_content() -> TestResult<()> {
    let workspace = temp_workspace("artifact-audit-cookbook-content")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", "cookbooks/bread-cookbook"),
                ("title", "Bread Cookbook"),
                ("kind", "cookbook"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "cookbooks/bread-cookbook"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("document audit failed"));
    assert!(output.content.contains("content_readiness=failed"));
    assert!(output.content.contains("structure_only_content: recipes/"));
    Ok(())
}

#[test]
fn artifact_audit_rejects_generic_project_docs_for_story() -> TestResult<()> {
    let workspace = temp_workspace("artifact-audit-kind")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let apply = dispatch(
        &action(
            "doc.scaffold",
            &[
                ("root", "stories/not-a-story"),
                ("title", "Project Documentation"),
                ("kind", "documentation"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    assert!(apply.content.contains("document scaffold created"));
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/not-a-story"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("artifact_kind_mismatch"));
    let catalog = fs::read_to_string(workspace.join("stories/not-a-story/catalog.toml"))?;
    assert!(catalog.contains("ProjectDocs"));
    Ok(())
}

fn markdown_count(root: &std::path::Path) -> TestResult<usize> {
    let mut count: usize = 0;
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            count = count.saturating_add(markdown_count(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
