use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{observe_result, DispatchOutput, DispatchState, ToolRuntime};
use rusqlite::Connection;

pub fn dispatch_doc_audit(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    let result = crate::doc::audit(
        &runtime.workspace,
        &param(params, "root"),
        &param(params, "count"),
        &param(params, "mode"),
    )
    .and_then(|report| {
        record_doc_audit(conn, &runtime.now, &report)?;
        Ok(report)
    });
    observe_result(result, action_text, runtime, state)
}

fn record_doc_audit(conn: &Connection, now: &str, report: &str) -> crate::error::ToolResult<()> {
    let Some(root) = line_value(report, "root") else {
        return Ok(());
    };
    let topology = topology_for_report(report);
    if topology == "unknown" {
        return Ok(());
    }
    crate::artifact_ledger_state::record_state(
        conn,
        &crate::artifact_ledger_state::LedgerStateChange {
            root: &root,
            kind: &kind_for_root(&root),
            scale: "unspecified",
            lifecycle: lifecycle_for_topology(topology),
            topology,
            readiness: "not-audited",
            objective_match: "unknown",
            weak_path_count: i64::from(topology != "passed"),
        },
        now,
    )?;
    Ok(())
}

fn line_value(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::to_string))
        .filter(|value| !value.trim().is_empty())
}

fn topology_for_report(report: &str) -> &'static str {
    if report.contains("missing_root") || report.contains("root_missing") {
        "missing"
    } else if report.starts_with("document audit passed") {
        "passed"
    } else if report.starts_with("document audit failed") {
        "failed"
    } else {
        "unknown"
    }
}

fn lifecycle_for_topology(topology: &str) -> &'static str {
    match topology {
        "missing" => "root-missing",
        "passed" => "document-passed",
        _ => "document-failed",
    }
}

fn kind_for_root(root: &str) -> String {
    if root.starts_with("stories/") {
        "story".to_string()
    } else if root.starts_with("cookbooks/") {
        "cookbook".to_string()
    } else if root.starts_with("dictionaries/") {
        "dictionary".to_string()
    } else {
        "artifact".to_string()
    }
}
