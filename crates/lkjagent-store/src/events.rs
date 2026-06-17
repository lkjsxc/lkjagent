use rusqlite::{params, Connection, Transaction};

use crate::error::StoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Owner,
    Action,
    Observation,
    Notice,
    QueueMutation,
    Compaction,
    Error,
}

impl EventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EventKind::Owner => "owner",
            EventKind::Action => "action",
            EventKind::Observation => "observation",
            EventKind::Notice => "notice",
            EventKind::QueueMutation => "queue_mutation",
            EventKind::Compaction => "compaction",
            EventKind::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRow {
    pub id: i64,
    pub turn: Option<i64>,
    pub kind: String,
    pub content: String,
    pub tokens: i64,
    pub created_at: String,
}

pub fn append_event(
    conn: &Connection,
    turn: Option<i64>,
    kind: EventKind,
    content: &str,
    tokens: i64,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO events (turn, kind, content, tokens, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![turn, kind.as_str(), content, tokens, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn append_event_tx(
    tx: &Transaction<'_>,
    turn: Option<i64>,
    kind: EventKind,
    content: &str,
    tokens: i64,
    now: &str,
) -> StoreResult<i64> {
    tx.execute(
        "INSERT INTO events (turn, kind, content, tokens, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![turn, kind.as_str(), content, tokens, now],
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn read_events(conn: &Connection) -> StoreResult<Vec<EventRow>> {
    let mut statement =
        conn.prepare("SELECT id, turn, kind, content, tokens, created_at FROM events ORDER BY id")?;
    let rows = statement.query_map([], |row| {
        Ok(EventRow {
            id: row.get(0)?,
            turn: row.get(1)?,
            kind: row.get(2)?,
            content: row.get(3)?,
            tokens: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let mut events = Vec::new();
    for row in rows {
        events.push(row?);
    }
    Ok(events)
}
