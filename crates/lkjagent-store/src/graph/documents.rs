use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDocumentRow {
    pub case_id: i64,
    pub root: String,
    pub kind: String,
    pub count_target: Option<i64>,
    pub count_mode: String,
    pub topology_status: String,
    pub audit_status: String,
}

pub fn upsert_document(conn: &Connection, row: &GraphDocumentRow, now: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO graph_document_state
         (case_id, root, kind, count_target, count_mode, topology_status,
          audit_status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
         ON CONFLICT(case_id) DO UPDATE SET
          root = excluded.root,
          kind = excluded.kind,
          count_target = excluded.count_target,
          count_mode = excluded.count_mode,
          topology_status = excluded.topology_status,
          audit_status = excluded.audit_status,
          updated_at = excluded.updated_at",
        params![
            row.case_id,
            row.root,
            row.kind,
            row.count_target,
            row.count_mode,
            row.topology_status,
            row.audit_status,
            now
        ],
    )?;
    Ok(())
}
