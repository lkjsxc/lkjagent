use std::path::Path;

use crate::address::{
    render_artifact_refusal, resolve_artifact_address, ArtifactAddress, ArtifactAddressKind,
    ResolveInput,
};
use crate::artifact_next_response::{
    audit_response, batch_response, can_repair_file, cursor_key, focused_response,
    missing_root_response, path_param, resolved_kind, root,
};
use crate::error::ToolResult;
use crate::fs::workspace_path;
use rusqlite::Connection;

pub fn next(workspace: &Path, root: &str, path: &str, kind: &str) -> ToolResult<String> {
    crate::artifact::reject_empty_root(root)?;
    let address = resolve_artifact_address(ResolveInput {
        workspace,
        requested_root: root,
        requested_path: path_param(path),
        kind,
    })?;
    next_for_address(workspace, address, kind)
}

pub fn next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    path: &str,
    kind: &str,
) -> ToolResult<String> {
    crate::artifact::reject_empty_root(root)?;
    let address = resolve_artifact_address(ResolveInput {
        workspace,
        requested_root: root,
        requested_path: path_param(path),
        kind,
    })?;
    next_for_address_with_cursor(workspace, conn, now, address, kind)
}

fn next_for_address(workspace: &Path, address: ArtifactAddress, kind: &str) -> ToolResult<String> {
    match address.kind {
        ArtifactAddressKind::MissingRoot => Ok(missing_root_response(&address)),
        ArtifactAddressKind::RootDirectory if address.weak_path.is_some() => {
            Ok(focused_response(&address, kind))
        }
        ArtifactAddressKind::RootDirectory => root_next(workspace, &address, kind),
        ArtifactAddressKind::FileUnderKnownRoot if can_repair_file(&address) => {
            Ok(focused_response(&address, kind))
        }
        _ => Ok(render_artifact_refusal("artifact.next", &address, kind)),
    }
}

fn next_for_address_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    address: ArtifactAddress,
    kind: &str,
) -> ToolResult<String> {
    match address.kind {
        ArtifactAddressKind::MissingRoot => {
            if let Some(root) = address.root.as_deref() {
                let kind = resolved_kind(kind, &workspace_path(workspace, root)?);
                let contract = crate::artifact_next_example::root_identity_contract(root, &kind);
                crate::artifact_cursor_support::record_identity_contract(
                    conn, root, &kind, &contract, now,
                )?;
            }
            Ok(missing_root_response(&address))
        }
        ArtifactAddressKind::RootDirectory if address.weak_path.is_some() => {
            Ok(focused_response(&address, kind))
        }
        ArtifactAddressKind::RootDirectory => {
            root_next_with_cursor(workspace, conn, now, &address, kind)
        }
        ArtifactAddressKind::FileUnderKnownRoot if can_repair_file(&address) => {
            Ok(focused_response(&address, kind))
        }
        _ => Ok(render_artifact_refusal("artifact.next", &address, kind)),
    }
}

fn root_next(workspace: &Path, address: &ArtifactAddress, kind: &str) -> ToolResult<String> {
    let root = root(address);
    let full = workspace_path(workspace, &root)?;
    let kind = resolved_kind(kind, &full);
    if crate::artifact_next_identity::root_needs_identity(&full)? {
        return Ok(
            crate::artifact_next_identity_contract::identity_contract_for(&root, &kind, &full)
                .response,
        );
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            return Ok(report.block_message(&root));
        }
    }
    if let Some(contract) =
        crate::artifact_next_story::story_contract_if_missing(&root, &kind, "unspecified", &full)?
    {
        return Ok(contract.response);
    }
    if let Some(contract) = crate::artifact_content_atom::contract_if_missing(&root, &kind, &full)?
    {
        return Ok(contract.response);
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        return audit_response(&root, &kind, "missing=0");
    }
    let selected = weak.into_iter().take(1).collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write_contract(&root, &kind, &selected);
    Ok(batch_response(&root, &kind, &selected, &valid_example))
}

fn root_next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    address: &ArtifactAddress,
    kind: &str,
) -> ToolResult<String> {
    let root = root(address);
    let full = workspace_path(workspace, &root)?;
    let kind = resolved_kind(kind, &full);
    if crate::artifact_next_identity::root_needs_identity(&full)? {
        let contract =
            crate::artifact_next_identity_contract::identity_contract_for(&root, &kind, &full);
        crate::artifact_next_cursor::record_story_batch(
            conn,
            now,
            &root,
            &kind,
            &contract.selected,
            &contract.valid_example,
        )?;
        return Ok(contract.response);
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            lkjagent_store::state::delete(conn, &cursor_key(&root))?;
            return Ok(report.block_message(&root));
        }
    }
    let scale = lkjagent_store::state::get(conn, &format!("artifact requested scale {root}"))?
        .unwrap_or_else(|| "unspecified".to_string());
    if let Some(contract) =
        crate::artifact_next_story::story_contract_if_missing(&root, &kind, &scale, &full)?
    {
        crate::artifact_next_cursor::record_story_batch(
            conn,
            now,
            &root,
            &kind,
            &contract.selected,
            &contract.valid_example,
        )?;
        return Ok(contract.response);
    }
    if let Some(contract) = crate::artifact_content_atom::contract_if_missing(&root, &kind, &full)?
    {
        crate::artifact_next_cursor::record_story_batch(
            conn,
            now,
            &root,
            &kind,
            &contract.selected,
            &contract.valid_example,
        )?;
        return Ok(contract.response);
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        lkjagent_store::state::delete(conn, &cursor_key(&root))?;
        return audit_response(&root, &kind, "missing=0");
    }
    crate::artifact_next_cursor::cursor_batch(conn, now, &root, &kind, weak)
}
