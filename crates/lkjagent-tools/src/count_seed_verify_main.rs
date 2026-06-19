use crate::count_profile_index::design_owner;
use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_main_file_sections(text: &str, label: &str) -> ToolResult<()> {
    require_one(text, &["## Segment Brief", "## セグメント概要"], label)?;
    require_one(text, &["## Sequence Ledger", "## 連続性台帳"], label)?;
    require_one(text, &["## Draft Content", "## 本文"], label)?;
    require_one(text, &["## Continuity Hand-Off", "## 継続メモ"], label)?;
    Ok(())
}

pub(crate) fn verify_main_file_content(
    text: &str,
    label: &str,
    index: usize,
    docs: usize,
    main: usize,
) -> ToolResult<()> {
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
    if let Some(owner) = design_owner(index, docs, main) {
        require_contains(
            text,
            &format!("docs/design-{owner:03}.md"),
            "main file design owner",
        )?;
    }
    require_contains(
        text,
        &format!("main/part-{index:03}.md"),
        "main file current path",
    )?;
    if index > 1 {
        require_contains(
            text,
            &format!("main/part-{:03}.md", index.saturating_sub(1)),
            "main file previous path",
        )?;
    }
    if index < main {
        require_contains(
            text,
            &format!("main/part-{:03}.md", index.saturating_add(1)),
            "main file next path",
        )?;
    }
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
