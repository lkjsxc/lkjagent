use crate::count_profile_index::design_owner;
use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_coverage_map(
    docs_index: Option<&str>,
    docs: usize,
    main: usize,
) -> ToolResult<&'static str> {
    let Some(text) = docs_index else {
        return Ok("n/a");
    };
    if docs == 0 || main == 0 {
        return Ok("n/a");
    }
    if !text.contains("## Coverage Map") && !text.contains("## 設計対応表") {
        return Err(ToolError::invalid(
            "counted document scaffold missing coverage map",
        ));
    }
    for index in 1..=docs {
        let Some((start, end)) = coverage_range(index, docs, main) else {
            continue;
        };
        require_contains(
            text,
            &format!("design-{index:03}.md"),
            "coverage map design entry",
        )?;
        require_contains(
            text,
            &format!("main/part-{start:03}.md"),
            "coverage map range start",
        )?;
        require_contains(
            text,
            &format!("main/part-{end:03}.md"),
            "coverage map range end",
        )?;
    }
    Ok("ok")
}

pub(crate) fn verify_part_ledger(
    main_index: Option<&str>,
    docs: usize,
    main: usize,
) -> ToolResult<&'static str> {
    let Some(text) = main_index else {
        return Ok("n/a");
    };
    if main == 0 {
        return Ok("n/a");
    }
    if !text.contains("## Part Ledger") && !text.contains("## 本編台帳") {
        return Err(ToolError::invalid(
            "counted document scaffold missing part ledger",
        ));
    }
    for index in 1..=main {
        let part = format!("- main/part-{index:03}.md:");
        let line = require_line(text, &part, "main index part ledger entry")?;
        if let Some(owner) = design_owner(index, docs, main) {
            require_contains(
                line,
                &format!("docs/design-{owner:03}.md"),
                "main index design owner",
            )?;
        }
    }
    Ok("ok")
}

pub(crate) fn verify_design_file_sections(text: &str, label: &str) -> ToolResult<()> {
    require_one(text, &["## Focus", "## 焦点"], label)?;
    require_one(text, &["## Coverage", "## 対象範囲"], label)?;
    require_one(text, &["## Design Task", "## 設計タスク"], label)?;
    require_one(text, &["## Verification Checks", "## 検証観点"], label)?;
    Ok(())
}

pub(crate) fn verify_design_file_content(
    text: &str,
    label: &str,
    has_main: bool,
) -> ToolResult<()> {
    require_one(text, &["## Objective Context", "## 依頼文"], label)?;
    require_one(text, &["## Requirement Anchors", "## 要求アンカー"], label)?;
    if has_main {
        require_contains(text, "main/part-", "design memo main coverage")?;
    }
    require_one(
        text,
        &[
            "The covered range preserves sequence continuity.",
            "担当範囲の前後関係が連続していること。",
        ],
        label,
    )?;
    Ok(())
}

pub(crate) fn verify_main_file_sections(text: &str, label: &str) -> ToolResult<()> {
    require_one(text, &["## Segment Brief", "## セグメント概要"], label)?;
    require_one(text, &["## Sequence Ledger", "## 連続性台帳"], label)?;
    require_one(text, &["## Draft Content", "## 本文"], label)?;
    require_one(text, &["## Continuity Hand-Off", "## 継続メモ"], label)?;
    Ok(())
}

pub(crate) fn verify_main_file_content(text: &str, label: &str) -> ToolResult<()> {
    require_one(
        text,
        &[
            "### Concrete Commitments",
            "### Execution Commitments",
            "### Analysis Commitments",
            "### Deliverable Commitments",
            "### 具体化メモ",
            "### 実行メモ",
            "### 分析メモ",
            "### 成果物メモ",
        ],
        label,
    )?;
    require_one(text, &["### Draft Passage", "### 本文断片"], label)?;
    require_one(text, &["### Specific Detail", "### 固有要素"], label)?;
    require_one(text, &["### Requirement Link", "### 要求との接続"], label)?;
    Ok(())
}

fn require_one(text: &str, needles: &[&str], label: &str) -> ToolResult<()> {
    if needles.iter().any(|needle| text.contains(needle)) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing sections in {label}"
        )))
    }
}

fn require_contains(text: &str, needle: &str, label: &str) -> ToolResult<()> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
}

fn require_line<'a>(text: &'a str, needle: &str, label: &str) -> ToolResult<&'a str> {
    text.lines()
        .find(|line| line.contains(needle))
        .ok_or_else(|| ToolError::invalid(format!("counted document scaffold missing {label}")))
}

fn coverage_range(index: usize, docs: usize, main: usize) -> Option<(usize, usize)> {
    if docs == 0 || main == 0 {
        return None;
    }
    let slot = index.saturating_sub(1).min(docs.saturating_sub(1));
    let start = slot.saturating_mul(main) / docs + 1;
    let end = (slot.saturating_add(1)).saturating_mul(main) / docs;
    Some((start.min(main), end.max(start).min(main)))
}
