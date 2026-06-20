mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn native_read_patch_tree_and_index_reduce_shell_need() -> TestResult<()> {
    let workspace = temp_workspace("typed-native")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    fs::create_dir_all(workspace.join("docs/topic"))?;
    fs::write(workspace.join("Cargo.toml"), "[workspace]\nmembers=[]\n")?;
    fs::write(workspace.join("README.md"), "# Root\n")?;
    fs::write(workspace.join("docs/README.md"), "# Docs\n")?;
    fs::write(workspace.join("docs/topic/a.md"), "alpha\nbeta\n")?;
    fs::write(workspace.join("docs/topic/b.md"), "gamma\ndelta\n")?;

    let many = dispatch(
        &action(
            "fs.read_many",
            &[
                ("paths", "docs/topic/a.md\ndocs/topic/b.md"),
                ("count", "1"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(many.content.contains("path=docs/topic/a.md"));
    assert!(many.content.contains("path=docs/topic/b.md"));

    let patch = "find:\nalpha\nreplace:\nALPHA\n";
    let patched = dispatch(
        &action("fs.patch", &[("path", "docs/topic/a.md"), ("patch", patch)]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(patched.content.contains("edits=1"));
    assert!(fs::read_to_string(workspace.join("docs/topic/a.md"))?.contains("ALPHA"));

    let tree = dispatch(
        &action("fs.tree", &[("path", "docs"), ("depth", "2")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(tree.content.contains("dir docs/topic"));
    assert!(tree.content.contains("file docs/topic/a.md"));

    let index = dispatch(
        &action("workspace.index", &[("path", "."), ("depth", "2")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(index.content.contains("manifest=Cargo.toml"));
    assert!(index.content.contains("readme=README.md"));
    Ok(())
}
