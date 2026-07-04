use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, TrustClass};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::exchange_rows::{insert_provider_exchange, ProviderExchangeRow};
use rusqlite::Connection;

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
