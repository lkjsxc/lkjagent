use std::path::Path;

use super::model::{
    AddressNextAction, ArtifactAddress, ArtifactAddressKind, PathKind, RootPathProblem,
};
use super::path::{nearest_catalog_root, parent_path, relative, root_looks_like_markdown_file};

pub fn resolve_file(
    workspace: &Path,
    requested: &str,
    full: &Path,
    kind: String,
) -> ArtifactAddress {
    let Some(root_path) = nearest_catalog_root(workspace, full) else {
        return ArtifactAddress {
            requested: requested.to_string(),
            root: None,
            weak_path: None,
            kind: ArtifactAddressKind::InvalidRoot,
            problem: Some(RootPathProblem::RootIsFile),
            detected: PathKind::File,
            next_action: AddressNextAction::InspectParent {
                path: parent_path(requested),
            },
        };
    };
    let root = relative(workspace, &root_path).unwrap_or_else(|| parent_path(requested));
    let weak = full
        .strip_prefix(&root_path)
        .ok()
        .map(|path| path.to_string_lossy().to_string())
        .filter(|path| !path.is_empty());
    let problem = if root_looks_like_markdown_file(&root) {
        RootPathProblem::RootEndsWithMarkdownSuffix
    } else {
        RootPathProblem::RootIsFile
    };
    ArtifactAddress {
        requested: requested.to_string(),
        root: Some(root.clone()),
        weak_path: weak.clone(),
        kind: ArtifactAddressKind::FileUnderKnownRoot,
        problem: Some(problem.clone()),
        detected: PathKind::File,
        next_action: file_next_action(problem, &root, weak, kind),
    }
}

fn file_next_action(
    problem: RootPathProblem,
    root: &str,
    weak: Option<String>,
    kind: String,
) -> AddressNextAction {
    match (problem, weak) {
        (RootPathProblem::RootIsFile, Some(path)) => AddressNextAction::RepairPath {
            root: root.to_string(),
            path,
            kind,
        },
        _ => AddressNextAction::InspectParent {
            path: parent_path(root),
        },
    }
}
