use lkjagent_store::artifact_cursor::{upsert_batch_cursor, BatchCursorInput};
use lkjagent_store::artifact_ledger::{upsert_artifact, ArtifactLedgerInput};
use rusqlite::Connection;

use crate::error::ToolResult;

pub struct NextBatchRecord<'a> {
    pub conn: &'a Connection,
    pub root: &'a str,
    pub kind: &'a str,
    pub weak_count: usize,
    pub selected: &'a [String],
    pub valid_example: &'a str,
    pub current_index: usize,
    pub now: &'a str,
}

pub fn record_next_batch(record: NextBatchRecord<'_>) -> ToolResult<()> {
    let ledger_id = upsert_repair_artifact(
        record.conn,
        record.root,
        record.kind,
        record.weak_count,
        record.now,
    )?;
    upsert_batch_cursor(
        record.conn,
        &BatchCursorInput {
            artifact_ledger_id: ledger_id,
            root: record.root,
            planned_paths: record.selected,
            completed_paths: &[],
            failed_paths: &[],
            current_index: i64::try_from(record.current_index).unwrap_or(i64::MAX),
            last_valid_example: record.valid_example,
            retry_counts: "none",
            fallback_mode: "batch-write",
            updated_at: record.now,
        },
    )?;
    Ok(())
}

fn upsert_repair_artifact(
    conn: &Connection,
    root: &str,
    kind: &str,
    weak_count: usize,
    now: &str,
) -> ToolResult<i64> {
    let case_id = case_id(conn)?;
    let kind = kind_or_default(kind);
    let topic = normalized_topic(root);
    let scale = stored_scale(conn, root)?;
    let artifact_id = format!("{case_id}:{kind}:{topic}:{scale}");
    upsert_artifact(
        conn,
        &ArtifactLedgerInput {
            case_id,
            artifact_id: &artifact_id,
            root,
            kind: &kind,
            normalized_topic: &topic,
            requested_scale: &scale,
            profile: &kind,
            lifecycle_state: "repair-planned",
            topology_status: "unknown",
            readiness_status: "failed",
            objective_match_status: "unknown",
            latest_audit_id: None,
            weak_path_count: i64::try_from(weak_count).unwrap_or(i64::MAX),
        },
        now,
    )
    .map_err(Into::into)
}

fn case_id(conn: &Connection) -> ToolResult<i64> {
    let Some(value) = lkjagent_store::state::get(conn, "authority case id")? else {
        return Ok(0);
    };
    Ok(value.parse::<i64>().ok().unwrap_or(0))
}

fn stored_scale(conn: &Connection, root: &str) -> ToolResult<String> {
    Ok(lkjagent_store::state::get(conn, &scale_key(root))?
        .unwrap_or_else(|| "unspecified".to_string()))
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

fn scale_key(root: &str) -> String {
    format!("artifact requested scale {root}")
}
