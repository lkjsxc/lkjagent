use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::{workspace_path, write};

const MAX_FILE_BYTES: usize = 65_536;
const MAX_TOTAL_BYTES: usize = 262_144;

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
    validate_batch(workspace, &parsed, max_files)?;
    let mut written = Vec::new();
    for file in parsed {
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

fn validate_batch(workspace: &Path, files: &[BatchFile], max_files: usize) -> ToolResult<()> {
    if files.len() > max_files {
        return Err(ToolError::invalid(format!(
            "too many files; max={max_files}"
        )));
    }
    reject_duplicates(files)?;
    let mut total = 0usize;
    for file in files {
        workspace_path(workspace, &file.path)?;
        crate::placeholder::reject(&file.content)?;
        let bytes = file.content.len();
        if bytes > MAX_FILE_BYTES {
            return Err(ToolError::invalid(format!(
                "file too large: {} bytes={bytes} max={MAX_FILE_BYTES}",
                file.path
            )));
        }
        total = total.saturating_add(bytes);
        if total > MAX_TOTAL_BYTES {
            return Err(ToolError::invalid(format!(
                "batch too large: bytes={total} max={MAX_TOTAL_BYTES}"
            )));
        }
    }
    Ok(())
}

fn parse_block(block: &str) -> ToolResult<Option<BatchFile>> {
    let trimmed = block.trim_start_matches(['\n', '\r', ' ', '\t']);
    if trimmed.trim().is_empty() {
        return Ok(None);
    }
    let Some((header, body)) = trimmed.split_once('\n') else {
        return Err(ToolError::invalid("each block needs content:"));
    };
    let path = parse_path_header(header)?;
    let content = parse_content(body)?;
    Ok(Some(BatchFile {
        path,
        content: content.to_string(),
    }))
}

fn parse_path_header(header: &str) -> ToolResult<String> {
    let trimmed = header.trim();
    let path = trimmed
        .strip_prefix("path:")
        .map(str::trim)
        .or_else(|| xml_path(trimmed))
        .or_else(|| angled_path(trimmed));
    let Some(path) = path.filter(|path| !path.is_empty()) else {
        return Err(ToolError::invalid("each block must start with path: "));
    };
    Ok(path.to_string())
}

fn xml_path(header: &str) -> Option<&str> {
    header
        .strip_prefix("<path>")
        .and_then(|rest| rest.strip_suffix("</path>"))
        .map(str::trim)
}

fn angled_path(header: &str) -> Option<&str> {
    header
        .strip_prefix("<path:")
        .map(|path| path.trim_end_matches('>').trim())
}

fn parse_content(body: &str) -> ToolResult<&str> {
    if let Some(content) = body.strip_prefix("content:\n") {
        return Ok(content);
    }
    if let Some(content) = body.strip_prefix("content:\r\n") {
        return Ok(content);
    }
    Err(ToolError::invalid("each block needs content:"))
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
