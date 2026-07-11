use std::path::Path;

use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, TrustClass};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_prompt_kernel::{build_prompt_card_plan, PromptCardPlan};
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::exchange_rows::{insert_provider_exchange, ProviderExchangeRow};
use lkjagent_store::prompt_rows::{insert_prompt_cards, insert_prompt_frame};
use rusqlite::Connection;

use crate::context_bridge::PromptContext;
use crate::model_call::CallRecord;

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
