use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::error::{StoreError, StoreResult};
use lkjagent_store::native_schema::{self, NATIVE_SCHEMA_VERSION, NATIVE_TABLES};
use rusqlite::{params, Connection};

fn path(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "lkjagent-{label}-{}-{nonce}.db",
        std::process::id()
    )))
}

fn names(connection: &Connection) -> StoreResult<Vec<String>> {
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn seed(connection: &Connection) -> StoreResult<()> {
    connection.execute(
        "INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence)
         VALUES('m',?1,'open',0,1,1)",
        [b"objective".as_slice()],
    )?;
    connection.execute(
        "INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,
         wall_time,payload,source_kind,source_id)
         VALUES('e','m',1,'owner',1,'now',?1,'owner','turn')",
        [b"event".as_slice()],
    )?;
    Ok(())
}

#[test]
fn durable_boundaries_fresh_schema_and_pragmas() -> Result<(), Box<dyn Error>> {
    let path = path("fresh")?;
    let connection = native_schema::open(&path)?;
    let mut expected = NATIVE_TABLES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(names(&connection)?, expected);
    assert_eq!(expected.len(), 18);
    let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let foreign_keys: i64 = connection.query_row("PRAGMA foreign_keys", [], |row| row.get(0))?;
    let timeout: i64 = connection.query_row("PRAGMA busy_timeout", [], |row| row.get(0))?;
    let journal: String = connection.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
    let violations: i64 =
        connection.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |row| {
            row.get(0)
        })?;
    assert_eq!(
        (version, foreign_keys, timeout),
        (NATIVE_SCHEMA_VERSION, 1, 5000)
    );
    assert_eq!((journal.as_str(), violations), ("wal", 0));
    let schema: String = connection.query_row(
        "SELECT group_concat(sql,' ') FROM sqlite_schema",
        [],
        |row| row.get(0),
    )?;
    for retired in [
        "tasks",
        "steps",
        "templates",
        "plans",
        "bridges",
        "state_edges",
        "search",
    ] {
        assert!(!schema.to_lowercase().contains(retired));
    }
    for secret in ["secret", "password", "api_key", "token_value"] {
        assert!(!schema.to_lowercase().contains(secret));
    }
    drop(connection);
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn durable_boundaries_exact_specs_and_unknown_cells() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open_in_memory()?;
    native_schema::setup(&connection)?;
    seed(&connection)?;
    let specs: [&[u8]; 8] = [
        b"selection\0",
        b"context\xff",
        b"tool\0",
        b"grammar",
        b"budget",
        b"recovery",
        b"exit",
        b"attachments\0\xff",
    ];
    connection.execute(
        "INSERT INTO runtime_decisions(id,matter_id,event_id,operation_key,idempotency_key,
         selected_monotonic_ms,selected_state,context_spec,tool_spec,grammar_spec,budget_spec,
         recovery_spec,check_spec,exit_spec,compiler_status,compiler_attachments,rendered_frame,status)
         VALUES('d','m','e',?1,?2,2,?3,?4,?5,?6,?7,?8,?9,x'0b','complete',?10,x'01','selected')",
        params![
            b"operation",
            b"idem",
            specs[0],
            specs[1],
            specs[2],
            specs[3],
            specs[4],
            specs[5],
            specs[6],
            specs[7]
        ],
    )?;
    let stored: (Vec<u8>, Vec<u8>) = connection.query_row(
        "SELECT context_spec,compiler_attachments FROM runtime_decisions WHERE id='d'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(stored, (specs[1].to_vec(), specs[7].to_vec()));
    let namespace = b"vendor\0namespace\xff";
    let payload = b"\x00\xffnot-json";
    connection.execute(
        "INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,
         source_event_id,fingerprint) VALUES('m',?1,?2,?3,'active','e',?4)",
        params![namespace, b"key\0", payload, b"fingerprint"],
    )?;
    let round_trip: (Vec<u8>, Vec<u8>) =
        connection.query_row("SELECT namespace,payload FROM state_cells", [], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
    assert_eq!(round_trip, (namespace.to_vec(), payload.to_vec()));
    Ok(())
}

#[test]
fn durable_boundaries_legacy_is_unchanged() -> Result<(), Box<dyn Error>> {
    let path = path("legacy")?;
    let legacy = Connection::open(&path)?;
    legacy.execute_batch("CREATE TABLE tasks(id INTEGER); PRAGMA user_version=77;")?;
    drop(legacy);
    let before = fs::read(&path)?;
    let error = match native_schema::open(&path) {
        Ok(_) => return Err("legacy schema was accepted".into()),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        StoreError::IncompatibleSchema { version: 77, .. }
    ));
    assert_eq!(fs::read(&path)?, before);
    let legacy = Connection::open(&path)?;
    assert_eq!(names(&legacy)?, vec!["tasks"]);
    drop(legacy);
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn durable_boundaries_fault_and_lock_are_typed() -> Result<(), Box<dyn Error>> {
    let limited = Connection::open_in_memory()?;
    limited.execute_batch("PRAGMA max_page_count=1")?;
    assert!(native_schema::setup(&limited).is_err());
    assert!(names(&limited)?.is_empty());
    let version: i64 = limited.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    assert_eq!(version, 0);
    let path = path("busy")?;
    let holder = Connection::open(&path)?;
    holder.execute_batch("BEGIN EXCLUSIVE")?;
    assert!(matches!(
        native_schema::open(&path),
        Err(StoreError::Busy(_))
    ));
    holder.execute_batch("ROLLBACK")?;
    drop(holder);
    fs::remove_file(path)?;
    Ok(())
}
