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
    assert!(root.join(support::design_path(28)).exists());
    assert!(!root.join(support::design_path(29)).exists());
    assert!(root.join(support::main_path(69)).exists());
    assert!(!root.join(support::main_path(70)).exists());
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
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
    assert!(root.join(support::design_path(28)).exists());
    assert!(!root.join(support::design_path(29)).exists());
    assert!(root.join(support::main_path(69)).exists());
    assert!(!root.join(support::main_path(70)).exists());
    Ok(())
}

#[test]
fn count_seed_honors_split_into_source_packets() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-split-into-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Split into \
         twenty-eight source packets. The rest as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 28, 69)?;
    assert!(root.join(support::design_path(28)).exists());
    assert!(!root.join(support::design_path(29)).exists());
    assert!(root.join(support::main_path(69)).exists());
    assert!(!root.join(support::main_path(70)).exists());
    Ok(())
}

#[test]
fn count_seed_honors_use_source_packets() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-use-source-packets")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Use \
         twenty-eight source packets. The rest as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 28, 69)?;
    assert!(root.join(support::design_path(28)).exists());
    assert!(!root.join(support::design_path(29)).exists());
    assert!(root.join(support::main_path(69)).exists());
    assert!(!root.join(support::main_path(70)).exists());
    Ok(())
}

#[test]
fn count_seed_honors_research_notes_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-research-notes-without-file-noun")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Use \
         twenty-four research notes. The rest as ordered report sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73)?;
    assert!(root.join(support::design_path(24)).exists());
    assert!(!root.join(support::design_path(25)).exists());
    assert!(root.join(support::main_path(73)).exists());
    assert!(!root.join(support::main_path(74)).exists());
    Ok(())
}

#[test]
fn count_seed_honors_evidence_summaries_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-evidence-summaries-without-file-noun")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a market intelligence dossier. Use \
         twenty-four evidence summaries. The rest as ordered report sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73)?;
    assert!(root.join(support::design_path(24)).exists());
    assert!(!root.join(support::design_path(25)).exists());
    assert!(root.join(support::main_path(73)).exists());
    assert!(!root.join(support::main_path(74)).exists());
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
    assert!(root.join(support::design_path(12)).exists());
    assert!(!root.join(support::design_path(13)).exists());
    assert!(root.join(support::main_path(85)).exists());
    Ok(())
}

#[test]
fn count_seed_does_not_treat_same_clause_total_as_split_allocation() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-same-clause-total-rest")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total and the rest as ordered report sections for a \
         market intelligence dossier. Count docs and main content together. Keep Codex/Spark \
         budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 12, 85)?;
    assert!(root.join(support::design_path(12)).exists());
    assert!(!root.join(support::design_path(13)).exists());
    assert!(root.join(support::main_path(85)).exists());
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
