use std::path::Path;

use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, TrustClass};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_prompt_kernel::{build_prompt_card_plan, PromptCardPlan};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::decision_rows::settle_decision;
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use lkjagent_store::exchange_rows::{insert_provider_exchange, ProviderExchangeRow};
use lkjagent_store::prompt_rows::{insert_prompt_cards, insert_prompt_frame};
use rusqlite::Connection;

use crate::context_bridge::PromptContext;
use crate::model_call::CallRecord;

pub fn persist_provider_exchange_intent(
    conn: &Connection,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    timeout_seconds: Option<u64>,
    started_at: &str,
) -> Result<(), String> {
    insert_provider_exchange(
        conn,
        &ProviderExchangeRow {
            id: format!("exchange-{}", decision.id),
            case_id: decision.case_id.clone(),
            decision_id: decision.id.clone(),
            exchange_ref: format!("unsettled-{}", decision.id),
            outcome_json: serde_json::json!({
            "state": "dispatching", "prompt_fingerprint": prompt.fingerprint })
            .to_string(),
            context_frame_fingerprint: decision.context_frame_fingerprint.clone(),
            timeout_seconds,
            started_at: started_at.to_string(),
            finished_at: None,
        },
    )
    .map_err(|error| error.to_string())
}

#[rustfmt::skip]
pub fn block_interrupted_decisions(conn: &Connection, decisions: &[RuntimeDecision],
    reason: &str, now: &str) -> Result<(), String> {
    let Some(first) = decisions.first() else { return Ok(()); };
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    for decision in decisions {
        let Some(label) = decision.selected_state_key.clone() else { continue; };
        let key = StateKey::from_label(&label).map_err(|error| error.message)?;
        let id = next_event_id(&tx, &first.case_id, "interrupted-source").map_err(|error| error.to_string())?;
        let event = RuntimeEvent { id, case_id: first.case_id.clone(), kind: "state.cell.suppress".to_string(),
            payload: RuntimeEventPayload::SuppressCell { key, reason: "provider outcome ambiguous".to_string() },
            source: "provider-recovery".to_string(), created_at: now.to_string(), decision_id: Some(decision.id.clone()) };
        append_and_apply_event(&tx, &event).map_err(|error| error.to_string())?;
    }
    let event_id = next_event_id(&tx, &first.case_id, "interrupted-provider").map_err(|error| error.to_string())?;
    let key = StateKey::new("completion", "blocked").map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, event_id.clone()); cell.payload_schema = "completion.blocked".to_string();
    cell.payload_json = serde_json::json!({ "reason": reason, "pending_decisions": decisions.iter().map(|item| &item.id).collect::<Vec<_>>(),
        "owner_action": "review the interrupted provider call and explicitly retry or replace the operation" }).to_string();
    let mut evidence_refs = Vec::new();
    for item in decisions { evidence_refs.push(EvidenceRef { source_type: "runtime_decision".to_string(),
        source_id: item.id.clone(), fingerprint: item.fingerprint().map_err(|error| error.message)? }); }
    cell.evidence_refs = evidence_refs;
    cell.created_at = now.to_string(); cell.updated_at = now.to_string();
    let event = RuntimeEvent { id: event_id, case_id: first.case_id.clone(), kind: "completion.blocked".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)), source: "provider-recovery".to_string(),
        created_at: now.to_string(), decision_id: Some(first.id.clone()) };
    append_and_apply_event(&tx, &event).map_err(|error| error.to_string())?;
    for decision in decisions { settle_decision(&tx, &decision.id, "interrupted", now).map_err(|error| error.to_string())?; }
    tx.commit().map_err(|error| error.to_string())
}

pub fn persist_provider_exchange(
    conn: &Connection,
    decision: &RuntimeDecision,
    record: &CallRecord,
    started_at: &str,
    finished_at: &str,
) -> Result<(), String> {
    insert_provider_exchange(
        conn,
        &ProviderExchangeRow {
            id: format!("exchange-{}", decision.id),
            case_id: decision.case_id.clone(),
            decision_id: decision.id.clone(),
            exchange_ref: record.exchange_ref.clone(),
            outcome_json: record.outcome_json.clone(),
            context_frame_fingerprint: decision.context_frame_fingerprint.clone(),
            timeout_seconds: record.timeout_seconds,
            started_at: started_at.to_string(),
            finished_at: Some(finished_at.to_string()),
        },
    )
    .map_err(|error| error.to_string())?;
    if let Some(item) = failure_context_item(decision, record, finished_at) {
        insert_context_item(conn, &decision.case_id, &item).map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn persist_prompt_frame(
    conn: &Connection,
    logs: &Path,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    context: &PromptContext,
    now: &str,
) -> Result<(), String> {
    let (body_ref, card_plan) = write_prompt_body(logs, decision, prompt, context)?;
    insert_prompt_frame(
        conn,
        &format!("prompt-{}", decision.id),
        decision,
        prompt,
        &body_ref,
        now,
    )
    .map_err(|error| error.to_string())?;
    insert_prompt_cards(conn, decision, &card_plan, now).map_err(|error| error.to_string())
}

fn write_prompt_body(
    logs: &Path,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    context: &PromptContext,
) -> Result<(String, PromptCardPlan), String> {
    let relative = format!(
        "logs/case-{}/decision-{}/prompt-frame.json",
        decision.case_id, decision.id
    );
    let path = logs
        .join(format!("case-{}", decision.case_id))
        .join(format!("decision-{}", decision.id));
    std::fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    let card_plan =
        build_prompt_card_plan(decision, prompt, &context.plan).map_err(|error| error.message)?;
    let body = serde_json::json!({
        "decision_id": decision.id, "prompt_fingerprint": prompt.fingerprint,
        "context_frame_fingerprint": decision.context_frame_fingerprint,
        "tool_view_fingerprint": decision.tool_view_fingerprint().unwrap_or_default(),
        "prompt_profile": card_plan.prompt_profile.clone(), "context_profile": card_plan.context_profile.clone(),
        "card_plan": card_plan, "context_plan": context.plan, "system": prompt.system,
        "user": prompt.user, "max_tokens": prompt.max_tokens, "stop": prompt.stop,
    });
    std::fs::write(path.join("prompt-frame.json"), body.to_string())
        .map_err(|error| error.to_string())?;
    Ok((relative, card_plan))
}

fn failure_context_item(
    decision: &RuntimeDecision,
    record: &CallRecord,
    created_at: &str,
) -> Option<ContextItem> {
    let contamination = contamination(&record.outcome_json)?;
    let mut item = ContextItem::clean_fact(
        format!("context-exchange-{}", decision.id),
        format!("recovery/{}", decision.id),
        record.outcome_json.clone(),
    );
    item.source_type = "provider_exchange".to_string();
    item.source_id = record.exchange_ref.clone();
    item.source_fingerprint = decision.fingerprint().unwrap_or_default();
    item.trust = TrustClass::Recovery;
    item.contamination = contamination;
    item.decision_id = Some(decision.id.clone());
    item.created_at = created_at.to_string();
    Some(item)
}

fn contamination(outcome_json: &str) -> Option<ContaminationClass> {
    if outcome_json.contains("parse_fault") {
        Some(ContaminationClass::FailedModelOutput)
    } else if outcome_json.contains("endpoint_error") {
        Some(ContaminationClass::RecoveryOnly)
    } else {
        None
    }
}
