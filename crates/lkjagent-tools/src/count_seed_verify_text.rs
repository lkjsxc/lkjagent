use crate::count_profile::Language;
use crate::count_profile_index::design_owner;
use crate::count_profile_paths::{design_path, main_path};
use crate::count_profile_stage::{stage_label, stage_range};
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
        require_contains(text, &design_path(index), "coverage map design entry")?;
        require_contains(text, &main_path(start), "coverage map range start")?;
        require_contains(text, &main_path(end), "coverage map range end")?;
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
    verify_progress_map(text, main)?;
    for index in 1..=main {
        let part = format!("- {}:", main_path(index));
        let line = require_line(text, &part, "main index part ledger entry")?;
        if let Some(owner) = design_owner(index, docs, main) {
            require_contains(line, &design_path(owner), "main index design owner")?;
        }
    }
    Ok("ok")
}

fn verify_progress_map(text: &str, main: usize) -> ToolResult<()> {
    for slot in 0..6 {
        let Some((start, end)) = stage_range(main, slot) else {
            continue;
        };
        require_stage_line(text, slot, start, end)?;
    }
    Ok(())
}

fn require_stage_line(text: &str, slot: usize, start: usize, end: usize) -> ToolResult<()> {
    let english = stage_line(stage_label(Language::English, slot), start, end, "through");
    let japanese = stage_line(stage_label(Language::Japanese, slot), start, end, "から");
    if text.contains(&english) || text.contains(&japanese) {
        Ok(())
    } else {
        Err(ToolError::invalid(
            "counted document scaffold missing progress map range",
        ))
    }
}

fn stage_line(label: &str, start: usize, end: usize, separator: &str) -> String {
    if start == end {
        format!("- {label}: {}", main_path(start))
    } else {
        format!(
            "- {label}: {} {separator} {}",
            main_path(start),
            main_path(end)
        )
    }
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
        require_contains(text, "main/arcs/", "design memo main coverage")?;
    }
    require_one(
        text,
        &[
            "The covered range preserves sequence continuity.",
            "担当範囲の前後関係が連続していること。",
            "追加や削除で規模目安を崩さないこと。",
        ],
        label,
    )?;
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
