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
    let design_owner_links = if docs > 0 && main > 0 {
        "required"
    } else {
        "n/a"
    };
    let sequence_paths = if main > 0 { "required" } else { "n/a" };
    require_manifest_line(
        root_index,
        "- root: structured-output",
        "audit manifest root",
    )?;
    require_manifest_line(
        root_index,
        format!("- files: {target}"),
        "audit manifest file count",
    )?;
    require_manifest_line(
        root_index,
        format!("- index_files: {index_files}"),
        "audit manifest index count",
    )?;
    require_manifest_line(
        root_index,
        format!("- design_memos: {docs}"),
        "audit manifest design count",
    )?;
    require_manifest_line(
        root_index,
        format!("- main_files: {main}"),
        "audit manifest main count",
    )?;
    require_manifest_line(
        root_index,
        format!("- index_scope: {index_scope}"),
        "audit manifest index scope",
    )?;
    require_manifest_line(
        root_index,
        "- section_scope: all",
        "audit manifest section scope",
    )?;
    require_manifest_line(
        root_index,
        format!("- content_blocks: {content_blocks}"),
        "audit manifest content blocks",
    )?;
    require_manifest_line(
        root_index,
        "- restart_guide: required",
        "audit manifest restart guide",
    )?;
    require_manifest_line(
        root_index,
        format!("- design_owner_links: {design_owner_links}"),
        "audit manifest design owner links",
    )?;
    require_manifest_line(
        root_index,
        format!("- sequence_paths: {sequence_paths}"),
        "audit manifest sequence paths",
    )?;
    require_manifest_line(
        root_index,
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

fn require_manifest_line(text: &str, line: impl AsRef<str>, label: &str) -> ToolResult<()> {
    let line = line.as_ref();
    require_budget_line(text, line, line, label)
}
