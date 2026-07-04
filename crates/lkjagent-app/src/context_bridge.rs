use std::collections::BTreeSet;

use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_context::{
    detect_contradictions, select_normal_context, ContextConflict, ContextItem,
};
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::context_rows::{context_items, insert_context_item};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use rusqlite::Connection;

use crate::context_resolution_bridge::{apply_resolutions, persist_conflict_edges};

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
    apply_resolutions(conn, &case_id, now)?;
    let items = context_items(conn, &case_id).map_err(|error| error.to_string())?;
    let conflicts = detect_contradictions(&items);
    for conflict in &conflicts {
        append_conflict_cell(conn, &case_id, conflict, now)?;
        persist_conflict_edges(conn, &case_id, conflict, now)?;
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

fn append_conflict_cell(
    conn: &Connection,
    case_id: &str,
    conflict: &ContextConflict,
    now: &str,
) -> Result<(), String> {
    let event_id =
        next_event_id(conn, case_id, "context-conflict").map_err(|error| error.to_string())?;
    let mut cell = conflict_cell(conflict, now)?;
    cell.source_event_id = event_id.clone();
    let event = RuntimeEvent {
        id: event_id,
        case_id: case_id.to_string(),
        kind: "context.conflict".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "context-bridge".to_string(),
        created_at: now.to_string(),
        decision_id: None,
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn conflict_cell(conflict: &ContextConflict, now: &str) -> Result<StateCell, String> {
    let mut cell = StateCell::active(
        StateKey::new("context", format!("conflict/{}", conflict.semantic_key))
            .map_err(|error| error.message)?,
        format!("context-conflict/{}", conflict.semantic_key),
    );
    cell.payload_schema = "context-conflict".to_string();
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
