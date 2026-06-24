use lkjagent_store::artifact_ledger::{upsert_artifact, ArtifactLedgerInput};
use rusqlite::Connection;

use crate::error::ToolResult;

pub(crate) struct LedgerStateChange<'a> {
    pub root: &'a str,
    pub kind: &'a str,
    pub scale: &'a str,
    pub lifecycle: &'a str,
    pub readiness: &'a str,
    pub objective_match: &'a str,
    pub weak_path_count: i64,
}

pub(crate) fn record_state(
    conn: &Connection,
    change: &LedgerStateChange<'_>,
    now: &str,
) -> ToolResult<i64> {
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
            objective_match_status: change.objective_match,
            latest_audit_id: None,
            weak_path_count: change.weak_path_count,
        },
        now,
    )
    .map_err(Into::into)
}

pub(crate) fn case_id(conn: &Connection) -> ToolResult<i64> {
    let Some(value) = lkjagent_store::state::get(conn, "authority case id")? else {
        return Ok(0);
    };
    Ok(value.parse::<i64>().ok().unwrap_or(0))
}

pub(crate) fn stored_scale(conn: &Connection, root: &str) -> ToolResult<String> {
    Ok(lkjagent_store::state::get(conn, &scale_key(root))?
        .unwrap_or_else(|| "unspecified".to_string()))
}

pub(crate) fn normalized_topic(root: &str) -> String {
    root.rsplit('/').next().unwrap_or(root).replace('_', "-")
}

pub(crate) fn kind_or_default(kind: &str) -> String {
    let trimmed = kind.trim();
    if trimmed.is_empty() {
        "artifact".to_string()
    } else {
        trimmed.to_ascii_lowercase()
    }
}

pub(crate) fn scale_or_default(scale: &str) -> String {
    let trimmed = scale.trim();
    if trimmed.is_empty() {
        "unspecified".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(crate) fn scale_key(root: &str) -> String {
    format!("artifact requested scale {root}")
}
