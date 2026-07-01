mod parse;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::{workspace_path, write};
use parse::{parse_files, BatchFile};

const MAX_FILE_BYTES: usize = crate::fs::MAX_INLINE_FILE_BYTES;
const MAX_TOTAL_BYTES: usize = 6_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchFileInfo {
    pub path: String,
    pub bytes: usize,
}

pub fn mkdir(workspace: &Path, path: &str) -> ToolResult<String> {
    reject_file_like_directory(path)?;
    let full = workspace_path(workspace, path)?;
    fs::create_dir_all(full)?;
    Ok(format!("directory created\npath={path}"))
}

fn reject_file_like_directory(path: &str) -> ToolResult<()> {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".md") || lower.ends_with(".toml") {
        return Err(ToolError::invalid(format!(
            "directory path looks like a file: {path}; use fs.write or fs.batch_write"
        )));
    }
    Ok(())
}

pub fn batch_write(workspace: &Path, files: &str, max_files: usize) -> ToolResult<String> {
    let parsed = parse_files(files)?;
    validate_batch(workspace, &parsed.files, max_files)?;
    let mut written = Vec::new();
    for file in parsed.files {
        write(workspace, &file.path, &file.content)?;
        written.push(format!("{} bytes={}", file.path, file.content.len()));
    }
    Ok(format!(
        "files_written={}\ninput_format={}\n{}",
        written.len(),
        parsed.format.as_str(),
        written.join("\n")
    ))
}

pub fn paths(files: &str) -> ToolResult<Vec<String>> {
    Ok(file_infos(files)?
        .into_iter()
        .map(|file| file.path)
        .collect())
}

pub fn file_infos(files: &str) -> ToolResult<Vec<BatchFileInfo>> {
    Ok(parse_files(files)?
        .files
        .into_iter()
        .map(|file| BatchFileInfo {
            path: file.path,
            bytes: file.content.len(),
        })
        .collect())
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
        crate::placeholder::reject_for_path(&file.path, &file.content)?;
        let bytes = file.content.len();
        if bytes > MAX_FILE_BYTES {
            return Err(ToolError::invalid(format!(
                "payload too large for fs.batch_write file: {} bytes={bytes} max={MAX_FILE_BYTES}",
                file.path
            )));
        }
        total = total.saturating_add(bytes);
        if total > MAX_TOTAL_BYTES {
            return Err(ToolError::invalid(format!(
                "payload too large for fs.batch_write batch: bytes={total} max={MAX_TOTAL_BYTES}"
            )));
        }
    }
    Ok(())
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
