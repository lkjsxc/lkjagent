use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::error::StoreError;
use lkjagent_store::native_schema;
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
