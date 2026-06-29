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
    assert!(output.contains(&format!("- {root}/catalog.toml")));
    for path in [
        "README.md",
        "objective.md",
        "setting-overview.md",
        "cast.md",
    ] {
        assert!(!output.contains(&format!("- {root}/{path}")));
    }
    assert!(!output.contains("request/objective.md"));
    assert!(output.contains("candidate_contract:"));

    dispatch_state.reset_repeat_tracking();
    let write = dispatch(
        &action(
            "fs.batch_write",
            &[("files", &file(root, "catalog.toml", "kind = \"story\"\n"))],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(write.contains("files_written=1"), "{write}");
    assert!(workspace.join(root).join("catalog.toml").is_file());
    Ok(())
}

fn file(root: &str, path: &str, content: impl AsRef<str>) -> String {
    format!("path: {root}/{path}\ncontent:\n{}", content.as_ref())
}
