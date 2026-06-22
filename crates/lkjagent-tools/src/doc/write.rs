use std::fs;
use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;

use super::model::ScaffoldPlan;
use super::names::join_root;

pub fn write_plan(workspace: &Path, plan: &ScaffoldPlan) -> ToolResult<()> {
    workspace_path(workspace, &plan.root)?;
    for file in &plan.files {
        let full = workspace_path(workspace, &join_root(&plan.root, &file.path))?;
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full, &file.body)?;
    }
    Ok(())
}
