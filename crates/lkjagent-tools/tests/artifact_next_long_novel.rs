mod support;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn long_novel_profile_uses_story_repair_sections() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-long-novel")?;
    let root = "stories/long-novel-with-detailed-settings";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    dispatch(
        &action(
            "artifact.apply",
            &[("root", root), ("kind", "novel"), ("title", "Long Novel")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "novel")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    let example = output
        .split_once("candidate_example:\n")
        .map(|(_, example)| example)
        .ok_or("missing candidate example")?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;

    assert!(output.contains("kind=story"));
    assert!(output.contains("scene content or reference detail"));
    assert!(output.contains("continuity notes"));
    assert!(output.contains("next_decision_required=true"));
    assert_eq!(parsed.tool, "fs.batch_write");
    Ok(())
}
