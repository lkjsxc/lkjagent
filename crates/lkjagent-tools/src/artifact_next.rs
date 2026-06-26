use std::path::Path;

use crate::address::{
    render_artifact_refusal, resolve_artifact_address, ArtifactAddress, ArtifactAddressKind,
    ResolveInput,
};
use crate::artifact_next_response::{
    audit_response, batch_response, can_repair_file, cursor_key, focused_response,
    missing_root_response, path_param, resolved_kind, root, root_identity_response,
};
use crate::error::ToolResult;
use crate::fs::workspace_path;
use rusqlite::Connection;

const WEAK_PATH_BATCH_SIZE: usize = 6;

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
                lkjagent_store::state::delete(conn, &cursor_key(root))?;
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
        return Ok(root_identity_response(&root, &kind));
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            return Ok(report.block_message(&root));
        }
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        return audit_response(&root, &kind, "missing=0");
    }
    let selected = weak
        .into_iter()
        .take(WEAK_PATH_BATCH_SIZE)
        .collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write(&root, &kind, &selected);
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
        lkjagent_store::state::delete(conn, &cursor_key(&root))?;
        return Ok(root_identity_response(&root, &kind));
    }
    if let Some(report) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !report.is_empty() {
            lkjagent_store::state::delete(conn, &cursor_key(&root))?;
            return Ok(report.block_message(&root));
        }
    }
    let weak = crate::doc::weak_content_paths(&full)?;
    if weak.is_empty() {
        lkjagent_store::state::delete(conn, &cursor_key(&root))?;
        return audit_response(&root, &kind, "missing=0");
    }
    cursor_batch(conn, now, &root, &kind, weak)
}

fn cursor_batch(
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
    weak: Vec<String>,
) -> ToolResult<String> {
    let start = next_start(conn, root, &weak)?;
    if start >= weak.len() {
        return audit_response(root, kind, &format!("missing={}", weak.len()));
    }
    let weak_count = weak.len();
    let selected = weak
        .into_iter()
        .skip(start)
        .take(WEAK_PATH_BATCH_SIZE)
        .collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write(root, kind, &selected);
    crate::artifact_cursor_support::record_next_batch(
        crate::artifact_cursor_support::NextBatchRecord {
            conn,
            root,
            kind,
            weak_count,
            selected: &selected,
            valid_example: &valid_example,
            current_index: start.saturating_add(selected.len()),
            now,
        },
    )?;
    if let Some(last) = selected.last() {
        lkjagent_store::state::set(conn, &cursor_key(root), last)?;
    }
    Ok(batch_response(root, kind, &selected, &valid_example))
}

fn next_start(conn: &Connection, root: &str, weak: &[String]) -> ToolResult<usize> {
    let Some(cursor) = lkjagent_store::state::get(conn, &cursor_key(root))? else {
        return Ok(0);
    };
    Ok(weak
        .iter()
        .position(|path| path > &cursor)
        .unwrap_or(weak.len()))
}
