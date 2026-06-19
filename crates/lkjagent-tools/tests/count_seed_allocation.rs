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

#[test]
fn count_seed_honors_kanji_design_count_hint_inside_total() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-kanji-design-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "合計百ファイル程度の大きな物語を作ってください。二十章ぶんの設計観点も含めます。",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-020.md").exists());
    assert!(!root.join("docs/design-021.md").exists());
    assert!(root.join("main/part-077.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- 設計メモ: 20"));
    assert!(readme.contains("- 本編ファイル: 77"));
    Ok(())
}

#[test]
fn count_seed_honors_english_word_design_count_hint_inside_total() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-english-word-design-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "Create about one hundred files total with twenty design memos and ordered main files.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-020.md").exists());
    assert!(!root.join("docs/design-021.md").exists());
    assert!(root.join("main/part-077.md").exists());
    let late_design = fs::read_to_string(root.join("docs/design-020.md"))?;
    assert!(late_design.contains("release readiness record"));
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 20"));
    assert!(readme.contains("- Main files: 77"));
    Ok(())
}

#[test]
fn count_seed_honors_outline_count_hint_inside_total() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-outline-hint")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "Create hundred files total with twenty outline files and ordered main files.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-020.md").exists());
    assert!(!root.join("docs/design-021.md").exists());
    assert!(root.join("main/part-077.md").exists());
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 20"));
    assert!(readme.contains("- Main files: 77"));
    Ok(())
}

#[test]
fn count_seed_keeps_file_count_stronger_than_design_wording() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-total-file-not-design")?;

    let report = scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Exact,
        },
        "Create about 100 files with a simple design and clear reading order.",
    )?;

    let root = workspace.join("structured-output");
    assert!(root.join("docs/design-012.md").exists());
    assert!(!root.join("docs/design-013.md").exists());
    assert!(root.join("main/part-085.md").exists());
    let mid_design = fs::read_to_string(root.join("docs/design-006.md"))?;
    assert!(mid_design.contains("## Verification Checks"));
    let mid_part = fs::read_to_string(root.join("main/part-042.md"))?;
    assert!(mid_part.contains("## Continuity Hand-Off"));
    let readme = fs::read_to_string(root.join("README.md"))?;
    assert!(readme.contains("- Design memos: 12"));
    assert!(readme.contains("- Main files: 85"));
    assert!(report.contains("index_files=2"));
    assert!(report.contains("root_index=ok"));
    assert!(report.contains("docs_index=ok"));
    assert!(report.contains("coverage_map=ok"));
    assert!(report.contains("main_index=ok"));
    assert!(report.contains("acceptance_audit=ok"));
    assert!(report.contains("part_ledger=ok"));
    assert!(report.contains("section_scope=all"));
    assert!(report.contains("design_sections=ok"));
    assert!(report.contains("main_sections=ok"));
    assert!(report.contains("first_main=ok"));
    assert!(report.contains("last_main=ok"));
    Ok(())
}

#[test]
fn count_seed_readme_preserves_approximate_count_mode() -> TestResult<()> {
    let workspace = temp_workspace("count-seed-approximate-mode")?;

    scaffold_counted_documents(
        &workspace,
        CountGuard {
            kind: CountKind::File,
            target: 100,
            mode: CountMode::Approximate,
        },
        "Create about 100 files total with twenty design memos and ordered main files.",
    )?;

    let readme = fs::read_to_string(workspace.join("structured-output/README.md"))?;
    assert!(readme.contains("within the approximate-count guard"));
    assert!(!readme.contains("exact counted deliverable"));
    Ok(())
}

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
