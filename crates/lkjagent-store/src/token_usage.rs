#[path = "token_usage_aggregate.rs"]
mod aggregate;

use rusqlite::{params, Connection, OptionalExtension};

pub use aggregate::{
    aggregate, aggregate_all, aggregate_latest, aggregate_session, aggregate_task,
    TokenUsageAggregate, TokenUsageFieldAggregate, TokenUsageScope,
};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenUsageEvent {
    pub task_id: Option<i64>,
    pub turn: i64,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cached_input_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
    pub context_window: Option<u64>,
    pub context_used_estimate: Option<u64>,
    pub source: String,
}

pub fn record(conn: &Connection, event: &TokenUsageEvent, now: &str) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO token_usage_events
         (task_id, turn, created_at, input_tokens, output_tokens,
          cached_input_tokens, total_tokens, context_window,
          context_used_estimate, source)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            event.task_id,
            event.turn,
            now,
            as_i64(event.input_tokens),
            as_i64(event.output_tokens),
            as_i64(event.cached_input_tokens),
            as_i64(event.total_tokens),
            as_i64(event.context_window),
            as_i64(event.context_used_estimate),
            event.source,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn latest(conn: &Connection) -> StoreResult<Option<TokenUsageEvent>> {
    conn.query_row(
        "SELECT task_id, turn, input_tokens, output_tokens,
                cached_input_tokens, total_tokens, context_window,
                context_used_estimate, source
         FROM token_usage_events
         ORDER BY id DESC
         LIMIT 1",
        [],
        read_event,
    )
    .optional()
    .map_err(Into::into)
}

fn read_event(row: &rusqlite::Row<'_>) -> rusqlite::Result<TokenUsageEvent> {
    Ok(TokenUsageEvent {
        task_id: row.get(0)?,
        turn: row.get(1)?,
        input_tokens: as_u64(row.get(2)?),
        output_tokens: as_u64(row.get(3)?),
        cached_input_tokens: as_u64(row.get(4)?),
        total_tokens: as_u64(row.get(5)?),
        context_window: as_u64(row.get(6)?),
        context_used_estimate: as_u64(row.get(7)?),
        source: row.get(8)?,
    })
}

fn as_i64(value: Option<u64>) -> Option<i64> {
    value.map(|value| value.min(i64::MAX as u64) as i64)
}

fn as_u64(value: Option<i64>) -> Option<u64> {
    value.map(|value| value.max(0) as u64)
}
