mod support;

use lkjagent_store::graph::{
    active_case, evidence_for_case, faults, link_memory, memory_links_for_case, open_case,
    record_event, record_evidence, snapshots, update_case, GraphEvidenceRow, OpenCase,
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
            raw_owner_text: "please fix bug".to_string(),
            objective_version: 1,
            family: "bug-fix".to_string(),
            subroute: "code-change".to_string(),
            route_reason: "fix wording".to_string(),
            phase: "planning".to_string(),
            active_node: "plan".to_string(),
            plan: "inspect before editing".to_string(),
            evidence_requirements: requirements.clone(),
            selected_packages: packages.clone(),
            pending_checks: pending.clone(),
            next_action_class: "survey-plan-context".to_string(),
            context_pressure: "green".to_string(),
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
    assert_eq!(active.raw_owner_text, "please fix bug");
    assert_eq!(active.subroute, "code-change");
    assert_eq!(active.context_pressure, "green");
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

#[test]
fn graph_tables_persist_recovery_and_compaction_state() -> TestResult<()> {
    let conn = memory_store()?;
    let case_id = open_case(
        &conn,
        OpenCase {
            objective: "recover tool failure".to_string(),
            raw_owner_text: "recover tool failure".to_string(),
            objective_version: 1,
            family: "recovery".to_string(),
            subroute: "recovery".to_string(),
            route_reason: "failure wording".to_string(),
            phase: "recovery".to_string(),
            active_node: "recover-tool".to_string(),
            plan: "inspect graph and choose alternate tool".to_string(),
            evidence_requirements: vec!["fault-recorded".to_string()],
            selected_packages: vec!["recovery-policy".to_string()],
            pending_checks: Vec::new(),
            next_action_class: "alternate-native-tool".to_string(),
            context_pressure: "yellow".to_string(),
        },
        "2026-01-01T00:00:00Z",
    )?;

    faults::record_fault(
        &conn,
        case_id,
        "tool",
        Some("fs.patch:a.md"),
        "patch matched twice",
        3,
        "2026-01-01T00:00:01Z",
    )?;
    faults::upsert_recovery_state(
        &conn,
        case_id,
        3,
        "inspect graph.next and choose alternate native tool",
        "2026-01-01T00:00:02Z",
    )?;
    let snapshot = snapshots::record_compaction_snapshot(
        &conn,
        case_id,
        "recovery",
        "recover-tool",
        "recover tool failure",
        &[
            "objective".to_string(),
            "plan".to_string(),
            "evidence".to_string(),
        ],
        "2026-01-01T00:00:03Z",
    )?;

    let ladder: i64 = conn.query_row(
        "SELECT ladder_position FROM graph_recovery_state WHERE case_id = ?1",
        [case_id],
        |row| row.get(0),
    )?;
    let preserved: String = conn.query_row(
        "SELECT preserved_fields FROM graph_compaction_snapshots WHERE id = ?1",
        [snapshot],
        |row| row.get(0),
    )?;
    assert_eq!(ladder, 3);
    assert!(preserved.contains("objective"));
    assert!(preserved.contains("evidence"));
    Ok(())
}
