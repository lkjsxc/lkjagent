mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn story_audit_accepts_handwritten_story_catalog_metadata() -> TestResult<()> {
    let workspace = temp_workspace("artifact-kind-story-catalog")?;
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("catalog.toml"),
        "title = \"Chronos Fracture\"\ndescription = \"Story bible.\"\n",
    )?;
    fs::write(root.join("README.md"), "# Chronos Fracture\n")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/chronos-fracture"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(!output.content.contains("artifact_kind_mismatch"));
    Ok(())
}

#[test]
fn artifact_next_infers_story_from_handwritten_catalog() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-story-catalog")?;
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("catalog.toml"),
        "description = \"Story bible.\"\n",
    )?;
    fs::write(root.join("README.md"), "# Chronos Fracture\n")?;
    fs::write(
        root.join("premise.md"),
        "# Premise\n\nChronos fractures causality.\n",
    )?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;

    let output = dispatch(
        &action("artifact.next", &[("root", "stories/chronos-fracture")]),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("kind=story"));
    Ok(())
}

#[test]
fn removed_doc_scaffold_does_not_create_story_profile() -> TestResult<()> {
    let workspace = temp_workspace("doc-scaffold-stories-root")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;

    let output = dispatch(
        &action(
            "doc.scaffold",
            &[
                ("root", "stories/chronos-fracture"),
                ("kind", "documentation"),
                ("title", "Chronos Fracture Story Bible"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("unknown tool: doc.scaffold"));
    assert!(!workspace.join("stories/chronos-fracture").exists());
    Ok(())
}

#[test]
fn stories_root_treats_generic_kind_as_story_contract() -> TestResult<()> {
    let workspace = temp_workspace("artifact-kind-stories-root")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;

    let next = dispatch(
        &action("artifact.next", &[("root", "stories/chronos-fracture")]),
        &runtime,
        &mut conn,
        &mut state(),
    );
    assert!(next.content.contains("kind=story"));

    assert!(next.content.contains("candidate_action=fs.batch_write"));
    assert!(next.content.contains("candidate_contract:"));
    assert!(!workspace.join("stories/chronos-fracture").exists());
    let apply = dispatch(
        &action(
            "artifact.apply",
            &[("root", "stories/chronos-fracture"), ("kind", "artifact")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );
    assert!(apply.content.contains("unknown tool: artifact.apply"));
    Ok(())
}
