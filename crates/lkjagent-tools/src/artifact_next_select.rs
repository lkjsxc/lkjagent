use lkjagent_store::artifact_graph::{
    split_lines, AtomRow, ContractInput, PlanRow, ReadinessInput,
};
use rusqlite::Connection;

use crate::artifact_contract_render::render_contract;
use crate::error::ToolResult;

pub fn next_for_plan(conn: &Connection, now: &str, root: &str) -> ToolResult<Option<String>> {
    let Some(plan) = lkjagent_store::artifact_graph::plan_for_root(conn, root)? else {
        return Ok(None);
    };
    if let Some(contract) = lkjagent_store::artifact_graph::active_contract_for_plan(conn, plan.id)?
    {
        let missing = missing_count(conn, plan.id)?;
        return Ok(Some(render_contract(&plan, &contract, missing)));
    }
    let atoms = lkjagent_store::artifact_graph::atoms_for_plan(conn, plan.id)?;
    if let Some(atom) = atoms.iter().find(|atom| writable(atom)) {
        let contract = contract_for(conn, &plan, atom, now)?;
        project_contract(conn, &plan, &atoms, atom, &contract.contract_id, now)?;
        return Ok(Some(render_contract(&plan, &contract, missing(&atoms))));
    }
    if atoms.iter().any(|atom| atom.status == "written") {
        return Ok(Some(audit_candidate(&plan)));
    }
    Ok(Some(readiness_candidate(&plan, &atoms)))
}

fn contract_for(
    conn: &Connection,
    plan: &PlanRow,
    atom: &AtomRow,
    now: &str,
) -> ToolResult<lkjagent_store::artifact_graph::ContractRow> {
    let contract_id = format!(
        "contract-{}-{}",
        plan.id,
        contract_index(conn, plan.id)? + 1
    );
    let atom_ids = vec![atom.atom_id.clone()];
    let paths = vec![full_path(&plan.root, &atom.path)];
    let required = split_lines(&atom.required_sections);
    let weak = split_lines(&atom.weak_classes);
    let continuity = format!("root={} atom={}", plan.root, atom.role);
    lkjagent_store::artifact_graph::create_contract(
        conn,
        &ContractInput {
            contract_id: &contract_id,
            plan_id: plan.id,
            atom_ids: &atom_ids,
            exact_paths: &paths,
            max_files: 1,
            max_file_bytes: atom.byte_budget,
            max_batch_bytes: atom.byte_budget,
            target_count: atom.target_count,
            count_floor: atom.count_floor,
            required_sections: &required,
            continuity_digest: &continuity,
            forbidden_weak_classes: &weak,
            status: "active",
        },
        now,
    )?;
    lkjagent_store::artifact_graph::update_atom_status(
        conn,
        &atom.atom_id,
        "contracted",
        atom.measured_count,
        &weak,
        now,
    )?;
    let active = lkjagent_store::artifact_graph::active_contract_for_plan(conn, plan.id)?;
    active.ok_or_else(|| crate::error::ToolError::invalid("active contract was not persisted"))
}

fn project_contract(
    conn: &Connection,
    plan: &PlanRow,
    atoms: &[AtomRow],
    atom: &AtomRow,
    contract_id: &str,
    now: &str,
) -> ToolResult<()> {
    let blockers = vec![format!("active-contract:{contract_id}")];
    let next_path = full_path(&plan.root, &atom.path);
    lkjagent_store::artifact_graph::upsert_readiness(
        conn,
        &ReadinessInput {
            plan_id: plan.id,
            status: "contracted",
            atom_total: atoms.len() as i64,
            atom_ready: ready_count(atoms) as i64,
            atom_missing: missing(atoms) as i64,
            next_atom_id: &atom.atom_id,
            next_path: &next_path,
            active_contract_id: contract_id,
            measured_total: measured_total(atoms) as i64,
            accepted_floor: plan.accepted_floor,
            assembly_pending: "false",
            completion_blockers: &blockers,
            updated_at: now,
        },
    )?;
    Ok(())
}

fn writable(atom: &AtomRow) -> bool {
    matches!(
        atom.status.as_str(),
        "planned" | "missing" | "weak" | "blocked"
    )
}

fn missing_count(conn: &Connection, plan_id: i64) -> ToolResult<usize> {
    Ok(missing(&lkjagent_store::artifact_graph::atoms_for_plan(
        conn, plan_id,
    )?))
}

fn missing(atoms: &[AtomRow]) -> usize {
    atoms
        .iter()
        .filter(|atom| !matches!(atom.status.as_str(), "ready" | "assembled"))
        .count()
}

fn ready_count(atoms: &[AtomRow]) -> usize {
    atoms
        .iter()
        .filter(|atom| matches!(atom.status.as_str(), "ready" | "assembled"))
        .count()
}

fn measured_total(atoms: &[AtomRow]) -> usize {
    atoms.iter().map(|atom| atom.measured_count as usize).sum()
}

fn contract_index(conn: &Connection, plan_id: i64) -> ToolResult<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM artifact_write_contracts WHERE plan_id = ?1",
        rusqlite::params![plan_id],
        |row| row.get(0),
    )
    .map_err(|error| crate::error::ToolError::Store(error.to_string()))
}

fn audit_candidate(plan: &PlanRow) -> String {
    format!(
        "artifact_next_result=ready_for_audit\nroot={}\nkind={}\nartifact_profile={}\nnext_decision_required=true\ncandidate_action=artifact.audit",
        plan.root, plan.artifact_kind, plan.profile
    )
}

fn readiness_candidate(plan: &PlanRow, atoms: &[AtomRow]) -> String {
    format!(
        "artifact_next_result=readiness_projection\nroot={}\nkind={}\nartifact_profile={}\natom_missing_count={}\nnext_atom=none\nnext_decision_required=true\ncandidate_action=artifact.audit",
        plan.root,
        plan.artifact_kind,
        plan.profile,
        missing(atoms)
    )
}

fn full_path(root: &str, path: &str) -> String {
    format!(
        "{}/{}",
        root.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}
