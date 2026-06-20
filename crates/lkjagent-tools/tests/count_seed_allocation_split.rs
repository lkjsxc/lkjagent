mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_unknown_support_files_in_remaining_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-remaining-split-reference")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market research dossier, with twenty-three \
         reference files and the remaining files as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(23)).exists());
    assert!(!root.join(support::design_path(24)).exists());
    assert!(root.join(support::main_path(74)).exists());
    assert!(!root.join(support::main_path(75)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 23"));
    assert!(readme.contains("- Main files: 74"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_honors_unknown_support_files_in_all_other_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-all-other-split-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market intelligence dossier, with twenty-four \
         source packet files and all other files as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(24)).exists());
    assert!(!root.join(support::design_path(25)).exists());
    assert!(root.join(support::main_path(73)).exists());
    assert!(!root.join(support::main_path(74)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 24"));
    assert!(readme.contains("- Main files: 73"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_honors_unknown_support_files_before_comma_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-comma-split-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market intelligence dossier, with twenty-six \
         source packet files, and the remaining files as ordered report sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(26)).exists());
    assert!(!root.join(support::design_path(27)).exists());
    assert!(root.join(support::main_path(71)).exists());
    assert!(!root.join(support::main_path(72)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 26"));
    assert!(readme.contains("- Main files: 71"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_honors_unknown_support_files_in_plus_rest_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-plus-rest-split-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market intelligence dossier, with twenty-five \
         source packet files plus the rest as ordered report sections. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(25)).exists());
    assert!(!root.join(support::design_path(26)).exists());
    assert!(root.join(support::main_path(72)).exists());
    assert!(!root.join(support::main_path(73)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 25"));
    assert!(readme.contains("- Main files: 72"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_honors_unknown_support_files_before_semicolon_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-semicolon-split-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market intelligence dossier; include \
         twenty-seven source packet files; the rest as ordered report sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(27)).exists());
    assert!(!root.join(support::design_path(28)).exists());
    assert!(root.join(support::main_path(70)).exists());
    assert!(!root.join(support::main_path(71)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 27"));
    assert!(readme.contains("- Main files: 70"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_does_not_reuse_total_count_before_rest_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-total-before-rest-split")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a market intelligence dossier; the rest as \
         ordered report sections. Count docs and main content together. Keep Codex/Spark budget \
         low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(12)).exists());
    assert!(!root.join(support::design_path(13)).exists());
    assert!(root.join(support::main_path(85)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 12"));
    assert!(readme.contains("- Main files: 85"));
    Ok(())
}
