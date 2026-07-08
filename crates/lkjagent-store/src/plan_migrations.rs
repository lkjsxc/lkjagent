use rusqlite::Connection;

use crate::error::StoreResult;

pub fn migrate(conn: &Connection) -> StoreResult<()> {
    ensure_column(conn, "queue", "force_new", "INTEGER NOT NULL DEFAULT 0")?;
    ensure_column(conn, "queue", "delivered_at", "TEXT")?;
    ensure_column(conn, "queue", "task_id", "INTEGER")?;
    ensure_column(conn, "queue", "route_lane", "TEXT")?;
    ensure_column(conn, "queue", "route_durability", "TEXT")?;
    ensure_column(conn, "queue", "route_title_seed", "TEXT")?;
    ensure_column(conn, "queue", "route_transform_allowed", "INTEGER")
}

pub fn migrate_checks(conn: &Connection) -> StoreResult<()> {
    ensure_column(conn, "check_results", "decision_id", "TEXT")?;
    ensure_column(conn, "check_results", "evidence_fingerprint", "TEXT")?;
    ensure_column(
        conn,
        "check_results",
        "artifact_refs_json",
        "TEXT NOT NULL DEFAULT '[]'",
    )
}

fn ensure_column(conn: &Connection, table: &str, column: &str, spec: &str) -> StoreResult<()> {
    if has_column(conn, table, column)? {
        return Ok(());
    }
    conn.execute(
        &format!("ALTER TABLE {table} ADD COLUMN {column} {spec}"),
        [],
    )?;
    Ok(())
}

fn has_column(conn: &Connection, table: &str, column: &str) -> StoreResult<bool> {
    let mut statement = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let rows = statement.query_map([], |row| row.get::<_, String>(1))?;
    for row in rows {
        if row? == column {
            return Ok(true);
        }
    }
    Ok(false)
}
