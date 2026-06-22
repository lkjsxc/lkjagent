use crate::error::{ToolError, ToolResult};

pub(crate) fn parse_patch(patch: &str) -> ToolResult<Vec<(String, String)>> {
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
