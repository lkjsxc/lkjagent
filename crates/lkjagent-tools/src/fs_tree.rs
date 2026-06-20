use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;

pub fn tree(workspace: &Path, path: &str, depth: usize, limit: usize) -> ToolResult<String> {
    if depth > 12 || limit == 0 || limit > 1000 {
        return Err(ToolError::invalid("depth must be <=12 and limit 1..1000"));
    }
    let root = workspace_path(workspace, path)?;
    let mut rows = vec![format!("root={path}")];
    walk(workspace, &root, 0, depth, limit, &mut rows)?;
    rows.truncate(limit);
    Ok(rows.join("\n"))
}

fn walk(
    workspace: &Path,
    path: &Path,
    level: usize,
    depth: usize,
    limit: usize,
    rows: &mut Vec<String>,
) -> ToolResult<()> {
    if level >= depth || rows.len() >= limit {
        return Ok(());
    }
    let mut children = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    children.sort();
    for child in children {
        rows.push(row(workspace, &child)?);
        if child.is_dir() {
            walk(
                workspace,
                &child,
                level.saturating_add(1),
                depth,
                limit,
                rows,
            )?;
        }
        if rows.len() >= limit {
            break;
        }
    }
    Ok(())
}

fn row(workspace: &Path, path: &Path) -> ToolResult<String> {
    let meta = fs::metadata(path)?;
    let rel = path
        .strip_prefix(workspace)
        .map_err(|error| ToolError::Io(error.to_string()))?;
    let kind = if meta.is_dir() { "dir" } else { "file" };
    Ok(format!("{kind} {}", rel.to_string_lossy()))
}
