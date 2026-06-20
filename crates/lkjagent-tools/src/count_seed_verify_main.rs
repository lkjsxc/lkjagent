use crate::count_profile_index::design_owner;
use crate::count_profile_paths::{design_path, main_path};
use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_main_file_sections(text: &str, label: &str) -> ToolResult<()> {
    require_one(text, &["## Segment Brief", "## セグメント概要"], label)?;
    require_one(text, &["## Sequence Ledger", "## 連続性台帳"], label)?;
    require_one(text, &["## Draft Content", "## 本文"], label)?;
    require_one(text, &["## Local Verification", "## ローカル検証"], label)?;
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
    require_one(
        text,
        &[
            "Design-owner status is recorded and checked before edits.",
            "設計担当の状態を記録し、編集前に確認します。",
        ],
        label,
    )?;
    require_one(
        text,
        &[
            "Sequence ledger names previous, current, and next paths.",
            "連続性台帳は前・現在・次のパスを示します。",
        ],
        label,
    )?;
    require_one(
        text,
        &[
            "Draft content includes concrete detail, passage, and requirement link.",
            "本文は固有要素、本文断片、要求との接続を含みます。",
        ],
        label,
    )?;
    require_one(
        text,
        &[
            "Handoff names the state later files can continue from.",
            "継続メモは後続ファイルが続けられる状態を示します。",
        ],
        label,
    )?;
    if let Some(owner) = design_owner(index, docs, main) {
        require_contains(text, &design_path(owner), "main file design owner")?;
    }
    require_contains(text, &main_path(index), "main file current path")?;
    if index > 1 {
        require_contains(
            text,
            &main_path(index.saturating_sub(1)),
            "main file previous path",
        )?;
    }
    if index < main {
        require_contains(
            text,
            &main_path(index.saturating_add(1)),
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
