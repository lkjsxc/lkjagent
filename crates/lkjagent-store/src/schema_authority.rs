use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS runtime_snapshots (
            id INTEGER PRIMARY KEY,
            case_scope TEXT NOT NULL,
            case_id INTEGER,
            queue_head INTEGER,
            queue_pending_count INTEGER NOT NULL,
            owner_objective TEXT NOT NULL,
            active_mode TEXT NOT NULL,
            active_node TEXT NOT NULL,
            missing_evidence TEXT NOT NULL,
            artifact_head TEXT,
            fault_head TEXT,
            compaction_head TEXT,
            maintenance_state TEXT NOT NULL,
            prompt_frame_id TEXT,
            context_frame_id TEXT,
            staleness_fingerprint TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_authority_events (
            id INTEGER PRIMARY KEY,
            snapshot_id INTEGER,
            case_scope TEXT NOT NULL,
            case_id INTEGER,
            event_kind TEXT NOT NULL,
            event_payload TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_authority_decisions (
            id INTEGER PRIMARY KEY,
            snapshot_id INTEGER,
            case_scope TEXT NOT NULL,
            case_id INTEGER,
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
            staleness_fingerprint TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_transitions (
            id INTEGER PRIMARY KEY,
            snapshot_id INTEGER NOT NULL,
            event_id INTEGER NOT NULL,
            decision_id INTEGER NOT NULL,
            case_scope TEXT NOT NULL,
            case_id INTEGER,
            from_node TEXT NOT NULL,
            to_node TEXT NOT NULL,
            transition_kind TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_effects (
            id INTEGER PRIMARY KEY,
            decision_id INTEGER NOT NULL,
            admission_id INTEGER,
            effect_kind TEXT NOT NULL,
            effect_summary TEXT NOT NULL,
            observation_event_id INTEGER,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_tool_admissions (
            id INTEGER PRIMARY KEY,
            decision_id INTEGER NOT NULL,
            case_scope TEXT NOT NULL,
            case_id INTEGER,
            requested_tool TEXT NOT NULL,
            admitted INTEGER NOT NULL,
            refusal_reason TEXT NOT NULL,
            exact_valid_example TEXT,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS runtime_snapshots_case_idx
            ON runtime_snapshots(case_scope, case_id, id);
        CREATE INDEX IF NOT EXISTS runtime_events_case_idx
            ON runtime_authority_events(case_scope, case_id, id);
        CREATE INDEX IF NOT EXISTS runtime_decisions_case_idx
            ON runtime_authority_decisions(case_scope, case_id, id);
        CREATE INDEX IF NOT EXISTS runtime_admissions_decision_tool_idx
            ON runtime_tool_admissions(decision_id, requested_tool);
        CREATE INDEX IF NOT EXISTS runtime_transitions_case_idx
            ON runtime_transitions(case_scope, case_id, id);
        ",
    )?;
    Ok(())
}
