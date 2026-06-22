mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn japanese_cookbook_profile_uses_japanese_paths() -> TestResult<()> {
    let workspace = temp_workspace("japanese-cookbook-profile")?;
    let output = run(
        &workspace,
        "artifact.apply",
        &[
            ("root", "cookbooks/japanese-foods"),
            ("title", "Japanese Food Cookbook"),
            ("kind", "cookbook"),
        ],
    )?;

    assert!(output.contains("profile=Cookbook"));
    assert!(workspace
        .join("cookbooks/japanese-foods/foundations/japanese-pantry.md")
        .is_file());
    assert!(workspace
        .join("cookbooks/japanese-foods/mains/ramen-noodles.md")
        .is_file());
    assert_absent(&workspace, "cookbooks/japanese-foods/recipes/ciabatta.md");
    assert_absent(
        &workspace,
        "cookbooks/japanese-foods/foundations/flour-water-salt-yeast.md",
    );
    Ok(())
}

#[test]
fn japanese_cookbook_drift_blocks_next_and_apply() -> TestResult<()> {
    let workspace = temp_workspace("japanese-cookbook-drift")?;
    let root = "cookbooks/japanese-foods";
    run(
        &workspace,
        "artifact.apply",
        &[
            ("root", root),
            ("title", "Japanese Food Cookbook"),
            ("kind", "cookbook"),
        ],
    )?;
    fs::write(
        workspace.join(root).join("mains/ramen-noodles.md"),
        "# Ramen Noodles\n\n## Purpose\n\nThis bread cookbook section drifts away from Japanese foods.\n",
    )?;

    let next = run(&workspace, "artifact.next", &[("root", root)])?;
    let apply = run(
        &workspace,
        "artifact.apply",
        &[
            ("root", root),
            ("title", "Japanese Food Cookbook"),
            ("kind", "cookbook"),
        ],
    )?;

    assert!(next.contains("artifact drift guard active"));
    assert!(next.contains("blocked=artifact.next,artifact.apply"));
    assert!(apply.contains("artifact drift guard active"));
    Ok(())
}

fn run(workspace: &Path, tool: &str, params: &[(&str, &str)]) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(
        &action(tool, params),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content)
}

fn assert_absent(workspace: &Path, path: &str) {
    assert!(
        !workspace.join(path).exists(),
        "unexpected drift path {path}"
    );
}
