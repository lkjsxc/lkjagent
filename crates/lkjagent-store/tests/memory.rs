mod support;

use lkjagent_store::graph::{link_memory, open_case, OpenCase};
use lkjagent_store::memory::{
    delete as delete_memory, digest, find, save, update as update_memory, MemoryKind, MemoryUpdate,
};
use support::{memory_store, TestResult};

#[test]
fn memory_find_ranks_kind_then_recency_and_digest_fits_budget() -> TestResult<()> {
    let mut conn = memory_store()?;
    let fact = save(
        &mut conn,
        MemoryKind::Fact,
        "parser",
        "protocol",
        "parser detail",
        100,
        "2026-01-01T00:00:00Z",
    )?;
    let summary = save(
        &mut conn,
        MemoryKind::TaskSummary,
        "parser",
        "protocol",
        "parser summary",
        100,
        "2026-01-01T00:00:01Z",
    )?;
    let lesson = save(
        &mut conn,
        MemoryKind::Lesson,
        "parser",
        "protocol",
        "parser lesson",
        2_100,
        "2026-01-01T00:00:02Z",
    )?;

    let found = find(&conn, "parser", 3)?;
    assert_eq!(found.first().map(|row| row.id), Some(summary));

    let digest = digest(&conn, Some(summary), 2_048)?;
    assert_eq!(digest.first().map(|row| row.id), Some(summary));
    assert!(digest.iter().any(|row| row.id == fact));
    assert!(!digest.iter().any(|row| row.id == lesson));
    Ok(())
}

#[test]
fn memory_updates_and_deletes_keep_fts_current() -> TestResult<()> {
    let mut conn = memory_store()?;
    let id = save(
        &mut conn,
        MemoryKind::Fact,
        "stale parser",
        "protocol",
        "old parser detail",
        100,
        "2026-01-01T00:00:00Z",
    )?;

    update_memory(
        &mut conn,
        id,
        MemoryUpdate {
            kind: MemoryKind::Lesson,
            title: "fresh parser",
            tags: "protocol",
            content: "new parser detail",
            tokens: 90,
        },
        "2026-01-01T00:00:01Z",
    )?;

    let stale = find(&conn, "stale", 3)?;
    let fresh = find(&conn, "fresh", 3)?;
    assert!(stale.is_empty());
    assert_eq!(fresh.first().map(|row| row.id), Some(id));

    delete_memory(&mut conn, id)?;
    let fresh = find(&conn, "fresh", 3)?;
    assert!(fresh.is_empty());
    Ok(())
}

#[test]
fn memory_find_prefers_active_graph_link_within_kind() -> TestResult<()> {
    let mut conn = memory_store()?;
    let newer = save(
        &mut conn,
        MemoryKind::Lesson,
        "parser",
        "protocol",
        "parser recovery detail",
        100,
        "2026-01-01T00:00:02Z",
    )?;
    let linked = save(
        &mut conn,
        MemoryKind::Lesson,
        "parser",
        "protocol",
        "parser recovery detail",
        100,
        "2026-01-01T00:00:01Z",
    )?;
    let requirements = vec!["memory".to_string()];
    let packages = Vec::new();
    let pending = Vec::new();
    let case_id = open_case(
        &conn,
        OpenCase {
            objective: "fix parser recovery".to_string(),
            raw_owner_text: "fix parser recovery".to_string(),
            objective_version: 1,
            family: "bug-fix".to_string(),
            subroute: "code-change".to_string(),
            route_reason: "fix wording".to_string(),
            phase: "execution".to_string(),
            active_node: "execute".to_string(),
            plan: "reuse linked parser memory".to_string(),
            evidence_requirements: requirements,
            selected_packages: packages,
            pending_checks: pending,
            next_action_class: "execute-step".to_string(),
            context_pressure: "green".to_string(),
        },
        "2026-01-01T00:00:03Z",
    )?;
    link_memory(
        &conn,
        case_id,
        linked,
        "execute",
        "task-context",
        "2026-01-01T00:00:03Z",
    )?;

    let found = find(&conn, "parser recovery", 2)?;
    assert_eq!(found.first().map(|row| row.id), Some(linked));
    assert!(found.iter().any(|row| row.id == newer));
    Ok(())
}
