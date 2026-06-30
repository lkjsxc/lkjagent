mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn story_readiness_reports_atom_status() -> TestResult<()> {
    let workspace = temp_workspace("story-readiness-atoms")?;
    seed_story_root(&workspace)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/atom-story"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("artifact audit failed"), "{output}");
    assert!(output.contains("artifact_atom_profile=story"), "{output}");
    assert!(output.contains("atom_status=missing"), "{output}");
    assert!(output.contains("atom_missing_count="), "{output}");
    assert!(output.contains("next_atom="), "{output}");
    assert!(
        output.contains("candidate_action=artifact.next"),
        "{output}"
    );
    Ok(())
}

fn seed_story_root(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/atom-story");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(
        root.join("README.md"),
        "# Atom Story\n\n## Purpose\n\nNavigate the story bible.\n\n- [notes](notes.md)\n",
    )?;
    fs::write(
        root.join("notes.md"),
        "# Notes\n\n## Purpose\n\nThis story bible page gives scene content and reference detail with concrete characters chasing a signal through a city while motives, pressure, continuity notes, setting texture, consequences, verification notes, stakes, discoveries, limits, dangers, and emotional turns remain intentionally broad for this audit.\n",
    )?;
    Ok(())
}
