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
