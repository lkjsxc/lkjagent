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

pub(crate) fn verify_audit_manifest(
    root_index: &str,
    target: usize,
    docs: usize,
    main: usize,
    index_files: usize,
) -> ToolResult<&'static str> {
    if !root_index.contains("## Audit Manifest") && !root_index.contains("## 監査マニフェスト")
    {
        return Err(ToolError::invalid(
            "counted document scaffold missing audit manifest",
        ));
    }
    let has_content = docs > 0 || main > 0;
    let index_scope = if index_files > 0 && has_content {
        "all"
    } else {
        "n/a"
    };
    let content_blocks = if has_content { "required" } else { "n/a" };
    require_budget_line(
        root_index,
        "- root: structured-output",
        "- root: structured-output",
        "audit manifest root",
    )?;
    require_budget_line(
        root_index,
        &format!("- files: {target}"),
        &format!("- files: {target}"),
        "audit manifest file count",
    )?;
    require_budget_line(
        root_index,
        &format!("- index_files: {index_files}"),
        &format!("- index_files: {index_files}"),
        "audit manifest index count",
    )?;
    require_budget_line(
        root_index,
        &format!("- design_memos: {docs}"),
        &format!("- design_memos: {docs}"),
        "audit manifest design count",
    )?;
    require_budget_line(
        root_index,
        &format!("- main_files: {main}"),
        &format!("- main_files: {main}"),
        "audit manifest main count",
    )?;
    require_budget_line(
        root_index,
        &format!("- index_scope: {index_scope}"),
        &format!("- index_scope: {index_scope}"),
        "audit manifest index scope",
    )?;
    require_budget_line(
        root_index,
        "- section_scope: all",
        "- section_scope: all",
        "audit manifest section scope",
    )?;
    require_budget_line(
        root_index,
        &format!("- content_blocks: {content_blocks}"),
        &format!("- content_blocks: {content_blocks}"),
        "audit manifest content blocks",
    )?;
    require_budget_line(
        root_index,
        "- completion: ready",
        "- completion: ready",
        "audit manifest completion",
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
