mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_english_appendix_note_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-appendix-note-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "Create one hundred files altogether for a field manual pack, with exactly sixteen \
         appendix note files and the remainder as ordered main manual sections. This is not \
         fiction. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(16)).exists());
    assert!(!root.join(support::design_path(17)).exists());
    assert!(root.join(support::main_path(81)).exists());
    assert!(!root.join(support::main_path(82)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 16"));
    assert!(readme.contains("- Main files: 81"));
    assert!(readme.contains("Kind contract: audit this deliverable as a guide"));
    Ok(())
}

#[test]
fn count_seed_honors_english_checklist_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-checklist-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a compliance operations handbook, with eighteen \
         checklist files and the remaining files as ordered procedure sections. Count docs and \
         main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(18)).exists());
    assert!(!root.join(support::design_path(19)).exists());
    assert!(root.join(support::main_path(79)).exists());
    assert!(!root.join(support::main_path(80)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 18"));
    assert!(readme.contains("- Main files: 79"));
    assert!(readme.contains("Kind contract: audit this deliverable as a guide"));
    assert!(first.contains("### Procedure Role"));
    Ok(())
}

#[test]
fn count_seed_honors_english_timeline_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-timeline-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a disaster response playbook, with seventeen \
         timeline files and the remaining files as ordered exercise modules. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(17)).exists());
    assert!(!root.join(support::design_path(18)).exists());
    assert!(root.join(support::main_path(80)).exists());
    assert!(!root.join(support::main_path(81)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 17"));
    assert!(readme.contains("- Main files: 80"));
    assert!(readme.contains("Kind contract: audit this deliverable as a guide"));
    assert!(first.contains("### Procedure Role"));
    Ok(())
}

#[test]
fn count_seed_honors_english_worldbuilding_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-worldbuilding-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create around one hundred files total for a large fantasy novel, with twenty-two \
         worldbuilding files and the remaining files as ordered chapter drafts. Include docs \
         and main content in that total. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(22)).exists());
    assert!(!root.join(support::design_path(23)).exists());
    assert!(root.join(support::main_path(75)).exists());
    assert!(!root.join(support::main_path(76)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 22"));
    assert!(readme.contains("- Main files: 75"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    Ok(())
}

#[test]
fn count_seed_honors_english_lore_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-lore-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a large mythic novel, with twenty-one lore \
         files and the remaining files as ordered chapter drafts. Count docs and main content \
         together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(21)).exists());
    assert!(!root.join(support::design_path(22)).exists());
    assert!(root.join(support::main_path(76)).exists());
    assert!(!root.join(support::main_path(77)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 21"));
    assert!(readme.contains("- Main files: 76"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    Ok(())
}

#[test]
fn count_seed_honors_english_character_sheet_file_count() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-en-character-sheet-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about one hundred files total for a large science fantasy novel, with nineteen \
         character sheet files and the remaining files as ordered chapter drafts. Count docs \
         and main content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join(support::design_path(19)).exists());
    assert!(!root.join(support::design_path(20)).exists());
    assert!(root.join(support::main_path(78)).exists());
    assert!(!root.join(support::main_path(79)).exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join(support::main_path(1)))?;
    assert!(readme.contains("- Design memos: 19"));
    assert!(readme.contains("- Main files: 78"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    Ok(())
}
