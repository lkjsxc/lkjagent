mod support;

use lkjagent_tools::dispatch::dispatch;
use std::fs;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn removed_doc_scaffold_is_unknown_for_existing_cataloged_root() -> TestResult<()> {
    let workspace = temp_workspace("doc-scaffold-existing-catalog")?;
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(root.join("README.md"), "# Chronos Fracture\n")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;

    let output = dispatch(
        &action(
            "doc.scaffold",
            &[
                ("root", "stories/chronos-fracture"),
                ("title", "Chronos Fracture"),
                ("kind", "documentation"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("unknown tool: doc.scaffold"));
    Ok(())
}
