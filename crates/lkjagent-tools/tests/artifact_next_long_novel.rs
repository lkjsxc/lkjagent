mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn long_novel_profile_uses_story_repair_sections() -> TestResult<()> {
    let workspace = temp_workspace("artifact-next-long-novel")?;
    let root = "stories/novel";
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    fs::create_dir_all(runtime.workspace.join(root).join("setting"))?;
    fs::write(
        runtime.workspace.join(root).join("catalog.toml"),
        "kind = \"story\"\n",
    )?;
    fs::write(runtime.workspace.join(root).join("README.md"), "# Novel\n")?;
    fs::write(
        runtime.workspace.join(root).join("setting/world.md"),
        "# World\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;

    let output = dispatch(
        &action("artifact.next", &[("root", root), ("kind", "novel")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    let contract = output
        .split_once("candidate_contract:\n")
        .map(|(_, contract)| contract)
        .ok_or("missing candidate contract")?;

    assert!(output.contains("kind=story"));
    assert!(output.contains("scene content or reference detail"));
    assert!(output.contains("continuity notes"));
    assert!(output.contains("next_decision_required=true"));
    assert!(contract.contains("tool=fs.batch_write"));
    Ok(())
}
