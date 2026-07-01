use std::collections::BTreeSet;

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{ToolError, ToolResult};
use crate::fs_batch::BatchFileInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeContract {
    paths: BTreeSet<String>,
    max_files: usize,
    max_file_bytes: usize,
    max_batch_bytes: usize,
}

pub fn validate(
    conn: &Connection,
    decision_id: Option<&str>,
    files: &[BatchFileInfo],
) -> ToolResult<()> {
    let Some(contract) = runtime_contract(conn, decision_id)? else {
        return Ok(());
    };
    for file in files {
        if !contract.paths.contains(&file.path) {
            return Err(ToolError::invalid(format!(
                "fs.batch_write path outside runtime write contract: {}",
                file.path
            )));
        }
    }
    validate_limits(&contract, files)
}

fn runtime_contract(
    conn: &Connection,
    decision_id: Option<&str>,
) -> ToolResult<Option<RuntimeContract>> {
    let Some(id) = decision_id.and_then(|value| value.parse::<i64>().ok()) else {
        return Ok(None);
    };
    let text = conn
        .query_row(
            "SELECT rendered_summary FROM runtime_prompt_frames
             WHERE decision_id = ?1 ORDER BY id DESC LIMIT 1",
            params![id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| ToolError::Store(error.to_string()))?;
    Ok(text.and_then(|value| parse_contract(&value)))
}

fn parse_contract(text: &str) -> Option<RuntimeContract> {
    let block = between(text, "<write-contract>", "</write-contract>")?;
    if between(block, "<tool>", "</tool>")?.trim() != "fs.batch_write" {
        return None;
    }
    let paths = paths(block);
    if paths.is_empty() {
        return None;
    }
    let limits = between(block, "<limits>", "</limits>").unwrap_or_default();
    Some(RuntimeContract {
        paths,
        max_files: limit(limits, "max_files").unwrap_or(usize::MAX),
        max_file_bytes: limit(limits, "max_file_bytes").unwrap_or(usize::MAX),
        max_batch_bytes: limit(limits, "max_batch_bytes").unwrap_or(usize::MAX),
    })
}

fn validate_limits(contract: &RuntimeContract, files: &[BatchFileInfo]) -> ToolResult<()> {
    if files.len() > contract.max_files {
        return Err(ToolError::invalid(format!(
            "fs.batch_write exceeds runtime write contract max_files: count={} max={}",
            files.len(),
            contract.max_files
        )));
    }
    let mut total = 0usize;
    for file in files {
        if file.bytes > contract.max_file_bytes {
            return Err(ToolError::invalid(format!(
                "fs.batch_write exceeds runtime write contract max_file_bytes: path={} bytes={} max={}",
                file.path, file.bytes, contract.max_file_bytes
            )));
        }
        total = total.saturating_add(file.bytes);
        if total > contract.max_batch_bytes {
            return Err(ToolError::invalid(format!(
                "fs.batch_write exceeds runtime write contract max_batch_bytes: bytes={total} max={}",
                contract.max_batch_bytes
            )));
        }
    }
    Ok(())
}

fn paths(block: &str) -> BTreeSet<String> {
    between(block, "<paths>", "</paths>")
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- "))
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect()
}

fn limit(limits: &str, name: &str) -> Option<usize> {
    limits
        .split_whitespace()
        .find_map(|part| part.strip_prefix(&format!("{name}=")))
        .and_then(|value| value.parse().ok())
}

fn between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let after = text.split_once(start)?.1;
    Some(after.split_once(end)?.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_write_contract_paths_and_limits() -> Result<(), String> {
        let text = "<write-contract>\n<tool>fs.batch_write</tool>\n<paths>\n- a.md\n</paths>\n<limits>max_files=1 max_file_bytes=7 max_batch_bytes=9</limits>\n</write-contract>";
        let Some(contract) = parse_contract(text) else {
            return Err("missing contract".to_string());
        };
        assert!(contract.paths.contains("a.md"));
        assert_eq!(contract.max_files, 1);
        assert_eq!(contract.max_file_bytes, 7);
        assert_eq!(contract.max_batch_bytes, 9);
        Ok(())
    }
}
