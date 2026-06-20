use std::fs;
use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn stat(workspace: &Path, path: &str) -> ToolResult<String> {
    let full = workspace_path(workspace, path)?;
    let meta = fs::metadata(&full)?;
    let kind = if meta.is_dir() { "dir" } else { "file" };
    let text = if meta.is_file() {
        fs::read_to_string(&full).ok()
    } else {
        None
    };
    let lines = text
        .as_deref()
        .map(|body| {
            if body.is_empty() {
                0
            } else {
                body.lines().count()
            }
        })
        .unwrap_or(0);
    let checksum = text.as_deref().map_or_else(
        || "none".to_string(),
        |body| format!("{:016x}", fnv1a(body.as_bytes())),
    );
    Ok(format!(
        "path={path}\nkind={kind}\nbytes={}\nlines={lines}\nchecksum={checksum}",
        meta.len()
    ))
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
