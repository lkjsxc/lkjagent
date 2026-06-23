use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, Copy)]
pub struct ProviderExchangeRequest<'a> {
    pub id: &'a str,
    pub case_id: &'a str,
    pub turn_id: i64,
    pub prompt_frame_id: Option<&'a str>,
    pub authority_decision_id: Option<&'a str>,
    pub admission_decision_id: Option<&'a str>,
    pub provider: &'a str,
    pub model: &'a str,
    pub created_at: &'a str,
    pub request_json: &'a str,
    pub request_hash: &'a str,
    pub redaction_schema_version: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct ProviderExchangeCompletion<'a> {
    pub id: &'a str,
    pub response_json: &'a str,
    pub response_hash: &'a str,
    pub finish_reason: &'a str,
    pub usage_json: Option<&'a str>,
    pub stats_json: Option<&'a str>,
    pub latency_ms: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct ProviderExchangeFailure<'a> {
    pub id: &'a str,
    pub error_class: &'a str,
    pub response_json: Option<&'a str>,
    pub response_hash: Option<&'a str>,
    pub latency_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExchangeRow {
    pub id: String,
    pub case_id: String,
    pub turn_id: i64,
    pub status: String,
    pub finish_reason: Option<String>,
    pub error_class: Option<String>,
    pub request_hash: String,
    pub response_hash: Option<String>,
}

pub fn record_request(conn: &Connection, input: &ProviderExchangeRequest<'_>) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO provider_exchange
         (id, case_id, turn_id, prompt_frame_id, authority_decision_id,
          admission_decision_id, provider, model, created_at, request_json,
          request_hash, status, redaction_schema_version)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 'requested', ?12)",
        params![
            input.id,
            input.case_id,
            input.turn_id,
            input.prompt_frame_id,
            input.authority_decision_id,
            input.admission_decision_id,
            input.provider,
            input.model,
            input.created_at,
            input.request_json,
            input.request_hash,
            input.redaction_schema_version,
        ],
    )?;
    Ok(())
}

pub fn complete_exchange(
    conn: &Connection,
    input: &ProviderExchangeCompletion<'_>,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE provider_exchange SET response_json = ?2, response_hash = ?3,
         finish_reason = ?4, usage_json = ?5, stats_json = ?6, latency_ms = ?7,
         status = 'succeeded', error_class = NULL WHERE id = ?1",
        params![
            input.id,
            input.response_json,
            input.response_hash,
            input.finish_reason,
            input.usage_json,
            input.stats_json,
            input.latency_ms,
        ],
    )?;
    Ok(())
}

pub fn fail_exchange(conn: &Connection, input: &ProviderExchangeFailure<'_>) -> StoreResult<()> {
    conn.execute(
        "UPDATE provider_exchange SET response_json = ?2, response_hash = ?3,
         latency_ms = ?4, status = 'failed', error_class = ?5 WHERE id = ?1",
        params![
            input.id,
            input.response_json,
            input.response_hash,
            input.latency_ms,
            input.error_class,
        ],
    )?;
    Ok(())
}

pub fn latest_for_case_turn(
    conn: &Connection,
    case_id: &str,
    turn_id: i64,
) -> StoreResult<Option<ProviderExchangeRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, turn_id, status, finish_reason, error_class,
         request_hash, response_hash FROM provider_exchange
         WHERE case_id = ?1 AND turn_id = ?2 ORDER BY created_at DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![case_id, turn_id])?;
    Ok(rows.next()?.map(row).transpose()?)
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProviderExchangeRow> {
    Ok(ProviderExchangeRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        turn_id: row.get(2)?,
        status: row.get(3)?,
        finish_reason: row.get(4)?,
        error_class: row.get(5)?,
        request_hash: row.get(6)?,
        response_hash: row.get(7)?,
    })
}
