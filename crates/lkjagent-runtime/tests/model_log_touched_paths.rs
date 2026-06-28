use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_runtime::model_log::render_current_log;
use lkjagent_store::artifact_cursor::{upsert_batch_cursor, BatchCursorInput};
use lkjagent_store::artifact_ledger::{upsert_artifact, ArtifactLedgerInput};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn model_log_touched_paths_include_artifact_and_write_ledgers() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    lkjagent_store::schema::setup(&conn)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create a long novel. with structured settings.",
        "2026-01-01T00:00:00Z",
    )?;
    let artifact_id = upsert_artifact(&conn, &artifact(), "2026-01-01T00:00:01Z")?;
    record_cursor(&conn, artifact_id)?;

    let rendered = render_current_log(
        &conn,
        "2026-01-01T00:00:03Z",
        ContextBudgetPolicy::default(),
    )?;

    assert!(rendered.contains("* `stories/novel`"));
    assert!(rendered.contains("* `stories/novel/project/premise.md`"));
    assert!(!rendered.contains("## Touched Paths\n\n* none"));
    Ok(())
}

fn record_cursor(conn: &Connection, artifact_ledger_id: i64) -> TestResult<()> {
    let planned = vec!["project/premise.md".to_string()];
    let completed = vec!["project/premise.md".to_string()];
    upsert_batch_cursor(
        conn,
        &BatchCursorInput {
            artifact_ledger_id,
            root: "stories/novel",
            planned_paths: &planned,
            completed_paths: &completed,
            failed_paths: &[],
            current_index: 1,
            last_valid_example: "<action />",
            retry_counts: "",
            fallback_mode: "",
            updated_at: "2026-01-01T00:00:02Z",
        },
    )?;
    Ok(())
}

fn artifact() -> ArtifactLedgerInput<'static> {
    ArtifactLedgerInput {
        case_id: 1,
        artifact_id: "1:novel:long-novel:unspecified",
        root: "stories/novel",
        kind: "novel",
        normalized_topic: "long-novel",
        requested_scale: "unspecified",
        profile: "NarrativeManuscript",
        lifecycle_state: "content-partial",
        topology_status: "passed",
        readiness_status: "failed",
        objective_match_status: "unknown",
        latest_audit_id: None,
        weak_path_count: 28,
    }
}
