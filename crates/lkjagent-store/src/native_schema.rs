use std::path::Path;

use rusqlite::Connection;

use crate::error::{StoreError, StoreResult};

pub const NATIVE_SCHEMA_VERSION: i64 = 1;
pub const NATIVE_TABLES: &[&str] = &[
    "checks",
    "config_fingerprints",
    "context_items",
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
