use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_acceptance_audit(root_index: &str) -> ToolResult<&'static str> {
    if root_index.contains("## Acceptance Audit") || root_index.contains("## 受入監査") {
        require_contains(root_index, "README.md", "acceptance audit entry point")?;
        Ok("ok")
    } else {
        Err(ToolError::invalid(
            "counted document scaffold missing acceptance audit",
        ))
    }
}

pub(crate) fn verify_coverage_map(
    docs_index: Option<&str>,
    main: usize,
) -> ToolResult<&'static str> {
    let Some(text) = docs_index else {
        return Ok("n/a");
    };
    if main == 0 {
        return Ok("n/a");
    }
    if !text.contains("## Coverage Map") && !text.contains("## 設計対応表") {
        return Err(ToolError::invalid(
            "counted document scaffold missing coverage map",
        ));
    }
    require_contains(text, "part-001.md", "coverage map first main")?;
    require_contains(
        text,
        &format!("part-{main:03}.md"),
        "coverage map last main",
    )?;
    Ok("ok")
}

pub(crate) fn verify_part_ledger(
    main_index: Option<&str>,
    main: usize,
) -> ToolResult<&'static str> {
    let Some(text) = main_index else {
        return Ok("n/a");
    };
    if main == 0 {
        return Ok("n/a");
    }
    require_contains(text, "part-001.md", "main index first part")?;
    require_contains(text, &format!("part-{main:03}.md"), "main index last part")?;
    if !text.contains("## Part Ledger") && !text.contains("## 本編台帳") {
        return Err(ToolError::invalid(
            "counted document scaffold missing part ledger",
        ));
    }
    Ok("ok")
}

pub(crate) fn verify_main_sections(
    first: Option<&str>,
    last: Option<&str>,
) -> ToolResult<&'static str> {
    let Some(first) = first else {
        return Ok("n/a");
    };
    let last = last.unwrap_or(first);
    require_main_sections(first, "first main part")?;
    require_main_sections(last, "last main part")?;
    Ok("ok")
}

fn require_main_sections(text: &str, label: &str) -> ToolResult<()> {
    require_one(text, &["## Segment Brief", "## セグメント概要"], label)?;
    require_one(text, &["## Sequence Ledger", "## 連続性台帳"], label)?;
    require_one(text, &["## Draft Content", "## 本文"], label)?;
    require_one(text, &["## Continuity Hand-Off", "## 継続メモ"], label)?;
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
