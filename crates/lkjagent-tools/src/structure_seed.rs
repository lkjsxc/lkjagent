mod generic;
mod knowledge;
mod model;

use std::path::Path;

use crate::error::ToolResult;

pub use model::ScaffoldProfile;

pub fn scaffold_recursive_docs(workspace: &Path) -> ToolResult<String> {
    scaffold_profile(workspace, ScaffoldProfile::Generic)
}

pub fn scaffold_profile(workspace: &Path, profile: ScaffoldProfile) -> ToolResult<String> {
    match profile {
        ScaffoldProfile::Generic => generic::scaffold(workspace),
        ScaffoldProfile::Knowledge => knowledge::scaffold(workspace),
    }
}
