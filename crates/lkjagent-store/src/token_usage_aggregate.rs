use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenUsageScope {
    Latest,
    Task(i64),
    Session,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TokenUsageFieldAggregate {
    pub sum: u64,
    pub known: u64,
    pub unknown: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TokenUsageAggregate {
    pub rows: u64,
    pub rows_with_unknown: u64,
    pub input_tokens: TokenUsageFieldAggregate,
    pub output_tokens: TokenUsageFieldAggregate,
    pub cached_input_tokens: TokenUsageFieldAggregate,
    pub total_tokens: TokenUsageFieldAggregate,
}

pub fn aggregate(conn: &Connection, scope: TokenUsageScope) -> StoreResult<TokenUsageAggregate> {
    match scope {
        TokenUsageScope::Latest => aggregate_latest(conn),
        TokenUsageScope::Task(task_id) => aggregate_task(conn, task_id),
        TokenUsageScope::Session => aggregate_session(conn),
        TokenUsageScope::All => aggregate_all(conn),
    }
}

pub fn aggregate_latest(conn: &Connection) -> StoreResult<TokenUsageAggregate> {
    aggregate_query(
        conn,
        "SELECT input_tokens, output_tokens, cached_input_tokens, total_tokens
         FROM token_usage_events
         ORDER BY id DESC
         LIMIT 1",
        [],
    )
}

pub fn aggregate_task(conn: &Connection, task_id: i64) -> StoreResult<TokenUsageAggregate> {
    aggregate_query(
        conn,
        "SELECT input_tokens, output_tokens, cached_input_tokens, total_tokens
         FROM token_usage_events
         WHERE task_id = ?1
         ORDER BY id ASC",
        params![task_id],
    )
}

pub fn aggregate_session(conn: &Connection) -> StoreResult<TokenUsageAggregate> {
    let Some(started_at) = session_started_at(conn)? else {
        return aggregate_all(conn);
    };
    aggregate_query(
        conn,
        "SELECT input_tokens, output_tokens, cached_input_tokens, total_tokens
         FROM token_usage_events
         WHERE created_at >= ?1
         ORDER BY id ASC",
        params![started_at],
    )
}

pub fn aggregate_all(conn: &Connection) -> StoreResult<TokenUsageAggregate> {
    aggregate_query(
        conn,
        "SELECT input_tokens, output_tokens, cached_input_tokens, total_tokens
         FROM token_usage_events
         ORDER BY id ASC",
        [],
    )
}

fn aggregate_query<P>(conn: &Connection, sql: &str, params: P) -> StoreResult<TokenUsageAggregate>
where
    P: rusqlite::Params,
{
    let mut statement = conn.prepare(sql)?;
    let rows = statement.query_map(params, read_values)?;
    let mut aggregate = TokenUsageAggregate::default();
    for row in rows {
        aggregate.add(row?);
    }
    Ok(aggregate)
}

fn read_values(row: &rusqlite::Row<'_>) -> rusqlite::Result<UsageValues> {
    Ok(UsageValues {
        input_tokens: as_u64(row.get(0)?),
        output_tokens: as_u64(row.get(1)?),
        cached_input_tokens: as_u64(row.get(2)?),
        total_tokens: as_u64(row.get(3)?),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UsageValues {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cached_input_tokens: Option<u64>,
    total_tokens: Option<u64>,
}

impl TokenUsageAggregate {
    fn add(&mut self, values: UsageValues) {
        self.rows = self.rows.saturating_add(1);
        self.input_tokens.add(values.input_tokens);
        self.output_tokens.add(values.output_tokens);
        self.cached_input_tokens.add(values.cached_input_tokens);
        self.total_tokens.add(values.total_tokens);
        if values.has_unknown() {
            self.rows_with_unknown = self.rows_with_unknown.saturating_add(1);
        }
    }
}

impl TokenUsageFieldAggregate {
    fn add(&mut self, value: Option<u64>) {
        match value {
            Some(value) => {
                self.known = self.known.saturating_add(1);
                self.sum = self.sum.saturating_add(value);
            }
            None => self.unknown = self.unknown.saturating_add(1),
        }
    }
}

impl UsageValues {
    fn has_unknown(self) -> bool {
        self.input_tokens.is_none()
            || self.output_tokens.is_none()
            || self.cached_input_tokens.is_none()
            || self.total_tokens.is_none()
    }
}

fn session_started_at(conn: &Connection) -> StoreResult<Option<String>> {
    Ok(crate::state::get(conn, "daemon lock")?.and_then(|lock| {
        let mut parts = lock.split('|');
        parts.next()?;
        parts.next().map(str::to_string)
    }))
}

fn as_u64(value: Option<i64>) -> Option<u64> {
    value.map(|value| value.max(0) as u64)
}
