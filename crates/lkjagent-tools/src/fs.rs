use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRead {
    pub path: String,
    pub start: usize,
    pub count: usize,
    pub total_lines: usize,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditReport {
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchReport {
    pub path: String,
    pub edits: usize,
}

pub fn read(workspace: &Path, path: &str, start: usize, count: usize) -> ToolResult<FileRead> {
    if start == 0 || count == 0 {
        return Err(ToolError::invalid("start and count must be positive"));
    }
    let full_path = workspace_path(workspace, path)?;
    let text = fs::read_to_string(&full_path)?;
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let total_lines = if text.is_empty() { 0 } else { lines.len() };
    let body = lines
        .iter()
        .skip(start.saturating_sub(1))
        .take(count)
        .copied()
        .collect::<String>();
    Ok(FileRead {
        path: path.to_string(),
        start,
        count,
        total_lines,
        body,
    })
}

pub fn read_many(
    workspace: &Path,
    paths: &str,
    start: usize,
    count: usize,
    total: usize,
) -> ToolResult<String> {
    if total == 0 || total > 1000 {
        return Err(ToolError::invalid("total must be 1..1000 lines"));
    }
    let mut remaining = total;
    let mut rows = Vec::new();
    for path in paths.lines().map(str::trim).filter(|path| !path.is_empty()) {
        if remaining == 0 {
            break;
        }
        let limit = count.min(remaining);
        let read = read(workspace, path, start, limit)?;
        remaining = remaining.saturating_sub(read.body.lines().count());
        rows.push(read_observation(&read));
    }
    if rows.is_empty() {
        return Err(ToolError::invalid("paths must name at least one file"));
    }
    Ok(rows.join("\n-- file --\n"))
}

pub fn read_observation(read: &FileRead) -> String {
    let returned = read.body.split_inclusive('\n').count();
    let end = read.start.saturating_add(returned).saturating_sub(1);
    format!(
        "path={} lines={}-{} total={}\n{}",
        read.path, read.start, end, read.total_lines, read.body
    )
}

pub fn write(workspace: &Path, path: &str, content: &str) -> ToolResult<String> {
    let full_path = workspace_path(workspace, path)?;
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&full_path, content)?;
    Ok(format!("path={path}\nbytes={}", content.len()))
}

pub fn edit(workspace: &Path, path: &str, find: &str, replace: &str) -> ToolResult<EditReport> {
    if find.is_empty() {
        return Err(ToolError::invalid("find must not be empty"));
    }
    let full_path = workspace_path(workspace, path)?;
    let text = fs::read_to_string(&full_path)?;
    let matches: Vec<usize> = text.match_indices(find).map(|(index, _)| index).collect();
    if matches.len() != 1 {
        return Err(ToolError::invalid(format!(
            "find matched {} times",
            matches.len()
        )));
    }
    let Some(index) = matches.first().copied() else {
        return Err(ToolError::invalid("find matched 0 times"));
    };
    let line = text[..index].lines().count().saturating_add(1);
    let next = text.replacen(find, replace, 1);
    fs::write(full_path, next)?;
    Ok(EditReport {
        path: path.to_string(),
        line,
    })
}

pub fn patch(workspace: &Path, path: &str, patch: &str) -> ToolResult<PatchReport> {
    let edits = parse_patch(patch)?;
    let full_path = workspace_path(workspace, path)?;
    let original = fs::read_to_string(&full_path)?;
    let mut text = original.clone();
    for (find, _) in &edits {
        let matches = text.match_indices(find).count();
        if matches != 1 {
            return Err(ToolError::invalid(format!(
                "patch find matched {matches} times"
            )));
        }
    }
    for (find, replace) in &edits {
        text = text.replacen(find, replace, 1);
    }
    if text == original {
        return Err(ToolError::invalid("patch made no changes"));
    }
    fs::write(full_path, text)?;
    Ok(PatchReport {
        path: path.to_string(),
        edits: edits.len(),
    })
}

pub fn edit_observation(report: &EditReport) -> String {
    format!("path={}\nline={}", report.path, report.line)
}

pub fn patch_observation(report: &PatchReport) -> String {
    format!("path={}\nedits={}", report.path, report.edits)
}

fn parse_patch(patch: &str) -> ToolResult<Vec<(String, String)>> {
    let mut edits = Vec::new();
    for block in patch.split("-- lkjagent-next-edit --") {
        let trimmed = block.trim_matches('\n');
        if trimmed.trim().is_empty() {
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("find:\n") else {
            return Err(ToolError::invalid("each patch block must start with find:"));
        };
        let Some((find, replace)) = rest.split_once("\nreplace:\n") else {
            return Err(ToolError::invalid("each patch block needs replace:"));
        };
        if find.is_empty() {
            return Err(ToolError::invalid("patch find must not be empty"));
        }
        edits.push((find.to_string(), replace.to_string()));
    }
    if edits.is_empty() || edits.len() > 20 {
        return Err(ToolError::invalid("patch edits must be 1..20"));
    }
    Ok(edits)
}

pub(crate) fn workspace_path(workspace: &Path, path: &str) -> ToolResult<PathBuf> {
    if path.trim().is_empty() {
        return Err(ToolError::invalid("path must not be empty"));
    }
    let relative = Path::new(path);
    if relative.is_absolute() || escapes_workspace(relative) {
        return Err(ToolError::invalid("path must stay inside the workspace"));
    }
    Ok(workspace.join(relative))
}

fn escapes_workspace(path: &Path) -> bool {
    path.components()
        .any(|part| !matches!(part, Component::Normal(_) | Component::CurDir))
}
