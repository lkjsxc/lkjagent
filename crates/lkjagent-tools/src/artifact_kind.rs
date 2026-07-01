use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub(crate) fn audit_kind(
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

pub(crate) fn kind_mismatch(root: &str, kind: &str, catalog: &str) -> bool {
    match kind.trim().to_ascii_lowercase().as_str() {
        "story" if story_root(root) => false,
        "story" => !story_catalog(catalog),
        "cookbook" => !catalog.to_ascii_lowercase().contains("cookbook"),
        _ => false,
    }
}

pub(crate) fn optional_catalog(root: &Path) -> String {
    fs::read_to_string(root.join("catalog.toml")).unwrap_or_else(|_| String::new())
}

fn ledger_kind(conn: &Connection, root: &str) -> ToolResult<Option<String>> {
    let case_id = crate::artifact_ledger_state::case_id(conn)?;
    let Some(row) = lkjagent_store::artifact_ledger::latest_for_case(conn, case_id)? else {
        return Ok(None);
    };
    Ok((row.root == root).then_some(row.kind))
}

fn story_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

fn story_catalog(catalog: &str) -> bool {
    let lower = catalog.to_ascii_lowercase();
    catalog.contains("NarrativeManuscript")
        || lower.contains("kind = \"story\"")
        || lower.contains("story bible")
}
