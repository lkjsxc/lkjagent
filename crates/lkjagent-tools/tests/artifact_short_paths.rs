mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_artifact_apply_does_not_write_sentence_root_card() -> TestResult<()> {
    let workspace = temp_workspace("artifact-card-long-novel")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let objective = "Create a SF novel. with detailed structured settings.";

    let output = dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", "stories/novel"),
                ("kind", "story"),
                ("title", objective),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    assert!(output.content.contains("unknown tool: artifact.apply"));
    assert!(!workspace.join("stories/novel/artifact-card.txt").exists());
    assert!(!workspace
        .join("stories/create-a-sf-novel-with-detailed-structured-settings")
        .exists());
    Ok(())
}
