mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn english_parenthetical_composition_keeps_manual_as_main_anchor() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-parenthetical-manual-anchor")?;
    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create a one hundred file operations readiness manual (documentation files and main procedure files combined), with twenty planning notes and the remaining files as ordered procedures. Save Codex/Spark budget.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("documentation files and main procedure files combined"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(first_part.contains(
        "Local objective: Turn \"operations readiness manual\" into this file's distinct contribution."
    ));
    assert_no_local_objective_starts_with(&root, "with twenty planning notes")?;
    Ok(())
}

fn assert_no_local_objective_starts_with(root: &std::path::Path, prefix: &str) -> TestResult<()> {
    for entry in fs::read_dir(root.join("main"))? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str()) == Some("md") {
            let text = fs::read_to_string(path)?;
            assert!(
                !text.contains(&format!("Local objective: Turn \"{prefix}")),
                "unexpected local objective prefix {prefix}"
            );
        }
    }
    Ok(())
}
