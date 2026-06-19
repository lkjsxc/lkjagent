use std::fs;
use std::path::Path;
use std::path::PathBuf;

use rusqlite::Connection;

use crate::error::CliError;

pub fn store_path(data_dir: &Path) -> PathBuf {
    data_dir.join("lkjagent.sqlite3")
}

pub fn open_store(data_dir: &Path) -> Result<Connection, CliError> {
    fs::create_dir_all(data_dir)?;
    let conn = Connection::open(store_path(data_dir))?;
    lkjagent_store::schema::setup(&conn)?;
    Ok(conn)
}

pub fn now_stamp() -> String {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".to_string(),
    }
}
