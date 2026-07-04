use std::collections::BTreeSet;

use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_context::{
    detect_contradictions, select_normal_context, ContextConflict, ContextItem,
};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::context_rows::{context_items, insert_context_item};
use lkjagent_store::state_rows::upsert_state_cell;
use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptContext {
    pub text: String,
    pub fingerprint: String,
}

pub fn prepare_prompt_context(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    now: &str,
) -> Result<PromptContext, String> {
    let case_id = snapshot.task.id.to_string();
    insert_context_item(conn, &case_id, &objective_item(snapshot, now))
        .map_err(|error| error.to_string())?;
    let items = context_items(conn, &case_id).map_err(|error| error.to_string())?;
    let conflicts = detect_contradictions(&items);
    for conflict in &conflicts {
        upsert_state_cell(conn, &case_id, &conflict_cell(conflict, now)?)
            .map_err(|error| error.to_string())?;
    }
    let text = render_context(&items, &conflicts);
    let fingerprint = stable_fingerprint(&text).map_err(|error| error.message)?;
    Ok(PromptContext { text, fingerprint })
}

pub fn snapshot_with_prompt_context(
    snapshot: &TaskSnapshot,
    context: &PromptContext,
) -> TaskSnapshot {
    let mut next = snapshot.clone();
    if !context.text.is_empty() {
        next.task
            .brief
            .push_str(&format!("\ncontext_items:\n{}", context.text));
    }
    next
}

fn objective_item(snapshot: &TaskSnapshot, now: &str) -> ContextItem {
    let mut item = ContextItem::clean_fact(
        format!("case-{}-objective", snapshot.task.id),
        "case-objective",
        snapshot.task.objective.clone(),
    );
    item.source_type = "owner".to_string();
    item.source_id = snapshot.task.id.to_string();
    item.source_fingerprint = format!("objective-{}", snapshot.task.id);
    item.created_at = now.to_string();
    item
}

fn conflict_cell(conflict: &ContextConflict, now: &str) -> Result<StateCell, String> {
    let mut cell = StateCell::active(
        StateKey::new("context", format!("conflict/{}", conflict.semantic_key))
            .map_err(|error| error.message)?,
        format!("context-conflict/{}", conflict.semantic_key),
    );
    cell.payload_schema = "context-conflict.v1".to_string();
    cell.payload_json = serde_json::json!({
        "semantic_key": conflict.semantic_key,
        "item_ids": conflict.item_ids,
    })
    .to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "context".to_string(),
        source_id: conflict.item_ids.join(","),
        fingerprint: format!("conflict-{}", conflict.item_ids.len()),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    Ok(cell)
}

fn render_context(items: &[ContextItem], conflicts: &[ContextConflict]) -> String {
    let conflict_keys = conflicts
        .iter()
        .map(|conflict| conflict.semantic_key.as_str())
        .collect::<BTreeSet<_>>();
    let mut lines = Vec::new();
    for item in select_normal_context(items) {
        if !conflict_keys.contains(item.semantic_key.as_str()) {
            lines.push(format!(
                "{} [{}:{}] {}",
                item.semantic_key, item.source_type, item.source_id, item.body
            ));
        }
    }
    for conflict in conflicts {
        lines.push(format!(
            "Unresolved conflict {} items={}",
            conflict.semantic_key,
            conflict.item_ids.join(",")
        ));
    }
    lines.join("\n")
}
