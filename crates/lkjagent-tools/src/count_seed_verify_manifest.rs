use crate::count_guard::{CountGuard, CountMode};
use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_audit_manifest(
    root_index: &str,
    guard: CountGuard,
    files: usize,
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
    let local_verification = if main > 0 { "required" } else { "n/a" };
    let reading_path = if main > 0 { "required" } else { "n/a" };
    let sequence_paths = if main > 0 { "required" } else { "n/a" };
    require_manifest_line(
        root_index,
        "- root: structured-output",
        "audit manifest root",
    )?;
    match guard.mode {
        CountMode::Exact => require_manifest_line(
            root_index,
            format!("- files: {}", guard.target),
            "audit manifest file count",
        )?,
        CountMode::Approximate => require_manifest_line(
            root_index,
            format!("- scale_files: about {files}"),
            "audit manifest scale file count",
        )?,
    }
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
        format!("- local_verification: {local_verification}"),
        "audit manifest local verification",
    )?;
    require_manifest_line(
        root_index,
        format!("- reading_path: {reading_path}"),
        "audit manifest reading path",
    )?;
    require_manifest_line(
        root_index,
        format!("- sequence_paths: {sequence_paths}"),
        "audit manifest sequence paths",
    )?;
    require_manifest_line(
        root_index,
        "- closure_reason: deterministic_scaffold",
        "audit manifest closure reason",
    )?;
    require_manifest_line(
        root_index,
        "- completion: ready",
        "audit manifest completion",
    )?;
    Ok("ok")
}

fn require_manifest_line(text: &str, line: impl AsRef<str>, label: &str) -> ToolResult<()> {
    let line = line.as_ref();
    if text.contains(line) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
}
