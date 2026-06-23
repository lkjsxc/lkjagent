use crate::model::RepoFile;

use super::findings::StructureFinding;

pub fn workspace_brief_findings(files: &[RepoFile], root: &str) -> Vec<StructureFinding> {
    if root.trim_end_matches('/') != "data/workspace" {
        return Vec::new();
    }
    if files
        .iter()
        .any(|file| file.path == "data/workspace/AGENTS.md")
    {
        return Vec::new();
    }
    vec![StructureFinding::new(
        "data/workspace",
        "structure workspace-brief",
        "create AGENTS.md with the runtime workspace brief",
    )]
}
