mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_infers_japanese_ordered_rest_support_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-foreshadow-list")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "合計で百ファイルぐらいの長編小説を作ってください。二十四個の伏線リストを\
         使い、残りは順番付きの本編章にしてください。docs と本編を合計して\
         数えてください。Codex/Spark の使用量は抑えてください。",
    )?;

    assert_counts(&workspace.join("structured-output"), 24, 73)?;
    Ok(())
}

#[test]
fn count_seed_does_not_infer_japanese_main_unit_as_support_split() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-main-chapters")?;

    scaffold_counted_documents(
        &workspace,
        file_guard(),
        "合計で百ファイルぐらいの長編小説を作ってください。二十四個の章を使い、\
         残りは順番付きの本編章にしてください。docs と本編を合計して\
         数えてください。Codex/Spark の使用量は抑えてください。",
    )?;

    assert_counts(&workspace.join("structured-output"), 12, 85)?;
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
    assert!(readme.contains(&format!("- 設計メモ: {design}")));
    assert!(readme.contains(&format!("- 本編ファイル: {main}")));
    assert!(root.join(format!("docs/design-{design:03}.md")).exists());
    assert!(!root
        .join(format!("docs/design-{:03}.md", design + 1))
        .exists());
    assert!(root.join(format!("main/part-{main:03}.md")).exists());
    assert!(!root.join(format!("main/part-{:03}.md", main + 1)).exists());
    Ok(())
}
