use rusqlite::{params, Connection};

use super::projection::{AssemblyRunInput, EventInput, ReadinessInput};
use super::write::join;
use crate::error::StoreResult;

pub fn record_event(conn: &Connection, input: &EventInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO artifact_atom_events
         (plan_id, atom_id, event_kind, summary, measured_count, weak_classes,
          contract_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.plan_id,
            input.atom_id,
            input.event_kind,
            input.summary,
            input.measured_count,
            join(input.weak_classes),
            input.contract_id,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_assembly_run(conn: &Connection, input: &AssemblyRunInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT OR REPLACE INTO artifact_assembly_runs
         (run_id, plan_id, source_atom_ids, target_paths, status, measured_count,
          summary, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.run_id,
            input.plan_id,
            join(input.source_atom_ids),
            join(input.target_paths),
            input.status,
            input.measured_count,
            input.summary,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn upsert_readiness(conn: &Connection, input: &ReadinessInput<'_>) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO artifact_readiness
         (plan_id, status, atom_total, atom_ready, atom_missing, next_atom_id,
          next_path, active_contract_id, measured_total, accepted_floor,
          assembly_pending, completion_blockers, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
         ON CONFLICT(plan_id) DO UPDATE SET
          status = excluded.status,
          atom_total = excluded.atom_total,
          atom_ready = excluded.atom_ready,
          atom_missing = excluded.atom_missing,
          next_atom_id = excluded.next_atom_id,
          next_path = excluded.next_path,
          active_contract_id = excluded.active_contract_id,
          measured_total = excluded.measured_total,
          accepted_floor = excluded.accepted_floor,
          assembly_pending = excluded.assembly_pending,
          completion_blockers = excluded.completion_blockers,
          updated_at = excluded.updated_at",
        params![
            input.plan_id,
            input.status,
            input.atom_total,
            input.atom_ready,
            input.atom_missing,
            input.next_atom_id,
            input.next_path,
            input.active_contract_id,
            input.measured_total,
            input.accepted_floor,
            input.assembly_pending,
            join(input.completion_blockers),
            input.updated_at,
        ],
    )?;
    Ok(())
}
