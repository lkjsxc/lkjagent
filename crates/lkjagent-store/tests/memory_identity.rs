mod support;

use lkjagent_store::memory::{save_decision, MemoryKind, MemoryWriteDecision};
use support::{memory_store, TestResult};

#[test]
fn memory_save_duplicate_title_and_content_skips() -> TestResult<()> {
    let mut conn = memory_store()?;
    let first = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "Graph Policy Context",
        "graph,policy",
        "Record policy candidates once before maintenance writes lessons.",
        100,
        "2026-01-01T00:00:00Z",
    )?;
    let second = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "Graph policy context",
        "policy",
        "Record policy candidates once before maintenance writes lessons.",
        100,
        "2026-01-01T00:00:01Z",
    )?;

    assert!(matches!(first, MemoryWriteDecision::Insert { .. }));
    assert_eq!(second.id(), first.id());
    assert!(matches!(second, MemoryWriteDecision::SkipDuplicate { .. }));
    Ok(())
}

#[test]
fn memory_save_duplicate_content_different_kind_policy_inserts() -> TestResult<()> {
    let mut conn = memory_store()?;
    let first = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "FTS punctuation",
        "memory",
        "Normalize graph dot note query text before FTS search.",
        100,
        "2026-01-01T00:00:00Z",
    )?;
    let second = save_decision(
        &mut conn,
        MemoryKind::Incident,
        "FTS punctuation",
        "memory",
        "Normalize graph dot note query text before FTS search.",
        100,
        "2026-01-01T00:00:01Z",
    )?;

    assert_ne!(second.id(), first.id());
    assert!(matches!(second, MemoryWriteDecision::Insert { .. }));
    Ok(())
}

#[test]
fn maintenance_duplicate_policy_candidate_updates_existing() -> TestResult<()> {
    let mut conn = memory_store()?;
    let first = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "Graph Policy Improvement Candidates",
        "maintenance,graph",
        "Record graph policy candidates once and skip duplicate maintenance lessons.",
        100,
        "2026-01-01T00:00:00Z",
    )?;
    let second = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "graph policy improvement candidates",
        "maintenance,graph",
        "Record graph policy candidates once and skip duplicate maintenance lessons now.",
        101,
        "2026-01-01T00:00:01Z",
    )?;

    assert_eq!(second.id(), first.id());
    assert!(matches!(second, MemoryWriteDecision::UpdateExisting { .. }));
    Ok(())
}

#[test]
fn memory_save_returns_existing_id_for_same_tags_and_content() -> TestResult<()> {
    let mut conn = memory_store()?;
    let first = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "Contract Mismatch",
        "contract,graph",
        "Generated graph plan examples must include checks or paths.",
        100,
        "2026-01-01T00:00:00Z",
    )?;
    let second = save_decision(
        &mut conn,
        MemoryKind::Lesson,
        "Graph Plan Example",
        "graph,contract",
        "Generated graph plan examples must include checks or paths.",
        100,
        "2026-01-01T00:00:01Z",
    )?;

    assert_eq!(second.id(), first.id());
    assert!(matches!(second, MemoryWriteDecision::SkipDuplicate { .. }));
    Ok(())
}
