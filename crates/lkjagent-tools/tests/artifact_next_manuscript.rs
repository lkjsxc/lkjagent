mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn manuscript_next_prefers_chapter_after_identity() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-manuscript")?;
    let root = "stories/bell-rings-twice";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    seed_identity(
        &runtime.workspace,
        root,
        "Create a 10,000 word novel in ten chapters.",
    )?;
    lkjagent_store::state::set(
        &conn,
        &format!("artifact requested scale {root}"),
        "full-draft",
    )?;

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "story")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(
        output.contains("next_paths:\n- manuscript/scenes/chapter-01/scene-01.md"),
        "{output}"
    );
    assert!(output.contains("finished scene prose"), "{output}");
    assert!(!output.contains("act-structure.md"), "{output}");
    Ok(())
}

#[test]
fn story_bible_request_keeps_story_bible_paths() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-story-bible")?;
    let root = "stories/reference-bible";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    seed_identity(
        &runtime.workspace,
        root,
        "Create a story bible with setting, cast, plot, and continuity notes.",
    )?;

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "story")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("act-structure.md"), "{output}");
    assert!(!output.contains("manuscript/chapter-01.md"), "{output}");
    Ok(())
}

fn seed_identity(workspace: &std::path::Path, root: &str, objective: &str) -> TestResult<()> {
    let root = workspace.join(root);
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(root.join("README.md"), "# Story\n")?;
    fs::write(
        root.join("objective.md"),
        format!("# Objective\n\n## Purpose\n\n{objective}\n"),
    )?;
    Ok(())
}
