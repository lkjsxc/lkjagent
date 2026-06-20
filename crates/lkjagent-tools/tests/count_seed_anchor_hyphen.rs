mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_english_main_anchor_trims_digit_hyphen_file_prefix() -> TestResult<()> {
    assert_hyphen_anchor(
        "count-seed-digit-hyphen-anchor",
        "Create a 100-file large story deliverable with documentation and manuscript files combined. Save Codex/Spark budget.",
        "a 100-file large story deliverable",
    )
}

#[test]
fn count_seed_english_main_anchor_trims_word_hyphen_file_prefix() -> TestResult<()> {
    assert_hyphen_anchor(
        "count-seed-word-hyphen-anchor",
        "Create a hundred-file large story deliverable with documentation and manuscript files combined. Save Codex/Spark budget.",
        "a hundred-file large story deliverable",
    )
}

fn assert_hyphen_anchor(name: &str, objective: &str, rejected_prefix: &str) -> TestResult<()> {
    let workspace = temp_workspace(name)?;
    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        objective,
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains(rejected_prefix));
    let first_part = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(first_part.contains(
        "Local objective: Turn \"large story deliverable\" into this file's distinct contribution."
    ));
    assert_no_local_objective_starts_with(&root, rejected_prefix)?;
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
