use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;
use rusqlite::Connection;

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
    let kind = audit_kind(workspace, conn, root, kind)?;
    if kind.eq_ignore_ascii_case("dictionary") {
        let report = crate::dictionary_audit::audit(workspace, root)?;
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    let report = crate::doc::audit(workspace, root, count, mode)?;
    let full = workspace_path(workspace, root)?;
    let catalog = optional_catalog(&full);
    if !catalog.is_empty() && kind_mismatch(root, &kind, &catalog) {
        let report = format!(
            "document audit failed\nroot={root}\nchecks=15\npassed=14\nfailed=1\nfailures:\n- artifact_kind_mismatch: expected={kind}\nnext_action=artifact.next identity contract for matching artifact kind"
        );
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    if !report.starts_with("document audit passed") {
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    if let Some(drift) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !drift.is_empty() {
            let report = drift.observation(root);
            let report = crate::artifact_ledger_support::record_audit(
                workspace, conn, root, &kind, &report, now,
            )?;
            return Ok(report);
        }
    }
    let report = crate::artifact_readiness::readiness_report(&kind, root, &full, &report)?;
    crate::artifact_ledger_support::record_audit(workspace, conn, root, &kind, &report, now)
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

fn audit_kind(
    workspace: &Path,
    conn: &Connection,
    root: &str,
    requested: &str,
) -> ToolResult<String> {
    let trimmed = requested.trim();
    if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("artifact") {
        return Ok(trimmed.to_ascii_lowercase());
    }
    if let Some(kind) = ledger_kind(conn, root)? {
        return Ok(kind);
    }
    let full = workspace_path(workspace, root)?;
    let catalog = optional_catalog(&full);
    if story_catalog(&catalog) || story_root(root) {
        return Ok("story".to_string());
    }
    if catalog.to_ascii_lowercase().contains("cookbook") {
        return Ok("cookbook".to_string());
    }
    Ok("artifact".to_string())
}

fn ledger_kind(conn: &Connection, root: &str) -> ToolResult<Option<String>> {
    let case_id = crate::artifact_ledger_state::case_id(conn)?;
    let Some(row) = lkjagent_store::artifact_ledger::latest_for_case(conn, case_id)? else {
        return Ok(None);
    };
    Ok((row.root == root).then_some(row.kind))
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

fn kind_mismatch(root: &str, kind: &str, catalog: &str) -> bool {
    match kind.trim().to_ascii_lowercase().as_str() {
        "story" if story_root(root) => false,
        "story" => !story_catalog(catalog),
        "cookbook" => !catalog.to_ascii_lowercase().contains("cookbook"),
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
