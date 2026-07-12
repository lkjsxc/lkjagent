use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{EffectError, EffectResult};

pub use crate::workspace_capability::{
    DirectoryEntry, DirectoryListing, EntryKind, FilePage, NumberedLine, OpenedWorkspace,
    SearchHit, SearchResult, LINE_BYTES, LIST_ENTRIES, READ_LINES, SEARCH_HITS, WORKSPACE_BYTES,
    WORKSPACE_FILES,
};

pub const TREE_ENTRIES: usize = 150;

pub fn resolve(root: &Path, path: &str) -> EffectResult<PathBuf> {
    let relative = Path::new(path);
    if path.trim().is_empty()
        || relative.is_absolute()
        || relative.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(EffectError::Path("path must stay inside workspace".into()));
    }
    let root = root.canonicalize()?;
    let candidate = root.join(relative);
    let mut check = candidate.clone();
    while !check.exists() {
        check = check
            .parent()
            .ok_or_else(|| EffectError::Path("no existing parent".into()))?
            .to_path_buf();
    }
    if !check.canonicalize()?.starts_with(&root) {
        return Err(EffectError::Path("path resolves outside workspace".into()));
    }
    Ok(candidate)
}

pub fn read(root: &Path, path: &str, offset: usize, count: usize) -> EffectResult<String> {
    let text = fs::read_to_string(resolve(root, path)?)?;
    let lines = text.lines().collect::<Vec<_>>();
    let count = if count == 0 {
        READ_LINES
    } else {
        count.min(READ_LINES)
    };
    Ok(format!(
        "path={path} offset={offset} count={count} total={} truncated={}\n{}",
        lines.len(),
        offset.saturating_add(count) < lines.len(),
        lines
            .into_iter()
            .skip(offset)
            .take(count)
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

pub fn write(root: &Path, path: &str, content: &str) -> EffectResult<String> {
    let full = resolve(root, path)?;
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(full, content)?;
    Ok(format!("path={path}\nbytes={}", content.len()))
}

pub fn list(root: &Path, path: &str, depth: usize) -> EffectResult<String> {
    let mut rows = Vec::new();
    legacy_walk(root, &resolve(root, path)?, 0, depth, &mut rows)?;
    rows.sort();
    rows.truncate(LIST_ENTRIES);
    Ok(rows.join("\n"))
}

pub fn tree(root: &Path, path: &str, depth: usize) -> EffectResult<String> {
    list(root, path, depth)
}

pub fn search(root: &Path, path: &str, query: &str) -> EffectResult<String> {
    let mut rows = Vec::new();
    legacy_search(root, &resolve(root, path)?, query, &mut rows)?;
    rows.truncate(SEARCH_HITS);
    Ok(rows.join("\n"))
}

fn legacy_walk(
    root: &Path,
    path: &Path,
    level: usize,
    depth: usize,
    rows: &mut Vec<String>,
) -> EffectResult<()> {
    let metadata = fs::metadata(path)?;
    let relative = path
        .strip_prefix(root)
        .map_err(|error| EffectError::Io(error.to_string()))?;
    let display = if relative.as_os_str().is_empty() {
        ".".into()
    } else {
        relative.display().to_string()
    };
    rows.push(format!(
        "{} {display}",
        if metadata.is_dir() { "dir" } else { "file" }
    ));
    if metadata.is_dir() && level < depth {
        for entry in fs::read_dir(path)?.filter_map(Result::ok) {
            legacy_walk(root, &entry.path(), level + 1, depth, rows)?;
        }
    }
    Ok(())
}

fn legacy_search(
    root: &Path,
    path: &Path,
    query: &str,
    rows: &mut Vec<String>,
) -> EffectResult<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)?.filter_map(Result::ok) {
            legacy_search(root, &entry.path(), query, rows)?;
        }
    } else if let Ok(text) = fs::read_to_string(path) {
        let relative = path
            .strip_prefix(root)
            .map_err(|error| EffectError::Io(error.to_string()))?;
        for (line, text) in text.lines().enumerate() {
            if text
                .to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
            {
                rows.push(format!("{}:{}: {text}", relative.display(), line + 1));
            }
        }
    }
    Ok(())
}
