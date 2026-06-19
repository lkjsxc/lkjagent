#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn temp_data(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-cli-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn open_store(data: &PathBuf) -> TestResult<Connection> {
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    lkjagent_store::schema::setup(&conn)?;
    Ok(conn)
}
