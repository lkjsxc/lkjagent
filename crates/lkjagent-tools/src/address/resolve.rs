use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;

use super::file::resolve_file;
use super::model::{
    AddressNextAction, ArtifactAddress, ArtifactAddressKind, PathKind, RootPathProblem,
};
use super::path::{
    clean_requested_path, kind_or_default, parent_path, path_name_ends_md,
    reject_escaping_relative, root_looks_like_markdown_file,
};

pub struct ResolveInput<'a> {
    pub workspace: &'a Path,
    pub requested_root: &'a str,
    pub requested_path: Option<&'a str>,
    pub kind: &'a str,
}

pub fn resolve_artifact_address(input: ResolveInput<'_>) -> ToolResult<ArtifactAddress> {
    let requested = input.requested_root.trim();
    let kind = kind_or_default(input.kind);
    if requested.is_empty() {
        return Ok(invalid(
            requested,
            None,
            None,
            PathKind::Missing,
            RootPathProblem::RootMissing,
            AddressNextAction::Refuse {
                reason: "root must not be empty".to_string(),
            },
        ));
    }
    let full = match workspace_path(input.workspace, requested) {
        Ok(path) => path,
        Err(_) => return Ok(outside_workspace(requested)),
    };
    if full.is_file() {
        return Ok(resolve_file(input.workspace, requested, &full, kind));
    }
    if full.is_dir() {
        return resolve_directory(input, requested, &full, kind);
    }
    if full.exists() {
        return Ok(invalid(
            requested,
            None,
            None,
            PathKind::Other,
            RootPathProblem::RootNotDirectory,
            AddressNextAction::InspectParent {
                path: parent_path(requested),
            },
        ));
    }
    if root_looks_like_markdown_file(requested) {
        return Ok(invalid(
            requested,
            None,
            None,
            PathKind::Missing,
            RootPathProblem::RootEndsWithMarkdownSuffix,
            AddressNextAction::InspectParent {
                path: parent_path(requested),
            },
        ));
    }
    missing_root(input, requested, kind)
}

fn outside_workspace(requested: &str) -> ArtifactAddress {
    invalid(
        requested,
        None,
        None,
        PathKind::Other,
        RootPathProblem::RootOutsideWorkspace,
        AddressNextAction::Refuse {
            reason: "root must stay inside workspace".to_string(),
        },
    )
}

fn missing_root(
    input: ResolveInput<'_>,
    requested: &str,
    kind: String,
) -> ToolResult<ArtifactAddress> {
    let weak_path = clean_requested_path(input.requested_path);
    if let Some(path) = &weak_path {
        reject_escaping_relative(path)?;
    }
    Ok(ArtifactAddress {
        requested: requested.to_string(),
        root: Some(requested.to_string()),
        weak_path,
        kind: ArtifactAddressKind::MissingRoot,
        problem: None,
        detected: PathKind::Missing,
        next_action: AddressNextAction::ApplyRoot {
            root: requested.to_string(),
            kind,
        },
    })
}

fn resolve_directory(
    input: ResolveInput<'_>,
    requested: &str,
    full: &Path,
    kind: String,
) -> ToolResult<ArtifactAddress> {
    if root_looks_like_markdown_file(requested) || path_name_ends_md(full) {
        return Ok(invalid(
            requested,
            Some(requested.to_string()),
            clean_requested_path(input.requested_path),
            PathKind::Directory,
            RootPathProblem::RootEndsWithMarkdownSuffix,
            AddressNextAction::InspectParent {
                path: parent_path(requested),
            },
        ));
    }
    if let Some(path) = clean_requested_path(input.requested_path) {
        reject_escaping_relative(&path)?;
        return Ok(directory_repair(requested, path, kind));
    }
    Ok(ArtifactAddress {
        requested: requested.to_string(),
        root: Some(requested.to_string()),
        weak_path: None,
        kind: ArtifactAddressKind::RootDirectory,
        problem: None,
        detected: PathKind::Directory,
        next_action: AddressNextAction::AuditRoot {
            root: requested.to_string(),
            kind,
        },
    })
}

fn directory_repair(requested: &str, path: String, kind: String) -> ArtifactAddress {
    ArtifactAddress {
        requested: requested.to_string(),
        root: Some(requested.to_string()),
        weak_path: Some(path.clone()),
        kind: ArtifactAddressKind::RootDirectory,
        problem: None,
        detected: PathKind::Directory,
        next_action: AddressNextAction::RepairPath {
            root: requested.to_string(),
            path,
            kind,
        },
    }
}

fn invalid(
    requested: &str,
    root: Option<String>,
    weak_path: Option<String>,
    detected: PathKind,
    problem: RootPathProblem,
    next_action: AddressNextAction,
) -> ArtifactAddress {
    ArtifactAddress {
        requested: requested.to_string(),
        root,
        weak_path,
        kind: ArtifactAddressKind::InvalidRoot,
        problem: Some(problem),
        detected,
        next_action,
    }
}
