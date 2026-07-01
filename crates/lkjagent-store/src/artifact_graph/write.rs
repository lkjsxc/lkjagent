use rusqlite::{params, Connection};

use super::model::{AtomInput, ContractInput, EdgeInput, PlanInput};
use crate::error::StoreResult;

pub fn upsert_plan(conn: &Connection, input: &PlanInput<'_>, now: &str) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO artifact_plans
         (case_id, artifact_id, owner_objective, artifact_kind, root, profile,
          normalized_title, measurement_kind, requested_total, accepted_floor,
          section_count, language_hint, forbidden_roots, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?15)
         ON CONFLICT(artifact_id) DO UPDATE SET
          owner_objective = excluded.owner_objective,
          artifact_kind = excluded.artifact_kind,
          root = excluded.root,
          profile = excluded.profile,
          normalized_title = excluded.normalized_title,
          measurement_kind = excluded.measurement_kind,
          requested_total = excluded.requested_total,
          accepted_floor = excluded.accepted_floor,
          section_count = excluded.section_count,
          language_hint = excluded.language_hint,
          forbidden_roots = excluded.forbidden_roots,
          status = excluded.status,
          updated_at = excluded.updated_at",
        params![
            input.case_id,
            input.artifact_id,
            input.owner_objective,
            input.artifact_kind,
            input.root,
            input.profile,
            input.normalized_title,
            input.measurement_kind,
            input.requested_total,
            input.accepted_floor,
            input.section_count,
            input.language_hint,
            join(input.forbidden_roots),
            input.status,
            now,
        ],
    )?;
    plan_id(conn, input.artifact_id)
}

pub fn replace_atoms(conn: &Connection, plan_id: i64, atoms: &[AtomInput<'_>]) -> StoreResult<()> {
    conn.execute(
        "DELETE FROM artifact_atoms WHERE plan_id = ?1",
        params![plan_id],
    )?;
    for atom in atoms {
        conn.execute(
            "INSERT INTO artifact_atoms
             (plan_id, atom_id, sequence, role, path, status, measurement_kind,
              target_count, count_floor, measured_count, byte_budget,
              required_sections, weak_classes, assembly_target, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, '')",
            params![
                atom.plan_id,
                atom.atom_id,
                atom.sequence,
                atom.role,
                atom.path,
                atom.status,
                atom.measurement_kind,
                atom.target_count,
                atom.count_floor,
                atom.measured_count,
                atom.byte_budget,
                join(atom.required_sections),
                join(atom.weak_classes),
                atom.assembly_target,
            ],
        )?;
    }
    Ok(())
}

pub fn replace_edges(conn: &Connection, plan_id: i64, edges: &[EdgeInput<'_>]) -> StoreResult<()> {
    conn.execute(
        "DELETE FROM artifact_atom_edges WHERE plan_id = ?1",
        params![plan_id],
    )?;
    for edge in edges {
        conn.execute(
            "INSERT INTO artifact_atom_edges (plan_id, from_atom_id, to_atom_id, relation)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                edge.plan_id,
                edge.from_atom_id,
                edge.to_atom_id,
                edge.relation
            ],
        )?;
    }
    Ok(())
}

pub fn create_contract(
    conn: &Connection,
    input: &ContractInput<'_>,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "UPDATE artifact_write_contracts SET status = 'superseded', updated_at = ?1
         WHERE plan_id = ?2 AND status = 'active'",
        params![now, input.plan_id],
    )?;
    conn.execute(
        "INSERT INTO artifact_write_contracts
         (contract_id, plan_id, atom_ids, exact_paths, max_files, max_file_bytes,
          max_batch_bytes, target_count, count_floor, required_sections,
          continuity_digest, forbidden_weak_classes, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?14)",
        params![
            input.contract_id,
            input.plan_id,
            join(input.atom_ids),
            join(input.exact_paths),
            input.max_files,
            input.max_file_bytes,
            input.max_batch_bytes,
            input.target_count,
            input.count_floor,
            join(input.required_sections),
            input.continuity_digest,
            join(input.forbidden_weak_classes),
            input.status,
            now,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_atom_status(
    conn: &Connection,
    atom_id: &str,
    status: &str,
    measured_count: i64,
    weak_classes: &[String],
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE artifact_atoms SET status = ?1, measured_count = ?2,
         weak_classes = ?3, updated_at = ?4 WHERE atom_id = ?5",
        params![status, measured_count, join(weak_classes), now, atom_id],
    )?;
    Ok(())
}

pub fn set_contract_status(
    conn: &Connection,
    contract_id: &str,
    status: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE artifact_write_contracts SET status = ?1, updated_at = ?2 WHERE contract_id = ?3",
        params![status, now, contract_id],
    )?;
    Ok(())
}

fn plan_id(conn: &Connection, artifact_id: &str) -> StoreResult<i64> {
    Ok(conn.query_row(
        "SELECT id FROM artifact_plans WHERE artifact_id = ?1",
        params![artifact_id],
        |row| row.get(0),
    )?)
}

pub(crate) fn join(values: &[String]) -> String {
    values.join("\n")
}
