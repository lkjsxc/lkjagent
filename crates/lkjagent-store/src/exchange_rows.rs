use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub struct ProviderExchangeRow {
    pub id: String,
    pub case_id: String,
    pub decision_id: String,
    pub exchange_ref: String,
    pub outcome_json: String,
    pub context_frame_fingerprint: String,
    pub timeout_seconds: Option<u64>,
    pub started_at: String,
    pub finished_at: Option<String>,
}

pub fn insert_provider_exchange(conn: &Connection, row: &ProviderExchangeRow) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO provider_exchanges
         (id, case_id, decision_id, exchange_ref, outcome_json,
          context_frame_fingerprint, timeout_seconds, started_at, finished_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            row.id,
            row.case_id,
            row.decision_id,
            row.exchange_ref,
            row.outcome_json,
            row.context_frame_fingerprint,
            row.timeout_seconds.map(|value| value as i64),
            row.started_at,
            row.finished_at,
        ],
    )?;
    Ok(())
}
