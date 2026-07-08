use lkjagent_core::runtime_context::ContextItem;
use rusqlite::{params, Connection};

use crate::artifact_rows::{artifacts, ArtifactRow};
use crate::error::StoreResult;
use crate::record_rows::{records, RecordRow};
use crate::row_json::{json_string, json_value};

pub fn insert_context_item(
    conn: &Connection,
    case_id: &str,
    item: &ContextItem,
) -> StoreResult<()> {
    let artifact_refs_json = json_string(&item.artifact_refs)?;
    let item_json = json_string(item)?;
    conn.execute(
        "INSERT OR REPLACE INTO context_items
         (id, case_id, semantic_key, body, source_type, source_id,
          source_fingerprint, trust_class, staleness_class, contamination_class,
          artifact_refs_json, decision_id, item_json, created_at,
          suppression_reason)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                 ?14, NULL)",
        params![
            item.id,
            case_id,
            item.semantic_key,
            item.body,
            item.source_type,
            item.source_id,
            item.source_fingerprint,
            format!("{:?}", item.trust),
            format!("{:?}", item.staleness),
            format!("{:?}", item.contamination),
            artifact_refs_json,
            item.decision_id,
            item_json,
            item.created_at,
        ],
    )?;
    Ok(())
}

pub fn context_items(conn: &Connection, case_id: &str) -> StoreResult<Vec<ContextItem>> {
    let mut statement = conn.prepare(
        "SELECT item_json FROM context_items
         WHERE case_id = ?1 AND suppression_reason IS NULL ORDER BY id",
    )?;
    let rows = statement.query_map([case_id], |row| row.get::<_, String>(0))?;
    let mut items = Vec::new();
    for row in rows {
        items.push(json_value(&row?)?);
    }
    Ok(items)
}

pub fn insert_workspace_context(conn: &Connection, case_id: &str, now: &str) -> StoreResult<()> {
    for row in records(conn, None, false)?.into_iter().take(5) {
        insert_context_item(conn, case_id, &record_item(&row, now))?;
    }
    let rows = artifacts(conn, "workspace")?;
    for row in rows
        .into_iter()
        .filter(|row| row.kind == "workspace-index")
        .take(5)
    {
        insert_context_item(conn, case_id, &index_item(&row, now))?;
    }
    Ok(())
}

fn record_item(row: &RecordRow, now: &str) -> ContextItem {
    let body = format!(
        "kind={} state={} title={} path={} fp={}",
        row.kind, row.state, row.title, row.path, row.fingerprint
    );
    let mut item = ContextItem::clean_fact(
        format!("workspace-record-{}", row.id),
        format!("workspace-record:{}", row.id),
        bounded(&body),
    );
    item.source_type = "workspace_record".to_string();
    item.source_id = row.id.clone();
    item.source_fingerprint = row.fingerprint.clone();
    item.created_at = now.to_string();
    item
}

fn index_item(row: &ArtifactRow, now: &str) -> ContextItem {
    let mut item = ContextItem::clean_fact(
        format!("workspace-index-{}", row.id),
        format!("workspace-index:{}", row.path),
        format!("path={} fp={}", row.path, row.fingerprint),
    );
    item.source_type = "workspace_index".to_string();
    item.source_id = row.id.clone();
    item.source_fingerprint = row.fingerprint.clone();
    item.artifact_refs = vec![row.id.clone()];
    item.created_at = now.to_string();
    item
}

fn bounded(text: &str) -> String {
    text.chars().take(240).collect()
}

pub fn insert_context_edge(
    conn: &Connection,
    case_id: &str,
    from_item_id: &str,
    to_item_id: &str,
    edge_kind: &str,
    reason: &str,
    created_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO context_edges
         (case_id, from_item_id, to_item_id, edge_kind, reason, created_at)
         SELECT ?1, ?2, ?3, ?4, ?5, ?6
         WHERE NOT EXISTS (
           SELECT 1 FROM context_edges
           WHERE case_id = ?1 AND from_item_id = ?2 AND to_item_id = ?3
           AND edge_kind = ?4 AND reason = ?5
         )",
        params![
            case_id,
            from_item_id,
            to_item_id,
            edge_kind,
            reason,
            created_at,
        ],
    )?;
    Ok(())
}

pub fn suppress_context_item(conn: &Connection, id: &str, reason: &str) -> StoreResult<usize> {
    let changed = conn.execute(
        "UPDATE context_items SET suppression_reason = ?1 WHERE id = ?2",
        params![reason, id],
    )?;
    Ok(changed)
}
