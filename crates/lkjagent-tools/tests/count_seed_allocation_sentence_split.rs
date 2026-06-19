mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_sentence_split_allocation() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-sentence-split-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Include \
         twenty-eight source packet files. The rest as ordered report sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 28, 69)?;
    assert!(root.join("docs/design-028.md").exists());
    assert!(!root.join("docs/design-029.md").exists());
    assert!(root.join("main/part-069.md").exists());
    assert!(!root.join("main/part-070.md").exists());
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(first.contains("### Analysis Role"));
    Ok(())
}

#[test]
fn count_seed_honors_source_packets_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-source-packets-without-file-noun")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Include \
         twenty-eight source packets. The rest as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 28, 69)?;
    assert!(root.join("docs/design-028.md").exists());
    assert!(!root.join("docs/design-029.md").exists());
    assert!(root.join("main/part-069.md").exists());
    assert!(!root.join("main/part-070.md").exists());
    Ok(())
}

#[test]
fn count_seed_does_not_treat_total_sentence_as_split_allocation() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-sentence-total-rest")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Include about one hundred files total. The rest as ordered report sections for a market \
         intelligence dossier. Count docs and main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 12, 85)?;
    assert!(root.join("docs/design-012.md").exists());
    assert!(!root.join("docs/design-013.md").exists());
    assert!(root.join("main/part-085.md").exists());
    Ok(())
}

fn file_guard() -> CountGuard {
    CountGuard {
        kind: CountKind::File,
        target: 100,
        mode: CountMode::Approximate,
    }
}

fn assert_counts(root: &Path, design: usize, main: usize) -> TestResult<()> {
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains(&format!("- Design memos: {design}")));
    assert!(readme.contains(&format!("- Main files: {main}")));
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    Ok(())
}
