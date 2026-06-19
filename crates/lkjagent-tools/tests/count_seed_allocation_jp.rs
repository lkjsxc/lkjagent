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

#[test]
fn count_seed_honors_japanese_setting_reference_file_count_phrase() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-setting-reference-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "ドキュメント用ファイルと本編ファイルを合計して百ファイルくらいの長編物語を\
         作ってください。設定資料ファイルを十八個、残りは章ごとの本編ファイルに\
         してください。Codex/Spark の消費は抑えてください。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-018.md").exists());
    assert!(!root.join("docs/design-019.md").exists());
    assert!(root.join("main/part-079.md").exists());
    assert!(!root.join("main/part-080.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 18"));
    assert!(readme.contains("- 本編ファイル: 79"));
    assert!(readme.contains("この成果物は 物語 として監査します"));
    Ok(())
}

#[test]
fn count_seed_honors_japanese_character_introductions_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-character-introductions")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "合計百ファイル程度の大きな物語を作ってください。二十四個の登場人物紹介を\
         使い、残りは順番付きの本編章にしてください。ドキュメントと本編を合わせて\
         数えてください。Codex/Sparkの予算は低く抑えてください。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 24"));
    assert!(readme.contains("- 本編ファイル: 73"));
    assert!(readme.contains("この成果物は 物語 として監査します"));
    Ok(())
}

#[test]
fn count_seed_honors_japanese_relationship_charts_without_file_noun() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-jp-relationship-charts")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "合計で百ファイルぐらいの長編小説を作ってください。二十四個の人物相関図を\
         使い、残りは順番付きの本編章にしてください。docs と本編を合計して\
         数えてください。Codex/Spark の使用量は抑えてください。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-024.md").exists());
    assert!(!root.join("docs/design-025.md").exists());
    assert!(root.join("main/part-073.md").exists());
    assert!(!root.join("main/part-074.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 24"));
    assert!(readme.contains("- 本編ファイル: 73"));
    assert!(readme.contains("この成果物は 物語 として監査します"));
    Ok(())
}
