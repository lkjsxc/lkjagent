mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_honors_japanese_design_file_count_phrase() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-design-file-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "ドキュメント用のファイルたちと本編のファイルたちを合計して百ファイルくらいの\
         大きな物語を一発で作ってください。設計メモは十六ファイル、残りは章ごとの\
         本編ファイルにしてください。Codex/Spark の消費は抑えてください。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-016.md").exists());
    assert!(!root.join("docs/design-017.md").exists());
    assert!(root.join("main/part-081.md").exists());
    assert!(!root.join("main/part-082.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 16"));
    assert!(readme.contains("- 本編ファイル: 81"));
    Ok(())
}
