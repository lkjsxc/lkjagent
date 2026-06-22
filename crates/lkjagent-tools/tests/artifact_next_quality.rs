mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn story_artifact_next_example_avoids_placeholder_phrases() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-quality-story")?;
    let root = "stories/run-log-story";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.apply",
            &[
                ("root", root),
                ("title", "Run Log Story"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "story")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("next_action=fs.batch_write"));
    for phrase in [
        "This section contains",
        "The body names facts",
        "This file records",
        "This section describes",
        "concrete artifact content tied to the requested root",
    ] {
        assert!(
            !output.contains(phrase),
            "bad artifact.next phrase: {phrase}"
        );
    }
    Ok(())
}
