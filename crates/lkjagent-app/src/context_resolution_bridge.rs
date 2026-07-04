use lkjagent_core::runtime_context::ContextConflict;
use lkjagent_store::context_rows::{insert_context_edge, suppress_context_item};
use rusqlite::Connection;

pub fn apply_resolutions(conn: &Connection, case_id: &str, now: &str) -> Result<(), String> {
    for resolution in resolution_cells(conn, case_id)? {
        let mut statement = conn
            .prepare(
                "SELECT id FROM context_items
                 WHERE case_id = ?1 AND semantic_key = ?2 AND id != ?3
                 AND suppression_reason IS NULL",
            )
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map(
                (
                    case_id,
                    &resolution.semantic_key,
                    &resolution.winning_item_id,
                ),
                |row| row.get::<_, String>(0),
            )
            .map_err(|error| error.to_string())?;
        for row in rows {
            suppress_loser(
                conn,
                case_id,
                &resolution,
                &row.map_err(|e| e.to_string())?,
                now,
            )?;
        }
    }
    Ok(())
}

pub fn persist_conflict_edges(
    conn: &Connection,
    case_id: &str,
    conflict: &ContextConflict,
    now: &str,
) -> Result<(), String> {
    let Some(first) = conflict.item_ids.first() else {
        return Ok(());
    };
    for other in conflict.item_ids.iter().skip(1) {
        insert_context_edge(
            conn,
            case_id,
            first,
            other,
            "contradicts",
            &conflict.semantic_key,
            now,
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn suppress_loser(
    conn: &Connection,
    case_id: &str,
    resolution: &Resolution,
    losing_id: &str,
    now: &str,
) -> Result<(), String> {
    suppress_context_item(conn, losing_id, "resolved-conflict")
        .map_err(|error| error.to_string())?;
    insert_context_edge(
        conn,
        case_id,
        losing_id,
        &resolution.winning_item_id,
        "resolved-by",
        "context resolution cell",
        now,
    )
    .map_err(|error| error.to_string())
}

fn resolution_cells(conn: &Connection, case_id: &str) -> Result<Vec<Resolution>, String> {
    let mut statement = conn
        .prepare(
            "SELECT payload_json FROM state_cells
             WHERE case_id = ?1 AND key_label LIKE 'context:resolve/%'
             AND status = 'Active'",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([case_id], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?;
    let mut output = Vec::new();
    for row in rows {
        if let Some(resolution) = resolution_from_json(&row.map_err(|e| e.to_string())?)? {
            output.push(resolution);
        }
    }
    Ok(output)
}

fn resolution_from_json(raw: &str) -> Result<Option<Resolution>, String> {
    let value: serde_json::Value = serde_json::from_str(raw).map_err(|error| error.to_string())?;
    let Some(semantic_key) = value
        .get("semantic_key")
        .and_then(serde_json::Value::as_str)
    else {
        return Ok(None);
    };
    let Some(winning_item_id) = value
        .get("winning_item_id")
        .and_then(serde_json::Value::as_str)
    else {
        return Ok(None);
    };
    Ok(Some(Resolution {
        semantic_key: semantic_key.to_string(),
        winning_item_id: winning_item_id.to_string(),
    }))
}

struct Resolution {
    semantic_key: String,
    winning_item_id: String,
}
