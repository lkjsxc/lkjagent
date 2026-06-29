mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_next_empty_story_root_returns_identity_batch() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-empty-story")?;
    let root = "stories/chronos-fracture";
    std::fs::create_dir_all(workspace.join(root))?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "story")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("artifact_next_result=root_needs_identity"));
    assert!(output.contains("candidate_action=fs.batch_write"));
    for path in [
        "catalog.toml",
        "README.md",
        "objective.md",
        "setting-overview.md",
        "cast.md",
    ] {
        assert!(output.contains(&format!("- {root}/{path}")));
    }
    assert!(!output.contains("request/objective.md"));
    assert!(output.contains("candidate_contract:"));

    dispatch_state.reset_repeat_tracking();
    let write = dispatch(
        &action("fs.batch_write", &[("files", &identity_files(root))]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(write.contains("files_written=5"), "{write}");
    for path in [
        "catalog.toml",
        "README.md",
        "objective.md",
        "setting-overview.md",
        "cast.md",
    ] {
        assert!(workspace.join(root).join(path).is_file());
    }
    Ok(())
}

fn identity_files(root: &str) -> String {
    [
        file(root, "catalog.toml", "kind = \"story\"\n"),
        file(root, "README.md", readme()),
        file(root, "objective.md", leaf("Objective")),
        file(root, "setting-overview.md", leaf("Setting Overview")),
        file(root, "cast.md", leaf("Cast")),
    ]
    .join("\n-- lkjagent-next-file --\n")
}

fn file(root: &str, path: &str, content: impl AsRef<str>) -> String {
    format!("path: {root}/{path}\ncontent:\n{}", content.as_ref())
}

fn readme() -> &'static str {
    "# Chronos Fracture\n\n## Purpose\n\nNavigate the story artifact.\n"
}

fn leaf(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis story bible reference detail records continuity note anchors, verification note checks, setting constraints, cast motives, and narrative context for the requested SF novel settings artifact.\n"
    )
}
