use std::path::Path;

use lkjagent_store::artifact_ledger::{record_weak_path, WeakPathInput};
use rusqlite::Connection;

use crate::artifact_ledger_state::{
    record_state, scale_key, scale_or_default, stored_scale, LedgerStateChange,
};
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
        &LedgerStateChange {
            root,
            kind,
            scale: &selected_scale,
            lifecycle: "identity-ready",
            readiness: "not-audited",
            objective_match: "unknown",
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
        &LedgerStateChange {
            root,
            kind,
            scale: &scale,
            lifecycle: "adopted-or-scaffolded",
            readiness: "needs-audit",
            objective_match: "unknown",
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
) -> ToolResult<String> {
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
    let lifecycle = lifecycle_for_report(passed, report);
    let readiness = readiness_for_report(passed, report);
    let ledger_id = record_state(
        conn,
        &LedgerStateChange {
            root,
            kind,
            scale: &scale,
            lifecycle,
            readiness,
            objective_match: objective_match(report),
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
    Ok(append_ledger_id(report, ledger_id))
}

fn append_ledger_id(report: &str, ledger_id: i64) -> String {
    format!("{report}\nartifact_ledger_id={ledger_id}")
}

fn weak_paths(workspace: &Path, root: &str) -> ToolResult<Vec<String>> {
    let full = workspace_path(workspace, root)?;
    if !full.exists() || full.is_file() {
        return Ok(vec![root.to_string()]);
    }
    crate::doc::weak_content_paths(&full)
}

fn lifecycle_for_report(passed: bool, report: &str) -> &'static str {
    if passed {
        "audit-passed"
    } else if report.contains("address_status=root_ends_with_markdown_suffix") {
        "invalid-root"
    } else {
        "content-partial"
    }
}

fn readiness_for_report(passed: bool, report: &str) -> &'static str {
    if passed {
        "passed"
    } else if report.contains("address_status=root_ends_with_markdown_suffix") {
        "invalid"
    } else {
        "failed"
    }
}

fn objective_match(report: &str) -> &'static str {
    if report.contains("address_status=root_ends_with_markdown_suffix") {
        "failed"
    } else {
        "unknown"
    }
}
