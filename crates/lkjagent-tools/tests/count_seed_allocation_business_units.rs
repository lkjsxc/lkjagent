mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_risk_registers_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-risk-registers")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "Create about one hundred files total for a compliance operations dossier. Use \
         twenty-four risk registers. The rest as ordered control sections. Count docs and main \
         content together. Keep Codex/Spark budget low.",
    )?;

    let root = workspace.join("structured-output");
    assert_counts(&root, 24, 73, "report")?;
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    Ok(())
}

fn file_guard() -> CountGuard {
    CountGuard {
        kind: CountKind::File,
        target: 100,
        mode: CountMode::Approximate,
    }
}

fn assert_counts(root: &Path, design: usize, main: usize, kind: &str) -> TestResult<()> {
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains(&format!("- Design memos: {design}")));
    assert!(readme.contains(&format!("- Main files: {main}")));
    assert!(readme.contains(&format!(
        "Kind contract: audit this deliverable as a {kind}"
    )));
    Ok(())
}
