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
    pub status: &'a str,
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
    pub provider: String,
    pub model: String,
    pub created_at: String,
    pub status: String,
    pub finish_reason: Option<String>,
    pub error_class: Option<String>,
    pub request_hash: String,
    pub response_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExchangeDetail {
    pub row: ProviderExchangeRow,
    pub request_json: String,
    pub response_json: Option<String>,
    pub usage_json: Option<String>,
    pub latency_ms: Option<i64>,
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
         status = ?8, error_class = NULL WHERE id = ?1",
        params![
            input.id,
            input.response_json,
            input.response_hash,
            input.finish_reason,
            input.usage_json,
            input.stats_json,
            input.latency_ms,
            input.status,
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
        "SELECT id, case_id, turn_id, provider, model, created_at, status,
         finish_reason, error_class, request_hash, response_hash FROM provider_exchange
         WHERE case_id = ?1 AND turn_id = ?2 ORDER BY created_at DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![case_id, turn_id])?;
    Ok(rows.next()?.map(row).transpose()?)
}

pub fn list_recent(conn: &Connection, limit: usize) -> StoreResult<Vec<ProviderExchangeRow>> {
    let sql = "SELECT id, case_id, turn_id, provider, model, created_at, status,
        finish_reason, error_class, request_hash, response_hash
        FROM provider_exchange ORDER BY created_at DESC LIMIT ?1";
    let mut statement = conn.prepare(sql)?;
    let rows = statement.query_map(params![limit as i64], row)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

pub fn detail_for_case_turn(
    conn: &Connection,
    case_id: &str,
    turn_id: i64,
) -> StoreResult<Option<ProviderExchangeDetail>> {
    let sql = "SELECT id, case_id, turn_id, provider, model, created_at, status,
        finish_reason, error_class, request_hash, response_hash, request_json,
        response_json, usage_json, latency_ms FROM provider_exchange
        WHERE case_id = ?1 AND turn_id = ?2 ORDER BY created_at DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id, turn_id])?;
    Ok(rows.next()?.map(detail).transpose()?)
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProviderExchangeRow> {
    Ok(ProviderExchangeRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        turn_id: row.get(2)?,
        provider: row.get(3)?,
        model: row.get(4)?,
        created_at: row.get(5)?,
        status: row.get(6)?,
        finish_reason: row.get(7)?,
        error_class: row.get(8)?,
        request_hash: row.get(9)?,
        response_hash: row.get(10)?,
    })
}

fn detail(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProviderExchangeDetail> {
    Ok(ProviderExchangeDetail {
        row: self::row(row)?,
        request_json: row.get(11)?,
        response_json: row.get(12)?,
        usage_json: row.get(13)?,
        latency_ms: row.get(14)?,
    })
}
