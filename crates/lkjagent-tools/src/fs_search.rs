use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;

pub fn search(
    workspace: &Path,
    path: &str,
    query: &str,
    include: &str,
    case_mode: &str,
    context: usize,
    limit: usize,
) -> ToolResult<String> {
    if query.trim().is_empty() || limit == 0 || limit > 500 {
        return Err(ToolError::invalid(
            "query is required and limit must be 1..500",
        ));
    }
    let root = workspace_path(workspace, path)?;
    let mut files = Vec::new();
    collect_files(&root, &mut files)?;
    files.sort();
    let needle = normalize(query, case_mode);
    let mut rows = Vec::new();
    for file in files
        .into_iter()
        .filter(|file| include_match(file, include))
    {
        scan_file(
            workspace, &file, &needle, case_mode, context, limit, &mut rows,
        )?;
        if rows.len() >= limit {
            break;
        }
    }
    Ok(rows.join("\n"))
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> ToolResult<()> {
    let meta = fs::metadata(path)?;
    if meta.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }
    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        collect_files(&entry.path(), files)?;
    }
    Ok(())
}

fn scan_file(
    workspace: &Path,
    file: &Path,
    needle: &str,
    case_mode: &str,
    context: usize,
    limit: usize,
    rows: &mut Vec<String>,
) -> ToolResult<()> {
    let Ok(text) = fs::read_to_string(file) else {
        return Ok(());
    };
    let lines = text.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        if normalize(line, case_mode).contains(needle) {
            push_match(workspace, file, index, &lines, context, rows)?;
            if rows.len() >= limit {
                break;
            }
        }
    }
    Ok(())
}

fn push_match(
    workspace: &Path,
    file: &Path,
    index: usize,
    lines: &[&str],
    context: usize,
    rows: &mut Vec<String>,
) -> ToolResult<()> {
    let start = index.saturating_sub(context);
    let end = usize::min(index.saturating_add(context).saturating_add(1), lines.len());
    let rel = file
        .strip_prefix(workspace)
        .map_err(|error| ToolError::Io(error.to_string()))?;
    let path = rel.to_string_lossy();
    for (line_index, line) in lines.iter().enumerate().take(end).skip(start) {
        rows.push(format!(
            "{}:{}: {}",
            path,
            line_index.saturating_add(1),
            line
        ));
    }
    Ok(())
}

fn normalize(value: &str, case_mode: &str) -> String {
    if case_mode == "sensitive" {
        value.to_string()
    } else {
        value.to_ascii_lowercase()
    }
}

fn include_match(path: &Path, include: &str) -> bool {
    include.trim().is_empty() || path.to_string_lossy().contains(include)
}
