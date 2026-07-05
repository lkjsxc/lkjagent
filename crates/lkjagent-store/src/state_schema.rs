use rusqlite::Connection;

use crate::error::StoreResult;

pub const STATE_LEDGER_TABLES: &[&str] = &[
    "cases",
    "runtime_events",
    "state_cells",
    "state_history",
    "runtime_decisions",
    "prompt_frames",
    "tool_admissions",
    "observations",
    "context_items",
    "context_edges",
    "state_edges",
    "workspace_records",
    "workspace_record_history",
    "artifacts",
    "provider_exchanges",
];

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS cases (
            id TEXT PRIMARY KEY,
            objective TEXT NOT NULL,
            lifecycle TEXT NOT NULL,
            summary TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_events (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            kind TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            source TEXT NOT NULL,
            decision_id TEXT,
            event_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(case_id) REFERENCES cases(id)
        );
        CREATE TABLE IF NOT EXISTS state_cells (
            case_id TEXT NOT NULL,
            key_label TEXT NOT NULL,
            namespace TEXT NOT NULL,
            name TEXT NOT NULL,
            status TEXT NOT NULL,
            priority INTEGER NOT NULL,
            confidence INTEGER NOT NULL,
            payload_schema TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            evidence_json TEXT NOT NULL,
            source_event_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            expires_at TEXT,
            cooldown_until TEXT,
            conflict_group TEXT,
            parent_key TEXT,
            cell_json TEXT NOT NULL,
            PRIMARY KEY(case_id, key_label),
            FOREIGN KEY(case_id) REFERENCES cases(id)
        );
        CREATE TABLE IF NOT EXISTS state_history (
            id INTEGER PRIMARY KEY,
            case_id TEXT NOT NULL,
            event_id TEXT NOT NULL,
            key_label TEXT NOT NULL,
            patch_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS runtime_decisions (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            operation_key TEXT NOT NULL,
            status TEXT NOT NULL,
            snapshot_fingerprint TEXT NOT NULL,
            state_vector_fingerprint TEXT NOT NULL,
            context_frame_fingerprint TEXT NOT NULL,
            tool_view_fingerprint TEXT NOT NULL,
            expected_envelope TEXT NOT NULL,
            model_budget_tokens INTEGER,
            evidence_requirements_json TEXT NOT NULL,
            recovery_policy TEXT NOT NULL,
            decision_json TEXT NOT NULL,
            selected_at TEXT NOT NULL,
            settled_at TEXT,
            FOREIGN KEY(case_id) REFERENCES cases(id)
        );
        ",
    )?;
    setup_tail(conn)
}

fn setup_tail(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS prompt_frames (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            decision_id TEXT NOT NULL,
            prompt_fingerprint TEXT NOT NULL,
            context_frame_fingerprint TEXT NOT NULL,
            tool_view_fingerprint TEXT NOT NULL,
            body_ref TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS tool_admissions (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            decision_id TEXT NOT NULL,
            tool_view_fingerprint TEXT NOT NULL,
            action_tool TEXT NOT NULL,
            status TEXT NOT NULL,
            parsed_action_json TEXT NOT NULL,
            result_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS observations (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            decision_id TEXT NOT NULL,
            admission_id TEXT,
            effect_name TEXT NOT NULL,
            status TEXT NOT NULL,
            content TEXT NOT NULL,
            artifact_refs_json TEXT NOT NULL,
            contamination_class TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS context_items (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            semantic_key TEXT NOT NULL,
            body TEXT NOT NULL,
            source_type TEXT NOT NULL,
            source_id TEXT NOT NULL,
            source_fingerprint TEXT NOT NULL,
            trust_class TEXT NOT NULL,
            staleness_class TEXT NOT NULL,
            contamination_class TEXT NOT NULL,
            artifact_refs_json TEXT NOT NULL,
            decision_id TEXT,
            item_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            suppression_reason TEXT
        );
        ",
    )?;
    setup_indexes(conn)
}

fn setup_indexes(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS context_edges (
            id INTEGER PRIMARY KEY,
            case_id TEXT NOT NULL,
            from_item_id TEXT NOT NULL,
            to_item_id TEXT NOT NULL,
            edge_kind TEXT NOT NULL,
            reason TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
CREATE TABLE IF NOT EXISTS artifacts (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            kind TEXT NOT NULL,
            path TEXT NOT NULL,
            fingerprint TEXT NOT NULL,
            parent_artifact_id TEXT,
            metadata_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS provider_exchanges (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            decision_id TEXT NOT NULL,
            exchange_ref TEXT NOT NULL,
            outcome_json TEXT NOT NULL,
            context_frame_fingerprint TEXT NOT NULL,
            timeout_seconds INTEGER,
            started_at TEXT NOT NULL,
            finished_at TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_state_cells_case_status ON state_cells(case_id, status, priority, conflict_group);
        CREATE INDEX IF NOT EXISTS idx_context_items_prompt ON context_items(case_id, semantic_key, contamination_class, trust_class, source_fingerprint);
        CREATE INDEX IF NOT EXISTS idx_runtime_decisions_unfinished ON runtime_decisions(case_id, status, selected_at);
        ",
    )?;
    crate::state_edge_schema::setup(conn)?;
    crate::record_schema::setup(conn)?;
    Ok(())
}
