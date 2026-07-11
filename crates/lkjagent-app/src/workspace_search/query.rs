use std::fs;
use std::path::Path;

use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::workspace_search::{search, SearchHit};
use rusqlite::Connection;

use super::Request;

const EXCERPT_BYTES: usize = 240;

pub fn run(conn: &Connection, workspace: &Path, request: &Request) -> Result<String, String> {
    let hits = search(conn, &request.query, &request.filter(), request.mode()?, 50)
        .map_err(|error| error.to_string())?;
    let mut accepted = Vec::new();
    let mut excluded = 0;
    for hit in hits {
        if current(workspace, &hit)? {
            accepted.push(render(&hit, &request.query));
        } else {
            excluded += 1;
        }
    }
    if accepted.is_empty() {
        return Ok(no_matches(excluded));
    }
    if excluded > 0 {
        accepted.push(format!("excluded_drifted={excluded}"));
    }
    Ok(accepted.join("\n"))
}

fn current(workspace: &Path, hit: &SearchHit) -> Result<bool, String> {
    let path = workspace.join(&hit.chunk.path);
    let Ok(text) = fs::read_to_string(path) else {
        return Ok(false);
    };
    let fingerprint = record_fingerprint(&text).map_err(|error| error.message)?;
    Ok(fingerprint == hit.chunk.revision_fingerprint)
}

fn render(hit: &SearchHit, query: &str) -> String {
    format!(
        "path={} document={} field={} kind={} state={} score={:.6}\n{}",
        hit.chunk.path,
        hit.chunk.document_id,
        hit.chunk.field,
        hit.chunk.kind,
        hit.chunk.state,
        hit.score,
        excerpt(&hit.chunk.content, query),
    )
}

fn no_matches(excluded: usize) -> String {
    if excluded == 0 {
        "no matches".to_string()
    } else {
        format!("no matches\nexcluded_drifted={excluded}")
    }
}

fn excerpt(content: &str, query: &str) -> String {
    let needle = query.split_whitespace().next().unwrap_or_default();
    let index = content.find(needle).unwrap_or(0);
    let start = floor_boundary(content, index.saturating_sub(EXCERPT_BYTES / 3));
    let end = ceiling_boundary(content, (start + EXCERPT_BYTES).min(content.len()));
    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < content.len() { "..." } else { "" };
    format!("{prefix}{}{suffix}", &content[start..end])
}

fn floor_boundary(text: &str, mut index: usize) -> usize {
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceiling_boundary(text: &str, mut index: usize) -> usize {
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}
