use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS state_edges (
            id TEXT PRIMARY KEY,
            scope TEXT NOT NULL,
            case_id TEXT,
            from_ref_kind TEXT NOT NULL,
            from_ref_id TEXT NOT NULL,
            to_ref_kind TEXT NOT NULL,
            to_ref_id TEXT NOT NULL,
            relation TEXT NOT NULL,
            reason TEXT NOT NULL,
            evidence_json TEXT NOT NULL,
            edge_json TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            source_event_id TEXT NOT NULL,
            suppression_reason TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_state_edges_scope_status
            ON state_edges(scope, status, relation);
        CREATE INDEX IF NOT EXISTS idx_state_edges_refs
            ON state_edges(from_ref_kind, from_ref_id, to_ref_kind, to_ref_id);
        ",
    )?;
    Ok(())
}
