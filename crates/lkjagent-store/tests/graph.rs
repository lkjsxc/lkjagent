mod support;

use lkjagent_store::graph::{
    active_case, evidence_for_case, link_memory, memory_links_for_case, open_case, record_event,
    record_evidence, update_case, GraphEvidenceRow, OpenCase,
};
use lkjagent_store::memory::{self, MemoryKind};
use support::{memory_store, TestResult};

#[test]
fn graph_tables_persist_active_cases_and_evidence() -> TestResult<()> {
    let mut conn = memory_store()?;
    let requirements = vec!["plan".to_string(), "observation".to_string()];
    let packages = vec!["planning-checklist".to_string()];
    let pending = vec!["focused verification".to_string()];
    let case_id = open_case(
        &conn,
        OpenCase {
            objective: "fix bug".to_string(),
            family: "bug-fix".to_string(),
            phase: "planning".to_string(),
            active_node: "plan".to_string(),
            plan: "inspect before editing".to_string(),
            evidence_requirements: requirements.clone(),
            selected_packages: packages.clone(),
            pending_checks: pending.clone(),
        },
        "2026-01-01T00:00:00Z",
    )?;

    record_event(
        &conn,
        case_id,
        "owner",
        "plan",
        "planning",
        "owner message delivered",
        "2026-01-01T00:00:00Z",
    )?;
    record_evidence(
        &conn,
        case_id,
        &GraphEvidenceRow {
            requirement: "plan".to_string(),
            kind: "note".to_string(),
            summary: "initial plan created".to_string(),
            path: None,
        },
        "2026-01-01T00:00:00Z",
    )?;
    update_case(
        &conn,
        case_id,
        "execution",
        "execute",
        "active",
        "2026-01-01T00:00:01Z",
    )?;

    let active = active_case(&conn)?.ok_or("missing active graph case")?;
    assert_eq!(active.id, case_id);
    assert_eq!(active.phase, "execution");
    assert_eq!(active.active_node, "execute");
    assert_eq!(active.evidence_requirements, requirements);
    assert_eq!(active.selected_packages, packages);
    assert_eq!(active.pending_checks, pending);

    let evidence = evidence_for_case(&conn, case_id)?;
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].requirement, "plan");

    let memory_id = memory::save(
        &mut conn,
        MemoryKind::TaskSummary,
        "fixed bug",
        "task",
        "summary body",
        2,
        "2026-01-01T00:00:02Z",
    )?;
    link_memory(
        &conn,
        case_id,
        memory_id,
        "execute",
        "task-summary",
        "2026-01-01T00:00:02Z",
    )?;
    let links = memory_links_for_case(&conn, case_id)?;
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].case_id, case_id);
    assert_eq!(links[0].memory_id, memory_id);
    assert_eq!(links[0].node, "execute");
    assert_eq!(links[0].reason, "task-summary");
    Ok(())
}
