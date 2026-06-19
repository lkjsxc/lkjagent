mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::benchmark_seed::scaffold_markdown_corpus;
use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use support::{temp_workspace, TestResult};

#[test]
fn benchmark_seed_creates_exact_markdown_only_tree() -> TestResult<()> {
    let workspace = temp_workspace("benchmark-seed")?;

    let report = scaffold_markdown_corpus(&workspace, 12)?;

    assert!(report.contains("markdown_files=12"));
    let root = workspace.join("docs/benchmark-corpus");
    assert_eq!(counts(&root)?.markdown, 12);
    assert_eq!(counts(&root)?.other, 0);
    assert_readmes(&root)?;
    let sample = fs::read_to_string(root.join("api/topic-001.md"))?;
    assert!(sample.contains("# API Topic 1"));
    assert!(sample.contains("## Purpose"));
    Ok(())
}

#[test]
fn count_seed_creates_exact_markdown_tree() -> TestResult<()> {
    let workspace = temp_workspace("count-seed")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 20,
            mode: CountMode::Exact,
        },
        "Create a structured archive for the northern expedition.",
    )?;

    assert!(report.contains("files=20"));
    assert!(report.contains("design_memos=12"));
    assert!(report.contains("main_files=5"));
    let root = workspace.join("structured-output");
    assert_eq!(counts(&root)?.markdown, 20);
    assert_eq!(counts(&root)?.other, 0);
    assert!(root.join("README.md").exists());
    assert!(root.join("docs/README.md").exists());
    assert!(root.join("main/README.md").exists());
    assert!(root.join("docs/design-001.md").exists());
    assert!(root.join("main/part-001.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("northern expedition"));
    assert!(readme.contains("## File Budget"));
    assert!(readme.contains("- Total files: 20"));
    assert!(readme.contains("## Acceptance Audit"));
    assert!(readme.contains("Part Ledger in main/README.md"));
    let docs_index = fs::read_to_string(root.join("docs/README.md"))?;
    assert!(docs_index.contains("Design memo count"));
    assert!(docs_index.contains("## Coverage Map"));
    assert!(docs_index.contains("design-001.md: main/part-001.md"));
    let main_index = fs::read_to_string(root.join("main/README.md"))?;
    assert!(main_index.contains("Main file count"));
    assert!(main_index.contains("## Progress Map"));
    assert!(main_index.contains("main/part-001.md"));
    assert!(main_index.contains("## Part Ledger"));
    assert!(main_index.contains("main/part-001.md: opening / scope unit"));
    assert!(main_index.contains("main/part-002.md: exploration / premise unit"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    let second_part = fs::read_to_string(root.join("main/part-002.md"))?;
    assert!(first_part.contains("## Segment Brief"));
    assert!(first_part.contains("File role: opening / scope unit"));
    assert!(second_part.contains("File role: exploration / premise unit"));
    assert!(first_part.contains("Continuity Hand-Off"));
    assert!(first_part.contains("Arc: 1"));
    assert!(first_part.contains("### Draft Passage"));
    assert!(first_part.contains("## Local Verification"));
    Ok(())
}

#[test]
fn count_seed_profiles_japanese_narrative_output() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-japanese")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 20,
            mode: CountMode::Exact,
        },
        "100ファイルぐらいの大きな物語を作ってください。",
    )?;

    let root = workspace.join("structured-output");
    assert_eq!(counts(&root)?.markdown, 20);
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("# 構造化成果物"));
    assert!(readme.contains("## 要求アンカー"));
    assert!(readme.contains("大きな物語"));
    assert!(readme.contains("## ファイル内訳"));
    assert!(readme.contains("- 合計ファイル数: 20"));
    assert!(readme.contains("## 受入監査"));
    let docs_index = fs::read_to_string(root.join("docs/README.md"))?;
    assert!(docs_index.contains("設計メモ数"));
    assert!(docs_index.contains("## 設計対応表"));
    assert!(docs_index.contains("design-001.md: main/part-001.md"));
    let main_index = fs::read_to_string(root.join("main/README.md"))?;
    assert!(main_index.contains("本編ファイル数"));
    assert!(main_index.contains("## 進行地図"));
    assert!(main_index.contains("main/part-001.md"));
    let design = fs::read_to_string(root.join("docs/design-001.md"))?;
    assert!(design.contains("範囲と受け入れ条件"));
    assert!(design.contains("## 対象範囲"));
    assert!(design.contains("main/part-001.md"));
    assert!(design.contains("## 検証観点"));
    let first_part = fs::read_to_string(root.join("main/part-001.md"))?;
    assert!(first_part.contains("# 本編 001"));
    assert!(first_part.contains("## セグメント概要"));
    assert!(first_part.contains("ファイル役割: 導入 / 発端の圧力"));
    assert!(first_part.contains("## 要求アンカー"));
    assert!(first_part.contains("## 本文"));
    assert!(first_part.contains("## ローカル検証"));
    assert!(first_part.contains("## 連続性台帳"));
    assert!(first_part.contains("- 前: なし"));
    assert!(first_part.contains("- 現在: main/part-001.md"));
    assert!(first_part.contains("- 次: main/part-002.md"));
    assert!(first_part.contains("### 場面の役割"));
    assert!(first_part.contains("### 転換点"));
    assert!(first_part.contains("### 具体化メモ"));
    assert!(first_part.contains("記録係の視点"));
    assert!(first_part.contains("### 本文断片"));
    assert!(first_part.contains("次へ進む理由"));
    assert!(first_part.contains("### 要求との接続"));
    assert!(first_part.contains("この節では「100ファイルぐらいの大きな物語を作ってください」"));
    Ok(())
}

#[test]
fn count_seed_keeps_decimal_version_inside_objective_anchor() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-version-anchor")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 8,
            mode: CountMode::Exact,
        },
        "Use GPT-5.3-Codex-Spark thrift. Create about 100 files total.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("- Use GPT-5.3-Codex-Spark thrift"));
    assert!(!readme.contains("- Use GPT-5\n- 3-Codex-Spark thrift"));
    let first_part = fs::read_to_string(workspace.join("structured-output/main/part-001.md"))?;
    assert!(first_part.contains(
        "Local objective: Turn \"Create about 100 files total\" into this file's distinct contribution."
    ));
    assert!(!first_part.contains("Local objective: Turn \"Use GPT-5.3-Codex-Spark thrift\""));
    Ok(())
}

fn assert_readmes(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let child = entry?.path();
        if child.is_dir() {
            assert!(child.join("README.md").exists(), "missing {:?}", child);
            assert_readmes(&child)?;
        }
    }
    Ok(())
}

#[derive(Default)]
struct Counts {
    markdown: usize,
    other: usize,
}

fn counts(path: &Path) -> TestResult<Counts> {
    let mut totals = Counts::default();
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            let nested = counts(&child)?;
            totals.markdown = totals.markdown.saturating_add(nested.markdown);
            totals.other = totals.other.saturating_add(nested.other);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            totals.markdown = totals.markdown.saturating_add(1);
        } else {
            totals.other = totals.other.saturating_add(1);
        }
    }
    Ok(totals)
}
