use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_acceptance_audit(
    root_index: &str,
    docs: usize,
    main: usize,
) -> ToolResult<&'static str> {
    if !root_index.contains("## Acceptance Audit") && !root_index.contains("## 受入監査") {
        return Err(ToolError::invalid(
            "counted document scaffold missing acceptance audit",
        ));
    }
    require_budget_line(
        root_index,
        "README.md",
        "README.md",
        "acceptance entry point",
    )?;
    require_budget_line(
        root_index,
        &format!("Design coverage: {docs} design memos"),
        &format!("設計範囲: {docs} 件の設計メモ"),
        "acceptance design coverage",
    )?;
    require_budget_line(
        root_index,
        &format!("Main coverage: {main} main files"),
        &format!("本編範囲: {main} 件の main ファイル"),
        "acceptance main coverage",
    )?;
    require_budget_line(
        root_index,
        "Content contract: every main file carries",
        "内容契約: 各 main ファイルは",
        "acceptance content contract",
    )?;
    require_budget_line(
        root_index,
        "Kind contract: audit this deliverable as",
        "種別契約: この成果物は",
        "acceptance kind contract",
    )?;
    Ok("ok")
}

pub(crate) fn verify_file_budget(
    root_index: &str,
    target: usize,
    docs: usize,
    main: usize,
    index_files: usize,
) -> ToolResult<&'static str> {
    if !root_index.contains("## File Budget") && !root_index.contains("## ファイル内訳") {
        return Err(ToolError::invalid(
            "counted document scaffold missing file budget",
        ));
    }
    require_budget_line(
        root_index,
        "- Root index: 1",
        "- ルート索引: 1",
        "root index count",
    )?;
    require_budget_line(
        root_index,
        &format!("- Directory indexes: {index_files}"),
        &format!("- ディレクトリ索引: {index_files}"),
        "directory index count",
    )?;
    require_budget_line(
        root_index,
        &format!("- Design memos: {docs}"),
        &format!("- 設計メモ: {docs}"),
        "design memo count",
    )?;
    require_budget_line(
        root_index,
        &format!("- Main files: {main}"),
        &format!("- 本編ファイル: {main}"),
        "main file count",
    )?;
    require_budget_line(
        root_index,
        &format!("- Total files: {target}"),
        &format!("- 合計ファイル数: {target}"),
        "total file count",
    )?;
    Ok("ok")
}

fn require_budget_line(text: &str, english: &str, japanese: &str, label: &str) -> ToolResult<()> {
    if text.contains(english) || text.contains(japanese) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
}
