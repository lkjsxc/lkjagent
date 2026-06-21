use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;
use rusqlite::Connection;

pub fn plan(
    root: &str,
    title: &str,
    kind: &str,
    scale: &str,
    sections: &str,
) -> ToolResult<String> {
    crate::doc::plan(root, kind, scale_count(scale), "approx", title, sections)
}

pub fn apply(
    workspace: &Path,
    root: &str,
    title: &str,
    kind: &str,
    mode: &str,
    sections: &str,
) -> ToolResult<String> {
    let title = title_or_root(title, root);
    crate::doc::scaffold(
        workspace,
        root,
        kind_or_default(kind),
        "",
        mode,
        &title,
        sections,
    )
}

pub fn audit(
    workspace: &Path,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
) -> ToolResult<String> {
    if kind.trim().eq_ignore_ascii_case("dictionary") {
        return crate::dictionary_audit::audit(workspace, root);
    }
    let report = crate::doc::audit(workspace, root, count, mode)?;
    let kind = kind.trim();
    if kind.is_empty() || !report.starts_with("document audit passed") {
        return Ok(report);
    }
    let full = workspace_path(workspace, root)?;
    let manifest = optional_manifest(&full);
    if kind_mismatch(kind, &manifest) {
        return Ok(format!(
            "document audit failed\nroot={root}\nchecks=15\npassed=14\nfailed=1\nfailures:\n- artifact_kind_mismatch: expected={kind}\nnext_action=artifact.apply matching artifact kind"
        ));
    }
    Ok(readiness_report(kind, &report))
}

pub fn next(workspace: &Path, root: &str, kind: &str) -> ToolResult<String> {
    crate::artifact_next::next(workspace, root, kind)
}

pub fn next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    root: &str,
    kind: &str,
) -> ToolResult<String> {
    crate::artifact_next::next_with_cursor(workspace, conn, root, kind)
}

fn scale_count(scale: &str) -> &str {
    let trimmed = scale.trim();
    if trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        trimmed
    } else {
        ""
    }
}

fn kind_or_default(kind: &str) -> &str {
    let trimmed = kind.trim();
    if trimmed.is_empty() {
        "artifact"
    } else {
        trimmed
    }
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

fn kind_mismatch(kind: &str, manifest: &str) -> bool {
    match kind.trim().to_ascii_lowercase().as_str() {
        "story" => !manifest.contains("NarrativeManuscript"),
        "cookbook" => !manifest.contains("Cookbook"),
        _ => false,
    }
}

fn readiness_report(kind: &str, report: &str) -> String {
    let converted = report.replace("document audit", "artifact audit");
    if !content_kind(kind) || !converted.starts_with("artifact audit passed") {
        return converted;
    }
    converted.replace(
        "next_action=record document-structure evidence",
        "readiness=content-bearing\nnext_action=record document-structure and artifact-readiness evidence",
    )
}

fn content_kind(kind: &str) -> bool {
    matches!(
        kind.trim().to_ascii_lowercase().as_str(),
        "cookbook" | "story"
    )
}

#[allow(clippy::manual_unwrap_or_default)]
fn optional_manifest(root: &Path) -> String {
    match fs::read_to_string(root.join(".lkj-doc-graph.md")) {
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
