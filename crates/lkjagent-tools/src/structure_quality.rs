use std::fs;
use std::path::Path;

use crate::error::ToolResult;

const WEAK_PHRASES: &[&str] = &[
    "this file is a scaffold",
    "this file records the",
    "keep this file semantic and linked from its local readme",
    "record concrete facts, decisions, and verification evidence",
    "concrete record for",
    "todo: replace",
    "lorem ipsum",
];

pub(crate) fn is_weak_markdown_file(path: &Path) -> ToolResult<bool> {
    if path.extension().is_none_or(|extension| extension != "md") {
        return Ok(false);
    }
    let lower = fs::read_to_string(path)?.to_ascii_lowercase();
    Ok(WEAK_PHRASES.iter().any(|phrase| lower.contains(phrase)))
}
