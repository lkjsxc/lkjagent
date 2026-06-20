use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;

pub fn summary(workspace: &Path, path: &str, depth: usize, limit: usize) -> ToolResult<String> {
    if depth > 8 || limit == 0 || limit > 1000 {
        return Err(ToolError::invalid("depth must be <=8 and limit 1..1000"));
    }
    let root = workspace_path(workspace, path)?;
    let mut rows = Vec::new();
    rows.push(format!("root={path}"));
    if root.join("Cargo.toml").exists() {
        rows.push("cargo_workspace=present".to_string());
        crates(&root, &mut rows)?;
    }
    if root.join("docs").is_dir() {
        rows.push("docs_root=docs".to_string());
    }
    tree(&root, &root, 0, depth, limit, &mut rows)?;
    rows.truncate(limit);
    Ok(rows.join("\n"))
}

fn crates(root: &Path, rows: &mut Vec<String>) -> ToolResult<()> {
    let crates_dir = root.join("crates");
    if !crates_dir.is_dir() {
        return Ok(());
    }
    let mut names = fs::read_dir(crates_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("Cargo.toml").exists())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();
    names.sort();
    rows.push(format!("crates={}", names.join(", ")));
    Ok(())
}

fn tree(
    root: &Path,
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
        let rel = child
            .strip_prefix(root)
            .map_err(|error| ToolError::Io(error.to_string()))?;
        let marker = if child.is_dir() { "dir" } else { "file" };
        rows.push(format!("{marker} {}", rel.to_string_lossy()));
        if child.is_dir() {
            tree(root, &child, level.saturating_add(1), depth, limit, rows)?;
        }
        if rows.len() >= limit {
            break;
        }
    }
    Ok(())
}
