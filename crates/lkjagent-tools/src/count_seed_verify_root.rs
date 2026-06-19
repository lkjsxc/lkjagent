use crate::error::{ToolError, ToolResult};

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
