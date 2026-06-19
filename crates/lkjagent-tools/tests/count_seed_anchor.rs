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

#[test]
fn count_seed_japanese_main_anchor_trims_parenthetical_file_total_prefix() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-japanese-parenthetical-count")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "プロンプト一発で100ファイル（そのドキュメントのファイルたちと実際の本編のファイルたちを合計して）ぐらいの大きな物語を作ってください。Codexの枠を節約してください。",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("100ファイル（そのドキュメント"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(
        first_part.contains("局所目的: 要求「大きな物語」をこのファイル固有の成果に変換します。")
    );
    assert_no_local_objective_starts_with(&root, "プロンプト一発で100ファイル")?;
    Ok(())
}

#[test]
fn count_seed_english_report_skips_docs_total_constraint_anchor() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-report-content-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Use GPT-5.3-Codex-Spark thrift. Create about 100 files total for a market research dossier with 18 analysis memos and ordered main report sections. Keep docs and main content together in the total.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("Kind contract: audit this deliverable as a report"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(first_part.contains("### Analysis Role"));
    assert!(first_part.contains(
        "Local objective: Turn \"a market research dossier with 18 analysis memos and ordered main report…\""
    ));
    assert_no_local_objective_starts_with(&root, "Keep docs")?;
    Ok(())
}

#[test]
fn count_seed_english_runbook_not_story_specific_stays_guide() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-runbook-content-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about 100 files total for an operations runbook with 12 planning docs and ordered main procedure files. Keep it generic and reusable, not story-specific.",
    )?;

    let root = workspace.join("structured-output");
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("Kind contract: audit this deliverable as a guide"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(first_part.contains("### Procedure Role"));
    assert!(!first_part.contains("### Scene Role"));
    assert!(first_part.contains(
        "Local objective: Turn \"an operations runbook with 12 planning docs and ordered main procedure f…\""
    ));
    assert_no_local_objective_starts_with(&root, "Keep it generic")?;
    Ok(())
}

fn assert_no_local_objective_starts_with(root: &std::path::Path, prefix: &str) -> TestResult<()> {
    for entry in fs::read_dir(root.join("main"))? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str()) == Some("md") {
            let text = fs::read_to_string(path)?;
            assert!(
                !text.contains(&format!("Local objective: Turn \"{prefix}")),
                "unexpected local objective prefix {prefix}"
            );
        }
    }
    Ok(())
}
