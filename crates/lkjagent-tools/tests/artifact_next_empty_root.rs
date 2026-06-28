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
    assert!(output.contains("- stories/chronos-fracture/catalog.toml"));
    assert!(output.contains("- stories/chronos-fracture/README.md"));
    assert!(output.contains("- stories/chronos-fracture/request/objective.md"));
    assert!(output.contains("candidate_contract:"));
    dispatch_state.reset_repeat_tracking();
    let files = "path: stories/chronos-fracture/catalog.toml\ncontent:\nkind = \"story\"\n\n-- lkjagent-next-file --\npath: stories/chronos-fracture/README.md\ncontent:\n# Chronos Fracture\n\n## Purpose\n\nNavigate the story artifact.\n\n-- lkjagent-next-file --\npath: stories/chronos-fracture/request/objective.md\ncontent:\n# Objective\n\n## Purpose\n\nCreate the requested SF novel settings artifact.\n";
    let write = dispatch(
        &action("fs.batch_write", &[("files", files)]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(write.contains("files_written=3"));
    assert!(workspace.join(root).join("catalog.toml").is_file());
    assert!(workspace.join(root).join("README.md").is_file());
    assert!(workspace.join(root).join("request/objective.md").is_file());
    Ok(())
}
