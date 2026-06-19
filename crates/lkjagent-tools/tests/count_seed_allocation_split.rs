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
    assert!(root.join("docs/design-023.md").exists());
    assert!(!root.join("docs/design-024.md").exists());
    assert!(root.join("main/part-074.md").exists());
    assert!(!root.join("main/part-075.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
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
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
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
    assert!(root.join("docs/design-026.md").exists());
    assert!(!root.join("docs/design-027.md").exists());
    assert!(root.join("main/part-071.md").exists());
    assert!(!root.join("main/part-072.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(readme.contains("- Design memos: 26"));
    assert!(readme.contains("- Main files: 71"));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    assert!(first.contains("### Analysis Role"));
    Ok(())
}
