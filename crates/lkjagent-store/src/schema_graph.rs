use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS graph_constraints (
            case_id INTEGER NOT NULL, kind TEXT NOT NULL, summary TEXT NOT NULL,
            source TEXT NOT NULL, strength TEXT NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_assumptions (
            case_id INTEGER NOT NULL, summary TEXT NOT NULL, status TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_questions (
            case_id INTEGER NOT NULL, question TEXT NOT NULL, status TEXT NOT NULL,
            created_at TEXT NOT NULL, resolved_at TEXT
        );
        CREATE TABLE IF NOT EXISTS graph_risks (
            case_id INTEGER NOT NULL, summary TEXT NOT NULL, mitigation TEXT NOT NULL,
            status TEXT NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_success_criteria (
            case_id INTEGER NOT NULL, summary TEXT NOT NULL, status TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_plan_steps (
            case_id INTEGER NOT NULL, step_id TEXT NOT NULL, title TEXT NOT NULL,
            rationale TEXT NOT NULL, status TEXT NOT NULL, node TEXT NOT NULL,
            target_paths TEXT NOT NULL, checks TEXT NOT NULL, sort_order INTEGER NOT NULL,
            created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
            PRIMARY KEY(case_id, step_id)
        );
        CREATE TABLE IF NOT EXISTS graph_decisions (
            case_id INTEGER NOT NULL, node TEXT NOT NULL, summary TEXT NOT NULL,
            reason TEXT NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_context_bindings (
            case_id INTEGER NOT NULL, package TEXT NOT NULL, reason TEXT NOT NULL,
            priority TEXT NOT NULL, compression_level TEXT NOT NULL DEFAULT 'green',
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_transitions (
            case_id INTEGER NOT NULL, from_node TEXT NOT NULL, to_node TEXT NOT NULL,
            decision TEXT NOT NULL, reason TEXT NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_faults (
            case_id INTEGER NOT NULL, kind TEXT NOT NULL, action_fingerprint TEXT,
            summary TEXT NOT NULL, count INTEGER NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_artifacts (
            case_id INTEGER NOT NULL, path TEXT NOT NULL, kind TEXT NOT NULL,
            status TEXT NOT NULL, summary TEXT NOT NULL, created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_document_state (
            case_id INTEGER PRIMARY KEY, root TEXT NOT NULL, kind TEXT NOT NULL,
            count_target INTEGER, count_mode TEXT NOT NULL, topology_status TEXT NOT NULL,
            audit_status TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_compaction_snapshots (
            id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL, phase TEXT NOT NULL,
            active_node TEXT NOT NULL, objective TEXT NOT NULL, preserved_fields TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_recovery_state (
            case_id INTEGER PRIMARY KEY, ladder_position INTEGER NOT NULL,
            strategy TEXT NOT NULL, updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS graph_state_tracks (
            case_id INTEGER NOT NULL, track_id TEXT NOT NULL, label TEXT NOT NULL,
            posture TEXT NOT NULL, intensity INTEGER NOT NULL, confidence INTEGER NOT NULL,
            phase TEXT NOT NULL, active_node TEXT NOT NULL, evidence_gap TEXT NOT NULL,
            next_affordances TEXT NOT NULL, risk TEXT NOT NULL, last_update_turn INTEGER,
            rank_score INTEGER NOT NULL, updated_at TEXT NOT NULL,
            PRIMARY KEY(case_id, track_id)
        );
        ",
    )?;
    ensure_column(
        conn,
        "graph_context_bindings",
        "compression_level",
        "TEXT NOT NULL DEFAULT 'green'",
    )?;
    Ok(())
}

fn ensure_column(
    conn: &Connection,
    table: &str,
    column: &str,
    definition: &str,
) -> StoreResult<()> {
    let mut statement = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let rows = statement.query_map([], |row| row.get::<_, String>(1))?;
    for row in rows {
        if row? == column {
            return Ok(());
        }
    }
    conn.execute(
        &format!("ALTER TABLE {table} ADD COLUMN {column} {definition}"),
        [],
    )?;
    Ok(())
}
