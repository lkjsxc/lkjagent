use std::collections::BTreeSet;

use rusqlite::Connection;

use crate::error::StoreResult;

pub fn application_tables(conn: &Connection) -> StoreResult<BTreeSet<String>> {
    let mut statement = conn.prepare(
        "SELECT name FROM sqlite_master
         WHERE type IN ('table', 'virtual table') AND name NOT LIKE 'sqlite_%'
         ORDER BY name",
    )?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    let mut output = BTreeSet::new();
    for row in rows {
        let name = row?;
        if !name.starts_with("memory_fts_") {
            output.insert(name);
        }
    }
    Ok(output)
}
