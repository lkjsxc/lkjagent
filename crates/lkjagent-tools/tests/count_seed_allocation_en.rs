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
    assert!(root.join("docs/design-016.md").exists());
    assert!(!root.join("docs/design-017.md").exists());
    assert!(root.join("main/part-081.md").exists());
    assert!(!root.join("main/part-082.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 16"));
    assert!(readme.contains("- Main files: 81"));
    assert!(readme.contains("Kind contract: audit this deliverable as a guide"));
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
    assert!(root.join("docs/design-022.md").exists());
    assert!(!root.join("docs/design-023.md").exists());
    assert!(root.join("main/part-075.md").exists());
    assert!(!root.join("main/part-076.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    let first = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(readme.contains("- Design memos: 22"));
    assert!(readme.contains("- Main files: 75"));
    assert!(readme.contains("Kind contract: audit this deliverable as a narrative"));
    assert!(first.contains("### Scene Role"));
    Ok(())
}
