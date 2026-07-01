mod support;

use lkjagent_store::artifact_graph::{
    active_contract_for_plan, atoms_for_plan, create_contract, latest_plan_for_case, replace_atoms,
    upsert_plan, upsert_readiness, AtomInput, ContractInput, PlanInput, ReadinessInput,
};
use support::{memory_store, TestResult};

#[test]
fn artifact_graph_persists_plan_atoms_contract_and_readiness() -> TestResult<()> {
    let conn = memory_store()?;
    let plan_id = upsert_plan(&conn, &plan(), "2026-01-01T00:00:00Z")?;
    let sections = vec!["purpose".to_string(), "analysis".to_string()];
    let weak = vec!["below-count-floor".to_string()];
    replace_atoms(&conn, plan_id, &[atom(plan_id, &sections, &weak)])?;
    let atom_ids = vec!["reports/map:analysis.md".to_string()];
    let paths = vec!["reports/map/analysis.md".to_string()];
    create_contract(
        &conn,
        &ContractInput {
            contract_id: "contract-1",
            plan_id,
            atom_ids: &atom_ids,
            exact_paths: &paths,
            max_files: 1,
            max_file_bytes: 1800,
            max_batch_bytes: 1800,
            target_count: 200,
            count_floor: 120,
            required_sections: &sections,
            continuity_digest: "root=reports/map atom=analysis",
            forbidden_weak_classes: &weak,
            status: "active",
        },
        "2026-01-01T00:00:01Z",
    )?;
    upsert_readiness(&conn, &readiness(plan_id))?;

    let plan = latest_plan_for_case(&conn, 7)?.ok_or("missing plan")?;
    assert_eq!(plan.profile, "report");
    assert_eq!(atoms_for_plan(&conn, plan_id)?.len(), 1);
    assert_eq!(
        active_contract_for_plan(&conn, plan_id)?
            .ok_or("missing contract")?
            .count_floor,
        120
    );
    Ok(())
}

fn atom<'a>(plan_id: i64, sections: &'a [String], weak: &'a [String]) -> AtomInput<'a> {
    AtomInput {
        plan_id,
        atom_id: "reports/map:analysis.md".to_string(),
        sequence: 1,
        role: "analysis",
        path: "analysis.md",
        status: "planned",
        measurement_kind: "words",
        target_count: 200,
        count_floor: 120,
        measured_count: 0,
        byte_budget: 1800,
        required_sections: sections,
        weak_classes: weak,
        assembly_target: "",
    }
}

fn plan() -> PlanInput<'static> {
    PlanInput {
        case_id: 7,
        artifact_id: "7:report:reports:map:1200",
        owner_objective: "Write a report",
        artifact_kind: "report",
        root: "reports/map",
        profile: "report",
        normalized_title: "Map",
        measurement_kind: "words",
        requested_total: 1200,
        accepted_floor: 1200,
        section_count: 6,
        language_hint: "unspecified",
        forbidden_roots: &[],
        status: "planned",
    }
}

fn readiness(plan_id: i64) -> ReadinessInput<'static> {
    ReadinessInput {
        plan_id,
        status: "contracted",
        atom_total: 1,
        atom_ready: 0,
        atom_missing: 1,
        next_atom_id: "reports/map:analysis.md",
        next_path: "reports/map/analysis.md",
        active_contract_id: "contract-1",
        measured_total: 0,
        accepted_floor: 1200,
        assembly_pending: "false",
        completion_blockers: &[],
        updated_at: "2026-01-01T00:00:01Z",
    }
}
