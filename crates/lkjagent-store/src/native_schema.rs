use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Transaction};

use crate::error::{StoreError, StoreResult};

pub const NATIVE_SCHEMA_VERSION: i64 = 1;
pub const NATIVE_TABLES: &[&str] = &[
    "checks",
    "config_fingerprints",
    "context_items",
    "conversation_message_checks",
    "conversation_messages",
    "daemon_leases",
    "effect_journal",
    "effect_targets",
    "matters",
    "obligations",
    "observations",
    "owner_turns",
    "provider_exchanges",
    "runtime_decisions",
    "runtime_events",
    "state_cells",
    "tool_admissions",
    "workspace_documents",
    "workspace_revisions",
];

pub fn open(path: impl AsRef<Path>) -> StoreResult<Connection> {
    let connection = Connection::open(path)?;
    setup(&connection)?;
    Ok(connection)
}

#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)]
pub struct MessageIdentity { pub id: String, pub sequence: i64 }
#[rustfmt::skip]
pub struct FinalClose<'a> {
    pub matter: &'a str, pub body: &'a [u8], pub body_fingerprint: &'a [u8],
    pub receipt: &'a [u8], pub receipt_fingerprint: &'a [u8], pub event: &'a str,
    pub event_sequence: i64, pub monotonic_ms: i64, pub wall_time: &'a str, pub payload: &'a [u8],
}
#[derive(Debug, PartialEq, Eq)]
pub struct ConversationMessage {
    pub id: String,
    pub sequence: i64,
    pub role: String,
    pub body: Vec<u8>,
    pub body_fingerprint: Vec<u8>,
    pub receipt: Option<Vec<u8>>,
    pub receipt_fingerprint: Option<Vec<u8>>,
    pub lifecycle: String,
    pub matter_id: String,
    pub cause_event_id: Option<String>,
    pub replacement_id: Option<String>,
}

#[rustfmt::skip]
pub fn conversation(
    connection: &Connection,
    before_sequence: Option<i64>,
    limit: i64,
) -> StoreResult<Vec<ConversationMessage>> {
    if limit <= 0 {
        return Err(StoreError::InvalidState("conversation limit must be positive".into()));
    }
    let mut statement = connection.prepare(
        "SELECT id,sequence,role,body,body_fingerprint,receipt,receipt_fingerprint,lifecycle,
         matter_id,cause_event_id,replacement_id FROM conversation_messages
         WHERE (?1 IS NULL OR sequence < ?1) ORDER BY sequence DESC,id DESC LIMIT ?2",
    )?;
    let rows = statement.query_map(params![before_sequence, limit], |row| {
        Ok(ConversationMessage {
            id: row.get(0)?, sequence: row.get(1)?, role: row.get(2)?, body: row.get(3)?,
            body_fingerprint: row.get(4)?, receipt: row.get(5)?, receipt_fingerprint: row.get(6)?,
            lifecycle: row.get(7)?, matter_id: row.get(8)?, cause_event_id: row.get(9)?,
            replacement_id: row.get(10)?,
        })
    })?;
    let mut messages = rows.collect::<Result<Vec<_>, _>>()?;
    messages.reverse();
    Ok(messages)
}

#[rustfmt::skip]
pub(crate) fn close(tx: &Transaction<'_>, value: &FinalClose<'_>) -> StoreResult<MessageIdentity> {
    let id = format!("completion-event/{}", value.event);
    if let Some(sequence) = tx.query_row("SELECT cm.sequence FROM conversation_messages cm JOIN matters m ON m.id=cm.matter_id WHERE cm.id=?1 AND cm.body=?2 AND cm.body_fingerprint=?3 AND cm.receipt=?4 AND cm.receipt_fingerprint=?5 AND cm.cause_event_id=?6 AND m.lifecycle='closed'", params![id,value.body,value.body_fingerprint,value.receipt,value.receipt_fingerprint,value.event], |r| r.get(0)).optional()? { return Ok(MessageIdentity { id, sequence }); }
    let blockers: i64 = tx.query_row("SELECT (SELECT count(*) FROM effect_journal j JOIN runtime_decisions d ON d.id=j.decision_id WHERE d.matter_id=?1 AND j.status NOT IN ('settled','compensated'))+(SELECT count(*) FROM runtime_decisions WHERE matter_id=?1 AND status NOT IN ('settled','failed'))+(SELECT count(*) FROM obligations o LEFT JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?1 AND o.required=1 AND (o.status!='passed' OR c.current!=1 OR c.passed!=1))", [value.matter], |r| r.get(0))?;
    if blockers != 0 { return Err(StoreError::InvalidState("matter has blocking operation, effect, or check".into())); }
    let sequence = tx.query_row("SELECT coalesce(max(sequence),0)+1 FROM conversation_messages", [], |r| r.get(0))?;
    tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'matter-completed',?4,?5,?6,'harness',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.payload,id])?;
    tx.execute("INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,receipt,receipt_fingerprint,lifecycle,matter_id,cause_event_id) VALUES(?1,?2,'agent',?3,?4,?5,?6,'active',?7,?8)", params![id,sequence,value.body,value.body_fingerprint,value.receipt,value.receipt_fingerprint,value.matter,value.event])?;
    tx.execute("INSERT INTO conversation_message_checks(message_id,check_id,evidence_fingerprint) SELECT ?1,c.id,c.evidence_fingerprint FROM obligations o JOIN checks c ON c.id=o.current_check_id WHERE o.matter_id=?2 AND o.required=1 AND o.status='passed' AND c.current=1 AND c.passed=1", params![id,value.matter])?;
    tx.execute("UPDATE conversation_messages SET lifecycle='replaced',replacement_id=?1 WHERE matter_id=?2 AND role='agent' AND lifecycle='active' AND id<>?1", params![id,value.matter])?;
    changed(tx.execute("UPDATE matters SET lifecycle='closed',closure_event_id=?1,closure_checks_passed=1,unsettled_effects=0,updated_sequence=?2 WHERE id=?3 AND lifecycle='open'", params![value.event,value.event_sequence,value.matter])?, "matter cannot close")?;
    Ok(MessageIdentity { id, sequence })
}
#[rustfmt::skip]
fn changed(count: usize, message: &str) -> StoreResult<()> {
    if count == 1 { Ok(()) } else { Err(StoreError::InvalidState(message.into())) }
}

pub fn setup(connection: &Connection) -> StoreResult<()> {
    match schema_state(connection)? {
        SchemaState::Empty => create_fresh(connection),
        SchemaState::Native => {
            validate_native(connection)?;
            configure(connection)
        }
        SchemaState::Incompatible { version, objects } => {
            Err(StoreError::IncompatibleSchema { version, objects })
        }
    }
}

fn create_fresh(connection: &Connection) -> StoreResult<()> {
    configure(connection)?;
    connection.execute_batch("BEGIN EXCLUSIVE;")?;
    let result = match schema_state(connection)? {
        SchemaState::Empty => create_schema(connection),
        SchemaState::Native | SchemaState::Incompatible { .. } => Err(incompatible(connection)?),
    };
    match result {
        Ok(()) => connection.execute_batch("COMMIT;").map_err(Into::into),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(error)
        }
    }
}

fn configure(connection: &Connection) -> StoreResult<()> {
    connection.execute_batch(
        "PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL;
         PRAGMA synchronous=FULL; PRAGMA busy_timeout=5000;",
    )?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum SchemaState {
    Empty,
    Native,
    Incompatible { version: i64, objects: Vec<String> },
}

fn schema_state(connection: &Connection) -> StoreResult<SchemaState> {
    let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let objects = object_names(connection)?;
    match (version, objects.is_empty()) {
        (0, true) => Ok(SchemaState::Empty),
        (NATIVE_SCHEMA_VERSION, false) => Ok(SchemaState::Native),
        _ => Ok(SchemaState::Incompatible { version, objects }),
    }
}

fn validate_native(connection: &Connection) -> StoreResult<()> {
    let expected = Connection::open_in_memory()?;
    expected.execute_batch(SCHEMA)?;
    if schema_objects(connection)? == schema_objects(&expected)? {
        Ok(())
    } else {
        Err(incompatible(connection)?)
    }
}

fn incompatible(connection: &Connection) -> StoreResult<StoreError> {
    let version = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    Ok(StoreError::IncompatibleSchema {
        version,
        objects: object_names(connection)?,
    })
}

fn object_names(connection: &Connection) -> StoreResult<Vec<String>> {
    Ok(schema_objects(connection)?
        .into_iter()
        .map(|(_, name, _, _)| name)
        .collect())
}

fn schema_objects(connection: &Connection) -> StoreResult<Vec<(String, String, String, String)>> {
    let mut statement = connection.prepare(
        "SELECT type,name,tbl_name,coalesce(sql,'') FROM sqlite_schema
         WHERE name NOT LIKE 'sqlite_%' ORDER BY type,name,tbl_name",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn create_schema(connection: &Connection) -> StoreResult<()> {
    connection.execute_batch(SCHEMA)?;
    connection.pragma_update(None, "user_version", NATIVE_SCHEMA_VERSION)?;
    Ok(())
}

const SCHEMA: &str = include_str!("native-schema.sql");
