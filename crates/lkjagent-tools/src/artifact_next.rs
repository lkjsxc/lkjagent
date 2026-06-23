use std::fs;
use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;
use rusqlite::Connection;

pub fn next(workspace: &Path, root: &str, kind: &str) -> ToolResult<String> {
    crate::artifact::reject_empty_root(root)?;
    let full = workspace_path(workspace, root)?;
    let kind = resolved_kind(kind, &full);
    if !full.exists() {
        return Ok(format!(
            "artifact next batch\nroot={root}\nkind={kind}\nmissing=root\nnext_action=artifact.apply\nvalid_example:\n{}",
            artifact_apply_example(root, &kind)
        ));
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            return Ok(report.block_message(root));
        }
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        return audit_response(root, &kind, "missing=0");
    }
    let selected = weak.into_iter().take(3).collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write(root, &kind, &selected);
    Ok(batch_response(root, &kind, &selected, &valid_example))
}

pub fn next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
) -> ToolResult<String> {
    crate::artifact::reject_empty_root(root)?;
    let full = workspace_path(workspace, root)?;
    let kind = resolved_kind(kind, &full);
    if !full.exists() {
        lkjagent_store::state::delete(conn, &cursor_key(root))?;
        return Ok(format!(
            "artifact next batch\nroot={root}\nkind={kind}\nmissing=root\nnext_action=artifact.apply\nvalid_example:\n{}",
            artifact_apply_example(root, &kind)
        ));
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            lkjagent_store::state::delete(conn, &cursor_key(root))?;
            return Ok(report.block_message(root));
        }
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        lkjagent_store::state::delete(conn, &cursor_key(root))?;
        return audit_response(root, &kind, "missing=0");
    }
    let start = next_start(conn, root, &weak)?;
    if start >= weak.len() {
        return audit_response(root, &kind, &format!("missing={}", weak.len()));
    }
    let weak_count = weak.len();
    let selected = weak.into_iter().skip(start).take(3).collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write(root, &kind, &selected);
    crate::artifact_cursor_support::record_next_batch(
        crate::artifact_cursor_support::NextBatchRecord {
            conn,
            root,
            kind: &kind,
            weak_count,
            selected: &selected,
            valid_example: &valid_example,
            current_index: start.saturating_add(selected.len()),
            now,
        },
    )?;
    if let Some(last) = selected.last() {
        lkjagent_store::state::set(conn, &cursor_key(root), last)?;
    }
    Ok(batch_response(root, &kind, &selected, &valid_example))
}

fn resolved_kind(kind: &str, root: &Path) -> String {
    let trimmed = kind.trim();
    if !trimmed.is_empty() {
        return trimmed.to_string();
    }
    let text = optional_catalog(root);
    if text.contains("Cookbook") {
        "cookbook".to_string()
    } else if text.contains("NarrativeManuscript") {
        "story".to_string()
    } else {
        "artifact".to_string()
    }
}

fn required_sections(kind: &str) -> &'static str {
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => "- title\n- purpose\n- ingredients or concept\n- method or procedure\n- timing, signals, and fixes\n- verification notes",
        "story" => "- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes",
        _ => "- title\n- purpose\n- concrete content\n- verification notes",
    }
}

#[allow(clippy::manual_unwrap_or_default)]
fn optional_catalog(root: &Path) -> String {
    match fs::read_to_string(root.join("catalog.toml")) {
        Ok(text) => text,
        Err(_) => String::new(),
    }
}

fn artifact_apply_example(root: &str, kind: &str) -> String {
    format!("<act>\n<tool>artifact.apply</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</act>")
}

fn artifact_audit_example(root: &str, kind: &str) -> String {
    format!("<act>\n<tool>artifact.audit</tool>\n<root>{root}</root>\n<kind>{kind}</kind>\n</act>")
}

fn batch_response(root: &str, kind: &str, selected: &[String], valid_example: &str) -> String {
    format!(
        "artifact next batch\nroot={root}\nkind={kind}\nmissing={}\nnext_paths:\n{}\nrequired_sections:\n{}\nnext_action=fs.batch_write\nvalid_example:\n{}",
        selected.len(),
        selected
            .iter()
            .map(|path| format!("- {path}"))
            .collect::<Vec<_>>()
            .join("\n"),
        required_sections(kind),
        valid_example
    )
}

fn audit_response(root: &str, kind: &str, missing: &str) -> ToolResult<String> {
    Ok(format!(
        "artifact next batch\nroot={root}\nkind={kind}\n{missing}\nnext_action=artifact.audit\nvalid_example:\n{}",
        artifact_audit_example(root, kind)
    ))
}

fn next_start(conn: &Connection, root: &str, weak: &[String]) -> ToolResult<usize> {
    let Some(cursor) = lkjagent_store::state::get(conn, &cursor_key(root))? else {
        return Ok(0);
    };
    Ok(weak
        .iter()
        .position(|path| path > &cursor)
        .unwrap_or(weak.len()))
}

fn cursor_key(root: &str) -> String {
    format!("artifact.next cursor {root}")
}
