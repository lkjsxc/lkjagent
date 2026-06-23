use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS runtime_authority_events (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            event_kind TEXT NOT NULL,
            event_payload TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_authority_decisions (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            event_id INTEGER NOT NULL,
            mission TEXT NOT NULL,
            active_mode TEXT NOT NULL,
            active_node TEXT NOT NULL,
            admitted_tools TEXT NOT NULL,
            blocked_tools TEXT NOT NULL,
            missing_evidence TEXT NOT NULL,
            forced_next_action TEXT NOT NULL,
            exact_valid_example TEXT,
            completion_allowed INTEGER NOT NULL,
            completion_refusal TEXT,
            recovery_route TEXT,
            compaction_required INTEGER NOT NULL,
            maintenance_allowed INTEGER NOT NULL,
            authority_fingerprint TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_tool_admissions (
            id INTEGER PRIMARY KEY,
            decision_id INTEGER NOT NULL,
            case_id INTEGER NOT NULL,
            requested_tool TEXT NOT NULL,
            admitted INTEGER NOT NULL,
            refusal_reason TEXT NOT NULL,
            exact_valid_example TEXT,
            created_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
