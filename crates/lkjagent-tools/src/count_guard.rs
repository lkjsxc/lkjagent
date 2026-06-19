use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

pub fn markdown_count_target(lower: &str) -> Option<usize> {
    if !(lower.contains("markdown") || lower.contains(".md")) || !lower.contains("file") {
        return None;
    }
    numbers(lower).into_iter().max()
}

pub fn verify_markdown_count(workspace: &Path, target: usize) -> ToolResult<()> {
    let mut best: Option<(PathBuf, usize)> = None;
    for candidate in candidates(workspace)? {
        let count = markdown_count(&candidate)?;
        if count == target {
            return Ok(());
        }
        if best
            .as_ref()
            .is_none_or(|(_, current)| distance(count, target) < distance(*current, target))
        {
            best = Some((candidate, count));
        }
    }
    Err(ToolError::invalid(report(workspace, target, best)))
}

fn numbers(text: &str) -> Vec<usize> {
    let mut values = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else {
            save_number(&mut values, &mut current);
        }
    }
    save_number(&mut values, &mut current);
    values
}

fn save_number(values: &mut Vec<usize>, current: &mut String) {
    if current.is_empty() {
        return;
    }
    if let Ok(value) = current.parse() {
        values.push(value);
    }
    current.clear();
}

fn candidates(workspace: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut roots = Vec::new();
    collect(workspace, 0, &mut roots)?;
    Ok(roots)
}

fn collect(path: &Path, depth: usize, roots: &mut Vec<PathBuf>) -> ToolResult<()> {
    if depth > 3 || hidden(path) {
        return Ok(());
    }
    if path.join("README.md").exists() {
        roots.push(path.to_path_buf());
    }
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            collect(&child, depth.saturating_add(1), roots)?;
        }
    }
    Ok(())
}

fn markdown_count(path: &Path) -> ToolResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() && !hidden(&child) {
            count = count.saturating_add(markdown_count(&child)?);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}

fn hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

fn distance(count: usize, target: usize) -> usize {
    count.max(target).saturating_sub(count.min(target))
}

fn report(workspace: &Path, target: usize, best: Option<(PathBuf, usize)>) -> String {
    let base = format!("markdown count incomplete: need exactly {target} markdown files");
    match best {
        Some((path, count)) => format!(
            "{base}; next action should repair the count with one compact shell.run script; best={} markdown_files={count}",
            relative(&path, workspace)
        ),
        None => format!("{base}; no README.md candidate found"),
    }
}

fn relative(path: &Path, workspace: &Path) -> String {
    path.strip_prefix(workspace)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}
