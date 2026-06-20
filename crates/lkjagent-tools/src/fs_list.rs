use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;

pub fn list(
    workspace: &Path,
    path: &str,
    depth: usize,
    kind: &str,
    limit: usize,
) -> ToolResult<String> {
    if depth > 12 || limit == 0 || limit > 1000 {
        return Err(ToolError::invalid("depth must be <=12 and limit 1..1000"));
    }
    let root = workspace_path(workspace, path)?;
    let mut rows = Vec::new();
    walk(workspace, &root, 0, depth, kind, &mut rows)?;
    rows.sort();
    rows.truncate(limit);
    Ok(rows.join("\n"))
}

fn walk(
    workspace: &Path,
    path: &Path,
    level: usize,
    max_depth: usize,
    kind: &str,
    rows: &mut Vec<String>,
) -> ToolResult<()> {
    let meta = fs::metadata(path)?;
    if include(kind, meta.is_dir()) {
        rows.push(row(workspace, path, &meta)?);
    }
    if !meta.is_dir() || level >= max_depth {
        return Ok(());
    }
    let mut children = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();
    children.sort();
    for child in children {
        walk(
            workspace,
            &child,
            level.saturating_add(1),
            max_depth,
            kind,
            rows,
        )?;
    }
    Ok(())
}

fn include(kind: &str, is_dir: bool) -> bool {
    matches!(kind, "all") || (kind == "dir" && is_dir) || (kind == "file" && !is_dir)
}

fn row(workspace: &Path, path: &Path, meta: &fs::Metadata) -> ToolResult<String> {
    let rel = path
        .strip_prefix(workspace)
        .map_err(|error| ToolError::Io(error.to_string()))?;
    let rel = if rel.as_os_str().is_empty() {
        "."
    } else {
        rel.to_str().unwrap_or(".")
    };
    let kind = if meta.is_dir() { "dir" } else { "file" };
    let lines = if meta.is_file() {
        line_count(path).unwrap_or(0)
    } else {
        0
    };
    Ok(format!("{kind} {rel} bytes={} lines={lines}", meta.len()))
}

fn line_count(path: &Path) -> Option<usize> {
    let text = fs::read_to_string(path).ok()?;
    Some(if text.is_empty() {
        0
    } else {
        text.lines().count()
    })
}
