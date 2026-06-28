mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_apply_writes_card_with_full_owner_objective() -> TestResult<()> {
    let workspace = temp_workspace("artifact-card-long-novel")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let objective = "Create a long novel. with structured settings.";

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

    let card = fs::read_to_string(workspace.join("stories/novel/artifact-card.txt"))?;
    assert!(output.content.contains("document scaffold created"));
    assert!(card.contains("<root>stories/novel</root>"));
    assert!(card.contains("<label>Create a long novel. with structured settings.</label>"));
    assert!(card.contains("<semantic-id>story:novel</semantic-id>"));
    Ok(())
}
