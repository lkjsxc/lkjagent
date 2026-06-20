use std::path::Path;

use crate::error::ToolResult;
use crate::fs::{workspace_path, write};

use super::model::ScaffoldPlan;
use super::names::join_root;

pub fn write_plan(workspace: &Path, plan: &ScaffoldPlan) -> ToolResult<()> {
    workspace_path(workspace, &plan.root)?;
    for file in &plan.files {
        write(workspace, &join_root(&plan.root, &file.path), &file.body)?;
    }
    Ok(())
}
