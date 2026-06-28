use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;
use rusqlite::Connection;

pub struct ApplyRequest<'a> {
    pub workspace: &'a Path,
    pub conn: &'a Connection,
    pub now: &'a str,
    pub root: &'a str,
    pub title: &'a str,
    pub kind: &'a str,
    pub mode: &'a str,
    pub sections: &'a str,
}

pub fn plan(
    conn: &Connection,
    now: &str,
    root: &str,
    title: &str,
    kind: &str,
    scale: &str,
    sections: &str,
) -> ToolResult<String> {
    let kind = kind_or_default(kind, root);
    let output = crate::doc::plan(root, kind, scale_count(scale), "approx", title, sections)?;
    crate::artifact_ledger_support::record_plan(conn, root, kind, scale, now)?;
    Ok(output)
}

pub fn apply(request: ApplyRequest<'_>) -> ToolResult<String> {
    crate::artifact_address_support::ensure_apply_root(
        request.workspace,
        request.root,
        request.kind,
    )?;
    let full = workspace_path(request.workspace, request.root)?;
    if full.exists() {
        if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
            if !report.is_empty() {
                return Err(ToolError::invalid(report.block_message(request.root)));
            }
        }
    }
    let title = title_or_root(request.title, request.root);
    let kind = kind_or_default(request.kind, request.root);
    let output = crate::doc::scaffold_allow_existing(
        request.workspace,
        request.root,
        kind,
        "",
        request.mode,
        &title,
        request.sections,
    )?;
    crate::artifact_card::write(request.workspace, request.root, &title, kind)?;
    crate::artifact_ledger_support::record_apply(request.conn, request.root, kind, request.now)?;
    Ok(output)
}

pub fn audit(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
) -> ToolResult<String> {
    if let Some(report) = crate::artifact_address_support::audit_refusal(workspace, root, kind)? {
        return crate::artifact_ledger_support::record_audit(
            workspace, conn, root, kind, &report, now,
        );
    }
    if kind.trim().eq_ignore_ascii_case("dictionary") {
        let report = crate::dictionary_audit::audit(workspace, root)?;
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, kind, &report, now,
        )?;
        return Ok(report);
    }
    let report = crate::doc::audit(workspace, root, count, mode)?;
    let kind = kind.trim();
    let full = workspace_path(workspace, root)?;
    let catalog = optional_catalog(&full);
    if !kind.is_empty() && !catalog.is_empty() && kind_mismatch(kind, &catalog) {
        let report = format!(
            "document audit failed\nroot={root}\nchecks=15\npassed=14\nfailed=1\nfailures:\n- artifact_kind_mismatch: expected={kind}\nnext_action=artifact.apply matching artifact kind"
        );
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, kind, &report, now,
        )?;
        return Ok(report);
    }
    if kind.is_empty() || !report.starts_with("document audit passed") {
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, kind, &report, now,
        )?;
        return Ok(report);
    }
    if let Some(drift) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !drift.is_empty() {
            let report = drift.observation(root);
            let report = crate::artifact_ledger_support::record_audit(
                workspace, conn, root, kind, &report, now,
            )?;
            return Ok(report);
        }
    }
    let report = crate::artifact_readiness::readiness_report(kind, root, &full, &report)?;
    crate::artifact_ledger_support::record_audit(workspace, conn, root, kind, &report, now)
}

pub fn next(workspace: &Path, root: &str, path: &str, kind: &str) -> ToolResult<String> {
    crate::artifact_next::next(workspace, root, path, kind)
}

pub fn next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    path: &str,
    kind: &str,
) -> ToolResult<String> {
    crate::artifact_next::next_with_cursor(workspace, conn, now, root, path, kind)
}

fn scale_count(scale: &str) -> &str {
    let trimmed = scale.trim();
    if trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        trimmed
    } else {
        ""
    }
}

fn kind_or_default<'a>(kind: &'a str, root: &str) -> &'a str {
    let trimmed = kind.trim();
    if (trimmed.is_empty() || trimmed.eq_ignore_ascii_case("artifact")) && story_root(root) {
        return "story";
    }
    if trimmed.is_empty() {
        "artifact"
    } else {
        trimmed
    }
}

fn story_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

fn title_or_root(title: &str, root: &str) -> String {
    let trimmed = title.trim();
    if !trimmed.is_empty() {
        return trimmed.to_string();
    }
    root.rsplit('/')
        .next()
        .map_or("Artifact", |name| name)
        .replace('-', " ")
}

fn kind_mismatch(kind: &str, catalog: &str) -> bool {
    match kind.trim().to_ascii_lowercase().as_str() {
        "story" => !story_catalog(catalog),
        "cookbook" => !catalog.contains("Cookbook"),
        _ => false,
    }
}

fn story_catalog(catalog: &str) -> bool {
    let lower = catalog.to_ascii_lowercase();
    catalog.contains("NarrativeManuscript")
        || lower.contains("kind = \"story\"")
        || lower.contains("story bible")
}

#[allow(clippy::manual_unwrap_or_default)]
fn optional_catalog(root: &Path) -> String {
    match fs::read_to_string(root.join("catalog.toml")) {
        Ok(text) => text,
        Err(_) => String::new(),
    }
}

pub fn reject_empty_root(root: &str) -> ToolResult<()> {
    if root.trim().is_empty() {
        return Err(ToolError::invalid("artifact root must not be empty"));
    }
    Ok(())
}
