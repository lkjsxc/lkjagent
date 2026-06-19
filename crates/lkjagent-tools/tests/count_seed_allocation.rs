mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_design_count_hint_inside_total() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-design-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "合計１００ファイル程度の大きな物語を作ってください。20章ぶんの設計観点も含めます。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-020.md").exists());
    assert!(!root.join("docs/design-021.md").exists());
    assert!(root.join("main/part-077.md").exists());
    assert!(!root.join("main/part-078.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 20"));
    assert!(readme.contains("- 本編ファイル: 77"));
    Ok(())
}
