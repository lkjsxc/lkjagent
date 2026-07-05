use lkjagent_core::runtime_event::{StatePatch, StatePatchOp};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell};
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::row_json::{json_string, json_value};

pub fn insert_case(
    conn: &Connection,
    id: &str,
    objective: &str,
    created_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT OR IGNORE INTO cases
         (id, objective, lifecycle, summary, created_at, updated_at)
         VALUES (?1, ?2, 'open', '', ?3, ?3)",
        params![id, objective, created_at],
    )?;
    Ok(())
}

pub fn upsert_state_cell(conn: &Connection, case_id: &str, cell: &StateCell) -> StoreResult<()> {
    let key_label = cell.key.as_label();
    let evidence_json = json_string(&cell.evidence_refs)?;
    let parent_key = cell.parent_key.as_ref().map(|key| key.as_label());
    let cell_json = json_string(cell)?;
    conn.execute(
        "INSERT INTO state_cells
         (case_id, key_label, namespace, name, status, priority, confidence,
          payload_schema, payload_json, evidence_json, source_event_id,
          created_at, updated_at, expires_at, cooldown_until, conflict_group,
          parent_key, cell_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                 ?14, ?15, ?16, ?17, ?18)
         ON CONFLICT(case_id, key_label) DO UPDATE SET
          status=excluded.status, priority=excluded.priority,
          confidence=excluded.confidence, payload_schema=excluded.payload_schema,
          payload_json=excluded.payload_json, evidence_json=excluded.evidence_json,
          source_event_id=excluded.source_event_id, updated_at=excluded.updated_at,
          expires_at=excluded.expires_at, cooldown_until=excluded.cooldown_until,
          conflict_group=excluded.conflict_group, parent_key=excluded.parent_key,
          cell_json=excluded.cell_json",
        params![
            case_id,
            key_label,
            cell.key.namespace,
            cell.key.name,
            format!("{:?}", cell.status),
            cell.priority,
            cell.confidence,
            cell.payload_schema,
            cell.payload_json,
            evidence_json,
            cell.source_event_id,
            cell.created_at,
            cell.updated_at,
            cell.expires_at,
            cell.cooldown_until,
            cell.conflict_group,
            parent_key,
            cell_json,
        ],
    )?;
    conn.execute(
        "INSERT INTO state_history
         (case_id, event_id, key_label, patch_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            case_id,
            cell.source_event_id,
            key_label,
            cell_json,
            cell.updated_at,
        ],
    )?;
    Ok(())
}

pub fn persist_state_patch(
    conn: &Connection,
    case_id: &str,
    patch: &StatePatch,
) -> StoreResult<()> {
    for operation in &patch.operations {
        match operation {
            StatePatchOp::Upsert(cell) => upsert_state_cell(conn, case_id, cell)?,
            StatePatchOp::SetStatus {
                key,
                status,
                updated_at,
                source_event_id,
                ..
            } => {
                let key_label = key.as_label();
                conn.execute(
                    "UPDATE state_cells SET status = ?1, updated_at = ?2,
                     source_event_id = ?3 WHERE case_id = ?4 AND key_label = ?5",
                    params![
                        format!("{:?}", status),
                        updated_at,
                        source_event_id,
                        case_id,
                        key_label,
                    ],
                )?;
                conn.execute(
                    "INSERT INTO state_history
                     (case_id, event_id, key_label, patch_json, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        case_id,
                        patch.event_id,
                        key_label,
                        json_string(operation)?,
                        updated_at,
                    ],
                )?;
            }
            StatePatchOp::UpsertEdge(edge) => {
                crate::state_edge_rows::insert_state_edge(conn, Some(case_id), edge)?;
            }
            StatePatchOp::SetEdgeStatus {
                edge_id, reason, ..
            } => {
                crate::state_edge_rows::suppress_state_edge(conn, edge_id, reason)?;
            }
        }
    }
    Ok(())
}

pub fn hydrate_snapshot(conn: &Connection, case_id: &str) -> StoreResult<RuntimeSnapshot> {
    let mut statement = conn.prepare(
        "SELECT cell_json FROM state_cells
         WHERE case_id = ?1 AND status = 'Active' ORDER BY key_label",
    )?;
    let rows = statement.query_map([case_id], |row| row.get::<_, String>(0))?;
    let mut snapshot = RuntimeSnapshot::empty(case_id);
    for row in rows {
        let cell: StateCell = json_value(&row?)?;
        snapshot.cells.insert(cell.key.clone(), cell);
    }
    for edge in crate::state_edge_rows::state_edges(conn, case_id)? {
        snapshot.edges.insert(edge.id.clone(), edge);
    }
    Ok(snapshot)
}
