mod support;

use std::fs;

use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn count_seed_main_anchor_uses_content_topic_not_count_command() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-content-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Use GPT-5.3-Codex-Spark thrift. Create about 100 files total for a large structured story deliverable with docs and main content.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("- Use GPT-5.3-Codex-Spark thrift"));
    let first_part = fs::read_to_string(workspace.join("structured-output/main/part-001.md"))?;
    assert!(first_part.contains(
        "Local objective: Turn \"a large structured story deliverable\" into this file's distinct contribution."
    ));
    assert!(!first_part.contains("Local objective: Turn \"Create about 100 files total"));
    Ok(())
}

#[test]
fn count_seed_japanese_main_anchor_skips_count_and_budget_clauses() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-japanese-content-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "合計100ファイルぐらいで、15個の設計メモと本編を含む大きな物語成果物を作ってください。ドキュメント群と実際の本編ファイル群を合わせた総数です。Codex/Sparkの枠を節約してください。",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("- 合計100ファイルぐらいで"));
    let first_part = fs::read_to_string(workspace.join("structured-output/main/part-001.md"))?;
    assert!(first_part.contains(
        "局所目的: 要求「15個の設計メモと本編を含む大きな物語成果物」をこのファイル固有の成果に変換します。"
    ));
    assert!(!first_part.contains("局所目的: 要求「合計100ファイルぐらいで"));
    assert!(!first_part.contains("局所目的: 要求「Codex/Spark"));
    Ok(())
}
