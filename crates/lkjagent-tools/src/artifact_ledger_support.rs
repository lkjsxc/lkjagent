use std::path::Path;

use lkjagent_store::artifact_ledger::{
    record_weak_path, upsert_artifact, ArtifactLedgerInput, WeakPathInput,
};
use rusqlite::Connection;

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn record_plan(
    conn: &Connection,
    root: &str,
    kind: &str,
    scale: &str,
    now: &str,
) -> ToolResult<()> {
    let selected_scale = scale_or_default(scale);
    record_state(
        conn,
        &StateChange {
            root,
            kind,
            scale: &selected_scale,
            lifecycle: "identity-ready",
            readiness: "not-audited",
            weak_path_count: 0,
        },
        now,
    )?;
    lkjagent_store::state::set(conn, &scale_key(root), &selected_scale)?;
    Ok(())
}

pub fn record_apply(conn: &Connection, root: &str, kind: &str, now: &str) -> ToolResult<()> {
    let scale = stored_scale(conn, root)?;
    record_state(
        conn,
        &StateChange {
            root,
            kind,
            scale: &scale,
            lifecycle: "adopted-or-scaffolded",
            readiness: "needs-audit",
            weak_path_count: 0,
        },
        now,
    )?;
    Ok(())
}

pub fn record_audit(
    workspace: &Path,
    conn: &Connection,
    root: &str,
    kind: &str,
    report: &str,
    now: &str,
) -> ToolResult<()> {
    let passed = report.starts_with("artifact audit passed")
        || report.starts_with("dictionary audit passed")
        || report.starts_with("document audit passed");
    let weak_paths = weak_paths(workspace, root)?;
    let weak_count = if passed {
        0
    } else {
        weak_paths.len().max(1) as i64
    };
    let scale = stored_scale(conn, root)?;
    let lifecycle = if passed {
        "audit-passed"
    } else {
        "content-partial"
    };
    let readiness = if passed { "passed" } else { "failed" };
    let ledger_id = record_state(
        conn,
        &StateChange {
            root,
            kind,
            scale: &scale,
            lifecycle,
            readiness,
            weak_path_count: weak_count,
        },
        now,
    )?;
    if !passed {
        for path in weak_paths {
            let missing = vec!["content-readiness".to_string()];
            let signals = vec!["audit-failed".to_string()];
            record_weak_path(
                conn,
                &WeakPathInput {
                    artifact_ledger_id: ledger_id,
                    path: &path,
                    role: "content-leaf",
                    missing_requirements: &missing,
                    weak_signals: &signals,
                    semantic_mismatch: "unknown",
                    retry_count: 0,
                    updated_at: now,
                },
            )?;
        }
    }
    Ok(())
}

struct StateChange<'a> {
    root: &'a str,
    kind: &'a str,
    scale: &'a str,
    lifecycle: &'a str,
    readiness: &'a str,
    weak_path_count: i64,
}

fn record_state(conn: &Connection, change: &StateChange<'_>, now: &str) -> ToolResult<i64> {
    let case_id = case_id(conn)?;
    let kind = kind_or_default(change.kind);
    let topic = normalized_topic(change.root);
    let scale = scale_or_default(change.scale);
    let artifact_id = format!("{case_id}:{kind}:{topic}:{scale}");
    upsert_artifact(
        conn,
        &ArtifactLedgerInput {
            case_id,
            artifact_id: &artifact_id,
            root: change.root,
            kind: &kind,
            normalized_topic: &topic,
            requested_scale: &scale,
            profile: &kind,
            lifecycle_state: change.lifecycle,
            topology_status: "unknown",
            readiness_status: change.readiness,
            objective_match_status: "unknown",
            latest_audit_id: None,
            weak_path_count: change.weak_path_count,
        },
        now,
    )
    .map_err(Into::into)
}

fn weak_paths(workspace: &Path, root: &str) -> ToolResult<Vec<String>> {
    let full = workspace_path(workspace, root)?;
    if !full.exists() {
        return Ok(vec![root.to_string()]);
    }
    crate::doc::weak_content_paths(&full)
}

fn case_id(conn: &Connection) -> ToolResult<i64> {
    let Some(value) = lkjagent_store::state::get(conn, "authority case id")? else {
        return Ok(0);
    };
    Ok(value.parse::<i64>().ok().unwrap_or(0))
}

fn normalized_topic(root: &str) -> String {
    root.rsplit('/').next().unwrap_or(root).replace('_', "-")
}

fn kind_or_default(kind: &str) -> String {
    let trimmed = kind.trim();
    if trimmed.is_empty() {
        "artifact".to_string()
    } else {
        trimmed.to_ascii_lowercase()
    }
}

fn scale_or_default(scale: &str) -> String {
    let trimmed = scale.trim();
    if trimmed.is_empty() {
        "unspecified".to_string()
    } else {
        trimmed.to_string()
    }
}

fn stored_scale(conn: &Connection, root: &str) -> ToolResult<String> {
    Ok(lkjagent_store::state::get(conn, &scale_key(root))?
        .unwrap_or_else(|| "unspecified".to_string()))
}

fn scale_key(root: &str) -> String {
    format!("artifact requested scale {root}")
}
