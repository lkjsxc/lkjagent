mod support;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
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
    assert!(output.contains("path: stories/chronos-fracture/catalog.toml"));
    assert!(output.contains("path: stories/chronos-fracture/README.md"));
    assert!(output.contains("path: stories/chronos-fracture/request/objective.md"));
    let example = candidate_example(&output)?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    dispatch_state.reset_repeat_tracking();
    let write = dispatch(&parsed, &runtime, &mut conn, &mut dispatch_state).content;

    assert!(write.contains("files_written=3"));
    assert!(workspace.join(root).join("catalog.toml").is_file());
    assert!(workspace.join(root).join("README.md").is_file());
    assert!(workspace.join(root).join("request/objective.md").is_file());
    Ok(())
}

fn candidate_example(output: &str) -> TestResult<&str> {
    output
        .split_once("candidate_example:\n")
        .map(|(_, example)| example)
        .ok_or_else(|| "missing candidate example".into())
}
