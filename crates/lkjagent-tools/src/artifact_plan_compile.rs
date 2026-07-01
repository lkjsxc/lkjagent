use lkjagent_store::artifact_graph::{AtomInput, EdgeInput, PlanInput, ReadinessInput};
use rusqlite::Connection;

use crate::artifact_objective::ObjectiveFrame;
use crate::artifact_profile::ProfileSpec;
use crate::error::ToolResult;

pub struct CompiledPlan {
    pub atom_count: usize,
    pub required_atoms: Vec<String>,
}

pub fn compile(
    conn: &Connection,
    frame: &ObjectiveFrame,
    profile: &ProfileSpec,
    case_id: i64,
    now: &str,
) -> ToolResult<CompiledPlan> {
    let artifact_id = artifact_id(case_id, frame, profile);
    let plan_id = lkjagent_store::artifact_graph::upsert_plan(
        conn,
        &PlanInput {
            case_id,
            artifact_id: &artifact_id,
            owner_objective: &frame.raw_text,
            artifact_kind: &frame.artifact_kind,
            root: &frame.root,
            profile: &profile.name,
            normalized_title: &frame.normalized_title,
            measurement_kind: &frame.measurement_kind,
            requested_total: frame.requested_total as i64,
            accepted_floor: frame.accepted_floor as i64,
            section_count: frame.section_count as i64,
            language_hint: &frame.language_hint,
            forbidden_roots: &frame.forbidden_roots,
            status: "planned",
        },
        now,
    )?;
    let atoms = atom_inputs(plan_id, &frame.root, profile, now);
    let edges = edge_inputs(plan_id, &frame.root, profile);
    let required_atoms = atoms
        .iter()
        .map(|atom| atom.path.to_string())
        .collect::<Vec<_>>();
    lkjagent_store::artifact_graph::replace_atoms(conn, plan_id, &atoms)?;
    lkjagent_store::artifact_graph::replace_edges(conn, plan_id, &edges)?;
    let blockers = vec!["atoms-not-audited".to_string()];
    lkjagent_store::artifact_graph::upsert_readiness(
        conn,
        &ReadinessInput {
            plan_id,
            status: "planned",
            atom_total: atoms.len() as i64,
            atom_ready: 0,
            atom_missing: atoms.len() as i64,
            next_atom_id: atoms.first().map_or("none", |atom| atom.atom_id.as_str()),
            next_path: atoms.first().map_or("none", |atom| atom.path),
            active_contract_id: "none",
            measured_total: 0,
            accepted_floor: frame.accepted_floor as i64,
            assembly_pending: "false",
            completion_blockers: &blockers,
            updated_at: now,
        },
    )?;
    Ok(CompiledPlan {
        atom_count: atoms.len(),
        required_atoms,
    })
}

fn atom_inputs<'a>(
    plan_id: i64,
    root: &'a str,
    profile: &'a ProfileSpec,
    _now: &'a str,
) -> Vec<AtomInput<'a>> {
    profile
        .atoms
        .iter()
        .enumerate()
        .map(|(index, atom)| AtomInput {
            plan_id,
            atom_id: atom_id(root, &atom.path),
            sequence: index as i64,
            role: &atom.role,
            path: &atom.path,
            status: "planned",
            measurement_kind: &atom.measurement_kind,
            target_count: atom.target_count as i64,
            count_floor: atom.count_floor as i64,
            measured_count: 0,
            byte_budget: atom.byte_budget as i64,
            required_sections: &atom.required_sections,
            weak_classes: &profile.weak_classes,
            assembly_target: atom.assembly_target.as_deref().unwrap_or(""),
        })
        .collect()
}

fn edge_inputs<'a>(plan_id: i64, root: &'a str, profile: &'a ProfileSpec) -> Vec<EdgeInput<'a>> {
    profile
        .atoms
        .iter()
        .flat_map(|atom| {
            atom.depends_on.iter().map(|dep| EdgeInput {
                plan_id,
                from_atom_id: atom_id(root, dep),
                to_atom_id: atom_id(root, &atom.path),
                relation: "depends-on",
            })
        })
        .collect()
}

fn artifact_id(case_id: i64, frame: &ObjectiveFrame, profile: &ProfileSpec) -> String {
    format!(
        "{case_id}:{}:{}:{}",
        profile.name,
        frame.root.replace('/', ":"),
        frame.accepted_floor
    )
}

fn atom_id(root: &str, path: &str) -> String {
    format!("{}:{}", root.trim_end_matches('/'), path)
}
