mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_artifact_apply_does_not_create_bread_paths() -> TestResult<()> {
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

    assert!(output.contains("unknown tool: artifact.apply"));
    assert_absent(&workspace, "cookbooks/japanese-foods/recipes/ciabatta.md");
    assert_absent(
        &workspace,
        "cookbooks/japanese-foods/foundations/flour-water-salt-yeast.md",
    );
    Ok(())
}

#[test]
fn japanese_cookbook_drift_blocks_next() -> TestResult<()> {
    let workspace = temp_workspace("japanese-cookbook-drift")?;
    let root = "cookbooks/japanese-foods";
    seed_japanese_cookbook(&workspace, root)?;
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
    assert!(next.contains("blocked=artifact.next"));
    assert!(apply.contains("unknown tool: artifact.apply"));
    Ok(())
}

fn seed_japanese_cookbook(workspace: &Path, root: &str) -> TestResult<()> {
    let root = workspace.join(root);
    fs::create_dir_all(root.join("mains"))?;
    fs::write(
        root.join("catalog.toml"),
        "kind = \"cookbook\"\nsubject = \"Japanese food\"\n",
    )?;
    fs::write(root.join("README.md"), "# Japanese Foods\n")?;
    fs::write(root.join("mains/ramen-noodles.md"), "# Ramen\n")?;
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
