use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{EffectError, EffectResult};

pub const READ_LINES: usize = 200;
pub const LIST_ENTRIES: usize = 200;
pub const TREE_ENTRIES: usize = 150;
pub const SEARCH_HITS: usize = 50;

pub fn resolve(root: &Path, path: &str) -> EffectResult<PathBuf> {
    if path.trim().is_empty() {
        return Err(EffectError::Path("path must not be empty".to_string()));
    }
    let relative = Path::new(path);
    if relative.is_absolute() || escapes(relative) {
        return Err(EffectError::Path(
            "path must stay inside workspace".to_string(),
        ));
    }
    let root = root.canonicalize()?;
    let candidate = root.join(relative);
    guard_existing_parent(&root, &candidate)?;
    Ok(candidate)
}

pub fn read(root: &Path, path: &str, offset: usize, count: usize) -> EffectResult<String> {
    let count = if count == 0 {
        READ_LINES
    } else {
        count.min(READ_LINES)
    };
    let text = fs::read_to_string(resolve(root, path)?)?;
    let lines = text.lines().collect::<Vec<_>>();
    let body = lines
        .iter()
        .skip(offset)
        .take(count)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!(
        "path={path} offset={offset} count={count} total={} truncated={}\n{body}",
        lines.len(),
        offset.saturating_add(count) < lines.len()
    ))
}

pub fn write(root: &Path, path: &str, content: &str) -> EffectResult<String> {
    let full = resolve(root, path)?;
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&full, content)?;
    Ok(format!("path={path}\nbytes={}", content.len()))
}

pub fn list(root: &Path, path: &str, depth: usize) -> EffectResult<String> {
    let base = resolve(root, path)?;
    let mut rows = Vec::new();
    walk(root, &base, 0, depth, LIST_ENTRIES, &mut rows)?;
    rows.sort();
    rows.truncate(LIST_ENTRIES);
    Ok(rows.join("\n"))
}

pub fn tree(root: &Path, path: &str, depth: usize) -> EffectResult<String> {
    let base = resolve(root, path)?;
    let mut rows = Vec::new();
    walk(root, &base, 0, depth, TREE_ENTRIES, &mut rows)?;
    rows.truncate(TREE_ENTRIES);
    Ok(rows.join("\n"))
}

pub fn search(root: &Path, path: &str, query: &str) -> EffectResult<String> {
    let base = resolve(root, path)?;
    let needle = query.to_ascii_lowercase();
    let mut rows = Vec::new();
    search_path(root, &base, &needle, &mut rows)?;
    rows.truncate(SEARCH_HITS);
    Ok(rows.join("\n"))
}

fn escapes(path: &Path) -> bool {
    path.components()
        .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
}

fn guard_existing_parent(root: &Path, candidate: &Path) -> EffectResult<()> {
    let check = if candidate.exists() {
        candidate.canonicalize()?
    } else {
        nearest_existing_parent(candidate, root)?.canonicalize()?
    };
    if check.starts_with(root) {
        Ok(())
    } else {
        Err(EffectError::Path(
            "path resolves outside workspace".to_string(),
        ))
    }
}

fn nearest_existing_parent(candidate: &Path, root: &Path) -> EffectResult<PathBuf> {
    let mut current = candidate
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| root.to_path_buf());
    while !current.exists() {
        let Some(parent) = current.parent().map(Path::to_path_buf) else {
            return Err(EffectError::Path("no existing parent".to_string()));
        };
        current = parent;
    }
    Ok(current)
}

fn walk(
    root: &Path,
    path: &Path,
    level: usize,
    max_depth: usize,
    limit: usize,
    rows: &mut Vec<String>,
) -> EffectResult<()> {
    if rows.len() >= limit {
        return Ok(());
    }
    let meta = fs::metadata(path)?;
    rows.push(row(root, path, meta.is_dir())?);
    if !meta.is_dir() || level >= max_depth {
        return Ok(());
    }
    let mut children = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    children.sort_by_key(|entry| entry.path());
    for child in children {
        walk(root, &child.path(), level + 1, max_depth, limit, rows)?;
    }
    Ok(())
}

fn row(root: &Path, path: &Path, is_dir: bool) -> EffectResult<String> {
    let rel = path
        .strip_prefix(root)
        .map_err(|error| EffectError::Io(error.to_string()))?;
    let label = if rel.as_os_str().is_empty() {
        ".".into()
    } else {
        rel.to_string_lossy()
    };
    Ok(format!("{} {label}", if is_dir { "dir" } else { "file" }))
}

fn search_path(root: &Path, path: &Path, needle: &str, rows: &mut Vec<String>) -> EffectResult<()> {
    if rows.len() >= SEARCH_HITS {
        return Ok(());
    }
    let meta = fs::metadata(path)?;
    if meta.is_dir() {
        for entry in fs::read_dir(path)?.filter_map(Result::ok) {
            search_path(root, &entry.path(), needle, rows)?;
        }
        return Ok(());
    }
    let Ok(text) = fs::read_to_string(path) else {
        return Ok(());
    };
    let rel = path
        .strip_prefix(root)
        .map_err(|error| EffectError::Io(error.to_string()))?;
    for (index, line) in text.lines().enumerate() {
        if line.to_ascii_lowercase().contains(needle) {
            rows.push(format!("{}:{}: {line}", rel.to_string_lossy(), index + 1));
        }
    }
    Ok(())
}
