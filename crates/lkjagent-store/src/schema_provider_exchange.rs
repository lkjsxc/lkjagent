use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS provider_exchange (
            id TEXT PRIMARY KEY,
            case_id TEXT NOT NULL,
            turn_id INTEGER NOT NULL,
            prompt_frame_id TEXT,
            authority_decision_id TEXT,
            admission_decision_id TEXT,
            provider TEXT NOT NULL,
            model TEXT NOT NULL,
            created_at TEXT NOT NULL,
            request_json TEXT NOT NULL,
            response_json TEXT,
            request_hash TEXT NOT NULL,
            response_hash TEXT,
            finish_reason TEXT,
            usage_json TEXT,
            stats_json TEXT,
            latency_ms INTEGER,
            status TEXT NOT NULL,
            error_class TEXT,
            redaction_schema_version INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS provider_exchange_case_turn_idx
            ON provider_exchange(case_id, turn_id);
        CREATE INDEX IF NOT EXISTS provider_exchange_created_idx
            ON provider_exchange(created_at);
        ",
    )?;
    Ok(())
}
