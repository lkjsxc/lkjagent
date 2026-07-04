use lkjagent_core::runtime_decision::RuntimeDecision;
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
            timeout_seconds: record.timeout_seconds,
            started_at: started_at.to_string(),
            finished_at: Some(finished_at.to_string()),
        },
    )
    .map_err(|error| error.to_string())
}
