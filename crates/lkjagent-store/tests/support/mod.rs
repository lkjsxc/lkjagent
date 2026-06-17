use rusqlite::Connection;

use lkjagent_store::schema::setup;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn memory_store() -> TestResult<Connection> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    Ok(conn)
}
