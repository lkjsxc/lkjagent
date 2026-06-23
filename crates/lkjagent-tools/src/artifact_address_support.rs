use std::path::Path;

use crate::address::{
    render_artifact_refusal, resolve_artifact_address, AddressNextAction, ArtifactAddressKind,
    ResolveInput, RootPathProblem,
};
use crate::error::{ToolError, ToolResult};

pub fn ensure_apply_root(workspace: &Path, root: &str, kind: &str) -> ToolResult<()> {
    let address = resolve_artifact_address(ResolveInput {
        workspace,
        requested_root: root,
        requested_path: None,
        kind,
    })?;
    if matches!(
        address.kind,
        ArtifactAddressKind::MissingRoot | ArtifactAddressKind::RootDirectory
    ) && address.problem.is_none()
    {
        return Ok(());
    }
    Err(ToolError::invalid(render_artifact_refusal(
        "artifact.apply",
        &address,
        kind,
    )))
}

pub fn audit_refusal(workspace: &Path, root: &str, kind: &str) -> ToolResult<Option<String>> {
    let mut address = resolve_artifact_address(ResolveInput {
        workspace,
        requested_root: root,
        requested_path: None,
        kind,
    })?;
    if matches!(address.kind, ArtifactAddressKind::RootDirectory) && address.problem.is_none() {
        return Ok(None);
    }
    if matches!(address.kind, ArtifactAddressKind::MissingRoot) && address.problem.is_none() {
        return Ok(None);
    }
    if address.problem == Some(RootPathProblem::RootIsFile) {
        if let Some(root) = address.root.clone() {
            address.next_action = AddressNextAction::AuditRoot {
                root,
                kind: kind_or_default(kind),
            };
        }
    }
    Ok(Some(render_artifact_refusal(
        "artifact.audit",
        &address,
        kind,
    )))
}

fn kind_or_default(kind: &str) -> String {
    let trimmed = kind.trim();
    if trimmed.is_empty() {
        "artifact".to_string()
    } else {
        trimmed.to_string()
    }
}
