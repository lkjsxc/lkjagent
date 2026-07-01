use std::path::Path;

use lkjagent_store::artifact_graph::{AtomRow, EventInput, ReadinessInput};
use rusqlite::Connection;

use crate::artifact_atom_measure::{measure_atom, Measurement};
use crate::error::ToolResult;

pub fn audit_plan(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
) -> ToolResult<Option<String>> {
    let Some(plan) = lkjagent_store::artifact_graph::plan_for_root(conn, root)? else {
        return Ok(None);
    };
    let full_root = crate::fs::workspace_path(workspace, &plan.root)?;
    let assembly =
        crate::artifact_manuscript_assembly::assemble_scene_atoms(&full_root, &plan.artifact_kind)?;
    record_assembly(conn, plan.id, &assembly, now)?;
    let atoms = lkjagent_store::artifact_graph::atoms_for_plan(conn, plan.id)?;
    let mut updated = Vec::new();
    for atom in atoms {
        let measured = measure_atom(&full_root, &atom)?;
        persist_measurement(conn, plan.id, &atom, &measured, now)?;
        updated.push((atom, measured));
    }
    let projection = project(&updated, plan.accepted_floor as usize);
    lkjagent_store::artifact_graph::upsert_readiness(conn, &projection.input(plan.id, now))?;
    Ok(Some(render(&plan, &projection)))
}

fn persist_measurement(
    conn: &Connection,
    plan_id: i64,
    atom: &AtomRow,
    measured: &Measurement,
    now: &str,
) -> ToolResult<()> {
    lkjagent_store::artifact_graph::update_atom_status(
        conn,
        &atom.atom_id,
        &measured.status,
        measured.count as i64,
        &measured.weak_classes,
        now,
    )?;
    lkjagent_store::artifact_graph::record_event(
        conn,
        &EventInput {
            plan_id,
            atom_id: &atom.atom_id,
            event_kind: "audit",
            summary: &measured.summary,
            measured_count: measured.count as i64,
            weak_classes: &measured.weak_classes,
            contract_id: None,
            created_at: now,
        },
    )?;
    Ok(())
}

struct Projection {
    status: String,
    atom_total: usize,
    atom_ready: usize,
    atom_missing: usize,
    next_atom_id: String,
    next_path: String,
    measured_total: usize,
    accepted_floor: usize,
    assembly_pending: bool,
    blockers: Vec<String>,
}

impl Projection {
    fn input<'a>(&'a self, plan_id: i64, now: &'a str) -> ReadinessInput<'a> {
        ReadinessInput {
            plan_id,
            status: &self.status,
            atom_total: self.atom_total as i64,
            atom_ready: self.atom_ready as i64,
            atom_missing: self.atom_missing as i64,
            next_atom_id: &self.next_atom_id,
            next_path: &self.next_path,
            active_contract_id: "none",
            measured_total: self.measured_total as i64,
            accepted_floor: self.accepted_floor as i64,
            assembly_pending: if self.assembly_pending {
                "true"
            } else {
                "false"
            },
            completion_blockers: &self.blockers,
            updated_at: now,
        }
    }
}

fn project(rows: &[(AtomRow, Measurement)], accepted_floor: usize) -> Projection {
    let atom_total = rows.len();
    let atom_ready = rows.iter().filter(|(_, m)| m.status == "ready").count();
    let measured_total = rows.iter().map(|(_, m)| m.count).sum();
    let next = rows.iter().find(|(_, m)| m.status != "ready");
    let blockers = blockers(next, measured_total, accepted_floor);
    Projection {
        status: if blockers.is_empty() {
            "ready"
        } else {
            "blocked"
        }
        .to_string(),
        atom_total,
        atom_ready,
        atom_missing: atom_total.saturating_sub(atom_ready),
        next_atom_id: next
            .map_or("none", |(atom, _)| atom.atom_id.as_str())
            .to_string(),
        next_path: next
            .map_or("none", |(atom, _)| atom.path.as_str())
            .to_string(),
        measured_total,
        accepted_floor,
        assembly_pending: rows
            .iter()
            .any(|(atom, m)| !atom.assembly_target.is_empty() && m.status == "ready"),
        blockers,
    }
}

fn blockers(next: Option<&(AtomRow, Measurement)>, measured: usize, floor: usize) -> Vec<String> {
    let mut out = Vec::new();
    if let Some((atom, measured_atom)) = next {
        out.push(format!(
            "atom:{}:{}",
            atom.path,
            measured_atom.weak_classes.join(",")
        ));
    }
    if measured < floor {
        out.push(format!("count:{measured}/{floor}"));
    }
    out
}

fn record_assembly(
    conn: &Connection,
    plan_id: i64,
    reports: &[crate::artifact_manuscript_assembly::AssemblyReport],
    now: &str,
) -> ToolResult<()> {
    for report in reports {
        let run_id = format!("assembly-{plan_id}-{}", report.target);
        lkjagent_store::artifact_graph::record_assembly_run(
            conn,
            &lkjagent_store::artifact_graph::AssemblyRunInput {
                plan_id,
                run_id: &run_id,
                source_atom_ids: &report.sources,
                target_paths: std::slice::from_ref(&report.target),
                status: "assembled",
                measured_count: report.words as i64,
                summary: "deterministic assembly",
                created_at: now,
            },
        )?;
    }
    Ok(())
}

fn render(plan: &lkjagent_store::artifact_graph::PlanRow, p: &Projection) -> String {
    format!(
        "artifact audit {}\nroot={}\nkind={}\nartifact_profile={}\nplan_status={}\natom_total={}\natom_ready={}\natom_missing={}\nnext_atom={}\nnext_path={}\nmeasured_total={}\naccepted_floor={}\nassembly_pending={}\nreadiness={}\ncompletion_blockers={}\nnext_decision_required=true\ncandidate_action={}",
        if p.status == "ready" { "passed" } else { "failed" }, plan.root, plan.artifact_kind,
        plan.profile, plan.status, p.atom_total, p.atom_ready, p.atom_missing, p.next_atom_id,
        p.next_path, p.measured_total, p.accepted_floor, p.assembly_pending, p.status,
        if p.blockers.is_empty() { "none".to_string() } else { p.blockers.join(";") },
        if p.status == "ready" { "graph.evidence" } else { "artifact.next" }
    )
}
