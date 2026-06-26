use std::fs;
use std::path::Path;

use crate::error::ToolResult;

pub fn root_needs_identity(root: &Path) -> ToolResult<bool> {
    if !root.join("catalog.toml").is_file() || !root.join("README.md").is_file() {
        return Ok(true);
    }
    Ok(!has_markdown_leaf(root)?)
}

fn has_markdown_leaf(dir: &Path) -> ToolResult<bool> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() && has_markdown_leaf(&path)? {
            return Ok(true);
        }
        if path.extension().is_some_and(|ext| ext == "md")
            && path.file_name().and_then(|name| name.to_str()) != Some("README.md")
        {
            return Ok(true);
        }
    }
    Ok(false)
}
