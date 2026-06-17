use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};
use crate::observe::estimate_tokens;

pub fn save(
    conn: &mut Connection,
    kind: &str,
    title: &str,
    tags: &str,
    content: &str,
    now: &str,
) -> ToolResult<String> {
    let kind = parse_kind(kind)?;
    if title.lines().count() != 1 || title.trim().is_empty() {
        return Err(ToolError::invalid("title must be one non-empty line"));
    }
    if content.trim().is_empty() {
        return Err(ToolError::invalid("content must not be empty"));
    }
    let tokens = estimate_tokens(content) as i64;
    let id = lkjagent_store::memory::save(conn, kind, title, tags, content, tokens, now)?;
    Ok(format!("memory_id={id}"))
}

pub fn find(conn: &Connection, query: &str, limit: usize) -> ToolResult<String> {
    if query.trim().is_empty() {
        return Err(ToolError::invalid("query must not be empty"));
    }
    if limit == 0 {
        return Err(ToolError::invalid("limit must be positive"));
    }
    let rows = lkjagent_store::memory::find(conn, query, limit as i64)?;
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "id={} kind={} title={} snippet={}",
            row.id,
            row.kind,
            row.title,
            snippet(&row.content)
        ));
    }
    if lines.is_empty() {
        Ok("no memory results".to_string())
    } else {
        Ok(lines.join("\n"))
    }
}

fn parse_kind(kind: &str) -> ToolResult<lkjagent_store::memory::MemoryKind> {
    match kind {
        "lesson" => Ok(lkjagent_store::memory::MemoryKind::Lesson),
        "fact" => Ok(lkjagent_store::memory::MemoryKind::Fact),
        "task-summary" => Ok(lkjagent_store::memory::MemoryKind::TaskSummary),
        "incident" => Ok(lkjagent_store::memory::MemoryKind::Incident),
        other => Err(ToolError::invalid(format!("unknown memory kind: {other}"))),
    }
}

fn snippet(content: &str) -> String {
    content.chars().take(120).collect()
}
