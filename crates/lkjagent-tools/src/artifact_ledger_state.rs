use lkjagent_store::artifact_ledger::{upsert_artifact, ArtifactLedgerInput};
use rusqlite::Connection;

use crate::error::ToolResult;

pub(crate) struct LedgerStateChange<'a> {
    pub root: &'a str,
    pub kind: &'a str,
    pub scale: &'a str,
    pub lifecycle: &'a str,
    pub topology: &'a str,
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
            topology_status: change.topology,
            readiness_status: change.readiness,
            objective_match_status: change.objective_match,
            latest_audit_id: None,
            weak_path_count: change.weak_path_count,
        },
        now,
    )
    .map_err(Into::into)
}

pub(crate) fn record_write_progress(
    conn: &Connection,
    paths: &[String],
    now: &str,
) -> ToolResult<()> {
    for root in roots_from_paths(paths) {
        record_state(
            conn,
            &LedgerStateChange {
                root: &root,
                kind: &kind_or_default_for_root(&root),
                scale: "unspecified",
                lifecycle: "write-progress",
                topology: "write-progress",
                readiness: "not-audited",
                objective_match: "unknown",
                weak_path_count: 0,
            },
            now,
        )?;
    }
    Ok(())
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

fn roots_from_paths(paths: &[String]) -> Vec<String> {
    let mut roots = Vec::new();
    for path in paths {
        let parts = path.split('/').take(2).collect::<Vec<_>>();
        if parts.len() == 2 && matches!(parts[0], "stories" | "cookbooks" | "dictionaries") {
            let root = format!("{}/{}", parts[0], parts[1]);
            if !roots.contains(&root) {
                roots.push(root);
            }
        }
    }
    roots
}

fn kind_or_default_for_root(root: &str) -> String {
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
