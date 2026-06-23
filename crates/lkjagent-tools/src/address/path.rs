use std::path::{Path, PathBuf};

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn root_looks_like_markdown_file(path: &str) -> bool {
    Path::new(path.trim())
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".md"))
}

pub fn nearest_catalog_root(workspace: &Path, full: &Path) -> Option<PathBuf> {
    let mut current = full.parent();
    while let Some(dir) = current {
        if !dir.starts_with(workspace) {
            return None;
        }
        if dir.join("catalog.toml").is_file() {
            return Some(dir.to_path_buf());
        }
        if dir == workspace {
            return None;
        }
        current = dir.parent();
    }
    None
}

pub fn clean_requested_path(path: Option<&str>) -> Option<String> {
    path.map(str::trim)
        .filter(|path| !path.is_empty())
        .map(str::to_string)
}

pub fn reject_escaping_relative(path: &str) -> ToolResult<()> {
    workspace_path(Path::new("."), path).map(|_| ())
}

pub fn path_name_ends_md(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".md"))
}

pub fn parent_path(path: &str) -> String {
    Path::new(path)
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .map(|parent| parent.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string())
}

pub fn relative(workspace: &Path, path: &Path) -> Option<String> {
    path.strip_prefix(workspace)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .map(|relative| relative.to_string_lossy().to_string())
}

pub fn kind_or_default(kind: &str) -> String {
    let trimmed = kind.trim();
    if trimmed.is_empty() {
        "artifact".to_string()
    } else {
        trimmed.to_string()
    }
}
