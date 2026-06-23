use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS artifact_ledger (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            artifact_id TEXT NOT NULL UNIQUE,
            root TEXT NOT NULL,
            kind TEXT NOT NULL,
            normalized_topic TEXT NOT NULL,
            requested_scale TEXT NOT NULL,
            profile TEXT NOT NULL,
            lifecycle_state TEXT NOT NULL,
            topology_status TEXT NOT NULL,
            readiness_status TEXT NOT NULL,
            objective_match_status TEXT NOT NULL,
            latest_audit_id TEXT,
            weak_path_count INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_weak_paths (
            id INTEGER PRIMARY KEY,
            artifact_ledger_id INTEGER NOT NULL,
            path TEXT NOT NULL,
            role TEXT NOT NULL,
            missing_requirements TEXT NOT NULL,
            weak_signals TEXT NOT NULL,
            semantic_mismatch TEXT NOT NULL,
            retry_count INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_batch_cursors (
            id INTEGER PRIMARY KEY,
            artifact_ledger_id INTEGER NOT NULL,
            root TEXT NOT NULL,
            planned_paths TEXT NOT NULL,
            completed_paths TEXT NOT NULL,
            failed_paths TEXT NOT NULL,
            current_index INTEGER NOT NULL,
            last_valid_example TEXT NOT NULL,
            retry_counts TEXT NOT NULL,
            fallback_mode TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
