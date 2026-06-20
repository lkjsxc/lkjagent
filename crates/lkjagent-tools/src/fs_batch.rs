use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::{workspace_path, write};

#[derive(Debug, Clone, PartialEq, Eq)]
struct BatchFile {
    path: String,
    content: String,
}

pub fn mkdir(workspace: &Path, path: &str) -> ToolResult<String> {
    let full = workspace_path(workspace, path)?;
    fs::create_dir_all(full)?;
    Ok(format!("directory created\npath={path}"))
}

pub fn batch_write(workspace: &Path, files: &str, max_files: usize) -> ToolResult<String> {
    let parsed = parse_files(files)?;
    if parsed.len() > max_files {
        return Err(ToolError::invalid(format!(
            "too many files; max={max_files}"
        )));
    }
    reject_duplicates(&parsed)?;
    let mut written = Vec::new();
    for file in parsed {
        workspace_path(workspace, &file.path)?;
        write(workspace, &file.path, &file.content)?;
        written.push(format!("{} bytes={}", file.path, file.content.len()));
    }
    Ok(format!(
        "files_written={}\n{}",
        written.len(),
        written.join("\n")
    ))
}

fn parse_files(input: &str) -> ToolResult<Vec<BatchFile>> {
    let mut out = Vec::new();
    for block in input.split("-- lkjagent-next-file --") {
        let Some(file) = parse_block(block)? else {
            continue;
        };
        out.push(file);
    }
    if out.is_empty() {
        return Err(ToolError::invalid(
            "files must contain at least one file block",
        ));
    }
    Ok(out)
}

fn parse_block(block: &str) -> ToolResult<Option<BatchFile>> {
    let trimmed = block.trim_matches('\n');
    if trimmed.trim().is_empty() {
        return Ok(None);
    }
    let Some(rest) = trimmed.strip_prefix("path: ") else {
        return Err(ToolError::invalid("each block must start with path: "));
    };
    let Some((path, content)) = rest.split_once("\ncontent:\n") else {
        return Err(ToolError::invalid("each block needs content:"));
    };
    if path.trim().is_empty() {
        return Err(ToolError::invalid("path must not be empty"));
    }
    Ok(Some(BatchFile {
        path: path.trim().to_string(),
        content: content.to_string(),
    }))
}

fn reject_duplicates(files: &[BatchFile]) -> ToolResult<()> {
    let mut seen = BTreeSet::new();
    for file in files {
        if !seen.insert(file.path.as_str()) {
            return Err(ToolError::invalid(format!("duplicate path: {}", file.path)));
        }
    }
    Ok(())
}
