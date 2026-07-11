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

pub fn migrate_effect_journal(conn: &Connection) -> StoreResult<()> {
    ensure_column(
        conn,
        "effect_journal",
        "command_ordinal",
        "INTEGER NOT NULL DEFAULT 0",
    )?;
    ensure_column(conn, "effect_journal", "observation_id", "TEXT")?;
    ensure_column(conn, "effect_journal", "target_path", "TEXT")?;
    conn.execute(
        "UPDATE effect_journal SET command_ordinal = rowid WHERE command_ordinal = 0",
        [],
    )?;
    conn.execute_batch(
        "CREATE UNIQUE INDEX IF NOT EXISTS effect_journal_decision_ordinal
         ON effect_journal(decision_id, command_ordinal);
         CREATE UNIQUE INDEX IF NOT EXISTS effect_journal_observation
         ON effect_journal(observation_id) WHERE observation_id IS NOT NULL;
         CREATE TABLE IF NOT EXISTS effect_target_revisions (
           journal_id TEXT NOT NULL REFERENCES effect_journal(id),
           target_ordinal INTEGER NOT NULL, role TEXT NOT NULL, path TEXT NOT NULL,
           prior_bytes BLOB, intended_bytes BLOB, prior_fingerprint TEXT NOT NULL,
           intended_fingerprint TEXT NOT NULL, artifacts_json TEXT NOT NULL DEFAULT '[]',
           PRIMARY KEY(journal_id, target_ordinal), UNIQUE(journal_id, path)
         );",
    )?;
    Ok(())
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
