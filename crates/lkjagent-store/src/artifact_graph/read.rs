use rusqlite::{params, Connection, OptionalExtension};

use super::model::{AtomRow, ContractRow, PlanRow};
use super::projection::ReadinessRow;
use crate::error::StoreResult;

pub fn latest_plan_for_case(conn: &Connection, case_id: i64) -> StoreResult<Option<PlanRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, artifact_id, artifact_kind, root, profile, status, accepted_floor
         FROM artifact_plans WHERE case_id = ?1 ORDER BY updated_at DESC, id DESC LIMIT 1",
    )?;
    statement
        .query_row(params![case_id], plan_row)
        .optional()
        .map_err(Into::into)
}

pub fn plan_for_root(conn: &Connection, root: &str) -> StoreResult<Option<PlanRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, artifact_id, artifact_kind, root, profile, status, accepted_floor
         FROM artifact_plans WHERE root = ?1 ORDER BY updated_at DESC, id DESC LIMIT 1",
    )?;
    statement
        .query_row(params![root], plan_row)
        .optional()
        .map_err(Into::into)
}

pub fn atoms_for_plan(conn: &Connection, plan_id: i64) -> StoreResult<Vec<AtomRow>> {
    let mut statement = conn.prepare(
        "SELECT id, atom_id, sequence, role, path, status, measurement_kind,
         target_count, count_floor, measured_count, byte_budget, required_sections,
         weak_classes, assembly_target FROM artifact_atoms
         WHERE plan_id = ?1 ORDER BY sequence, id",
    )?;
    let rows = statement.query_map(params![plan_id], atom_row)?;
    collect(rows)
}

pub fn active_contract_for_plan(
    conn: &Connection,
    plan_id: i64,
) -> StoreResult<Option<ContractRow>> {
    let mut statement = conn.prepare(
        "SELECT id, contract_id, plan_id, atom_ids, exact_paths, max_files,
         max_file_bytes, max_batch_bytes, target_count, count_floor,
         required_sections, continuity_digest, forbidden_weak_classes, status
         FROM artifact_write_contracts WHERE plan_id = ?1 AND status = 'active'
         ORDER BY updated_at DESC, id DESC LIMIT 1",
    )?;
    statement
        .query_row(params![plan_id], contract_row)
        .optional()
        .map_err(Into::into)
}

pub fn active_contracts(conn: &Connection) -> StoreResult<Vec<ContractRow>> {
    let mut statement = conn.prepare(
        "SELECT id, contract_id, plan_id, atom_ids, exact_paths, max_files,
         max_file_bytes, max_batch_bytes, target_count, count_floor,
         required_sections, continuity_digest, forbidden_weak_classes, status
         FROM artifact_write_contracts WHERE status = 'active' ORDER BY id",
    )?;
    let rows = statement.query_map([], contract_row)?;
    collect(rows)
}

pub fn readiness_for_plan(conn: &Connection, plan_id: i64) -> StoreResult<Option<ReadinessRow>> {
    let mut statement = conn.prepare(
        "SELECT r.plan_id, p.root, p.profile, p.status, r.status, r.atom_total,
         r.atom_ready, r.atom_missing, r.next_atom_id, r.next_path,
         r.active_contract_id, r.measured_total, r.accepted_floor,
         r.assembly_pending, r.completion_blockers
         FROM artifact_readiness r JOIN artifact_plans p ON p.id = r.plan_id
         WHERE r.plan_id = ?1",
    )?;
    statement
        .query_row(params![plan_id], readiness_row)
        .optional()
        .map_err(Into::into)
}

pub fn readiness_for_case(conn: &Connection, case_id: i64) -> StoreResult<Option<ReadinessRow>> {
    let Some(plan) = latest_plan_for_case(conn, case_id)? else {
        return Ok(None);
    };
    readiness_for_plan(conn, plan.id)
}

fn collect<T, F>(rows: rusqlite::MappedRows<'_, F>) -> StoreResult<Vec<T>>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn plan_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanRow> {
    Ok(PlanRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        artifact_id: row.get(2)?,
        artifact_kind: row.get(3)?,
        root: row.get(4)?,
        profile: row.get(5)?,
        status: row.get(6)?,
        accepted_floor: row.get(7)?,
    })
}

fn atom_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AtomRow> {
    Ok(AtomRow {
        id: row.get(0)?,
        atom_id: row.get(1)?,
        sequence: row.get(2)?,
        role: row.get(3)?,
        path: row.get(4)?,
        status: row.get(5)?,
        measurement_kind: row.get(6)?,
        target_count: row.get(7)?,
        count_floor: row.get(8)?,
        measured_count: row.get(9)?,
        byte_budget: row.get(10)?,
        required_sections: row.get(11)?,
        weak_classes: row.get(12)?,
        assembly_target: row.get(13)?,
    })
}

fn contract_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ContractRow> {
    Ok(ContractRow {
        id: row.get(0)?,
        contract_id: row.get(1)?,
        plan_id: row.get(2)?,
        atom_ids: row.get(3)?,
        exact_paths: row.get(4)?,
        max_files: row.get(5)?,
        max_file_bytes: row.get(6)?,
        max_batch_bytes: row.get(7)?,
        target_count: row.get(8)?,
        count_floor: row.get(9)?,
        required_sections: row.get(10)?,
        continuity_digest: row.get(11)?,
        forbidden_weak_classes: row.get(12)?,
        status: row.get(13)?,
    })
}

fn readiness_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ReadinessRow> {
    Ok(ReadinessRow {
        plan_id: row.get(0)?,
        root: row.get(1)?,
        profile: row.get(2)?,
        plan_status: row.get(3)?,
        status: row.get(4)?,
        atom_total: row.get(5)?,
        atom_ready: row.get(6)?,
        atom_missing: row.get(7)?,
        next_atom_id: row.get(8)?,
        next_path: row.get(9)?,
        active_contract_id: row.get(10)?,
        measured_total: row.get(11)?,
        accepted_floor: row.get(12)?,
        assembly_pending: row.get(13)?,
        completion_blockers: row.get(14)?,
    })
}
