use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::error::StoreError;
use lkjagent_store::native_schema;
use lkjagent_store::transactions::{Intake, NativeStore};
use rusqlite::Connection;

fn path(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "lkjagent-{label}-{}-{nonce}.db",
        std::process::id()
    )))
}

#[test]
fn durable_boundaries_reopens_exact_native_schema() -> Result<(), Box<dyn Error>> {
    let path = path("native-reopen")?;
    let connection = native_schema::open(&path)?;
    connection.execute(
        "INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence)
         VALUES('m',x'01','open',0,1,1)",
        [],
    )?;
    drop(connection);

    let reopened = native_schema::open(&path)?;
    let matters: i64 = reopened.query_row("SELECT count(*) FROM matters", [], |row| row.get(0))?;
    assert_eq!(matters, 1);
    drop(reopened);
    fs::remove_file(path)?;
    Ok(())
}

#[test]
#[rustfmt::skip]
fn durable_boundaries_conversation_sequence_continues_after_reopen() -> Result<(), Box<dyn Error>> {
    let path = path("conversation-reopen")?;
    let mut store = NativeStore::open(&path)?;
    let first = Intake { matter: "m1", objective: b"one", turn: "t1", queue_sequence: 1,
        raw_text: b"one", message_fingerprint: b"fp1", event: "e1", event_sequence: 1,
        event_payload: b"one", monotonic_ms: 1, wall_time: "now", obligations: &[], cells: &[] };
    assert_eq!(store.owner_intake(&first)?.sequence, 1);
    drop(store);
    let mut reopened = NativeStore::open(&path)?;
    let second = Intake { matter: "m2", objective: b"two", turn: "t2", queue_sequence: 2,
        raw_text: b"two", message_fingerprint: b"fp2", event: "e2", event_sequence: 1,
        event_payload: b"two", monotonic_ms: 2, wall_time: "now", obligations: &[], cells: &[] };
    assert_eq!(reopened.owner_intake(&second)?.sequence, 2);
    let page = native_schema::conversation(&Connection::open(&path)?, Some(2), 10)?;
    assert_eq!(page.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), ["owner-turn/t1"]);
    drop(reopened);
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn durable_boundaries_rejects_altered_native_without_mutation() -> Result<(), Box<dyn Error>> {
    let path = path("native-altered")?;
    drop(native_schema::open(&path)?);
    let connection = Connection::open(&path)?;
    connection.execute_batch("ALTER TABLE matters ADD COLUMN rogue BLOB;")?;
    drop(connection);
    let before = fs::read(&path)?;

    let error = match native_schema::open(&path) {
        Ok(_) => return Err("altered native schema was accepted".into()),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        StoreError::IncompatibleSchema { version: 1, .. }
    ));
    assert_eq!(fs::read(&path)?, before);
    fs::remove_file(path)?;
    Ok(())
}
