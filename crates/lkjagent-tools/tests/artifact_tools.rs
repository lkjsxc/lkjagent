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
        .join("stories/memory-market/.lkj-doc-graph.md")
        .is_file());
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
    let manifest = fs::read_to_string(workspace.join("stories/not-a-story/.lkj-doc-graph.md"))?;
    assert!(manifest.contains("ProjectDocs"));
    Ok(())
}
