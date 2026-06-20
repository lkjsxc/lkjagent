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

pub fn index(workspace: &Path, path: &str, depth: usize, limit: usize) -> ToolResult<String> {
    if depth > 8 || limit == 0 || limit > 1000 {
        return Err(ToolError::invalid("depth must be <=8 and limit 1..1000"));
    }
    let root = workspace_path(workspace, path)?;
    let mut index = WorkspaceIndex::default();
    collect(&root, &root, 0, depth, &mut index)?;
    let mut rows = vec![
        format!("root={path}"),
        format!("directories={}", index.directories),
        format!("files={}", index.files),
        format!("readmes={}", index.readmes.len()),
    ];
    push_named("manifest", &index.manifests, &mut rows);
    push_named("readme", &index.readmes, &mut rows);
    push_named("top_dir", &index.top_dirs, &mut rows);
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

#[derive(Debug, Default)]
struct WorkspaceIndex {
    directories: usize,
    files: usize,
    manifests: Vec<String>,
    readmes: Vec<String>,
    top_dirs: Vec<String>,
}

fn collect(
    root: &Path,
    path: &Path,
    level: usize,
    depth: usize,
    index: &mut WorkspaceIndex,
) -> ToolResult<()> {
    if level >= depth {
        return Ok(());
    }
    let mut children = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    children.sort();
    for child in children {
        record_path(root, &child, level, index)?;
        if child.is_dir() {
            collect(root, &child, level.saturating_add(1), depth, index)?;
        }
    }
    Ok(())
}

fn record_path(
    root: &Path,
    path: &Path,
    level: usize,
    index: &mut WorkspaceIndex,
) -> ToolResult<()> {
    let rel = path
        .strip_prefix(root)
        .map_err(|error| ToolError::Io(error.to_string()))?
        .to_string_lossy()
        .to_string();
    if path.is_dir() {
        index.directories = index.directories.saturating_add(1);
        if level == 0 {
            index.top_dirs.push(rel);
        }
        return Ok(());
    }
    index.files = index.files.saturating_add(1);
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return Ok(());
    };
    if name == "README.md" {
        index.readmes.push(rel.clone());
    }
    if matches!(
        name,
        "Cargo.toml" | "package.json" | "pyproject.toml" | "go.mod"
    ) {
        index.manifests.push(rel);
    }
    Ok(())
}

fn push_named(name: &str, values: &[String], rows: &mut Vec<String>) {
    for value in values.iter().take(24) {
        rows.push(format!("{name}={value}"));
    }
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
