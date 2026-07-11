use lkjagent_app::progress_bridge;
use lkjagent_core::runtime_candidate::selected_candidate_at;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::decision_rows::insert_runtime_decision;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case, upsert_state_cell};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn repeated_vector_emits_strategy_change_and_changed_strategy_resets_window() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Keep making useful progress", "t0")?;
    conn.execute(
        "INSERT INTO config (key, value) VALUES ('runtime.no_progress_window', '2')",
        [],
    )?;

    let first = decision("decision-1", "retry-same-decision");
    let second = decision("decision-2", "retry-same-decision");
    insert_runtime_decision(&conn, &first, "pending", "t1")?;
    insert_runtime_decision(&conn, &second, "pending", "t2")?;
    insert_observation(&conn, "observation-1", "decision-1", "t1")?;
    progress_bridge::record(&conn, &first, "t1")?;
    insert_observation(&conn, "observation-2", "decision-2", "t2")?;
    conn.execute("INSERT INTO observations (id, case_id, decision_id, admission_id, effect_name,
        status, content, artifact_refs_json, contamination_class, created_at) VALUES
        ('external', 'case-1', 'decision-2', NULL, 'fs.read', 'ok', 'volatile raw bytes', '[]', 'ExternalRaw', 't2')", [])?;
    progress_bridge::record(&conn, &second, "t2")?;

    let payload: String = conn.query_row(
        "SELECT payload_json FROM state_cells WHERE payload_schema = 'recovery.no-progress'",
        [],
        |row| row.get(0),
    )?;
    assert!(payload.contains("\"next_strategy\":\"inspect-state\""));
    assert!(payload.contains("\"window\":2"));
    let vectors: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'runtime.progress'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(vectors, 2);

    let changed = decision("decision-3", "inspect-state");
    insert_runtime_decision(&conn, &changed, "pending", "t3")?;
    progress_bridge::record(&conn, &changed, "t3")?;
    let adaptations: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'recovery.no-progress'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(adaptations, 1);
    Ok(())
}

#[test]
fn vectors_keep_only_current_check_and_artifact_revisions() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Track current evidence", "t0")?;
    conn.execute(
        "INSERT INTO config (key, value) VALUES ('runtime.no_progress_window', '3')",
        [],
    )?;
    let first = decision("decision-current-1", "default");
    insert_runtime_decision(&conn, &first, "pending", "t1")?;
    conn.execute(
        "INSERT INTO check_results (step_id, name, params_json, decision_id,
        evidence_fingerprint, artifact_refs_json, passed, measured, created_at)
        VALUES (1, 'current-check', '{}', 'decision-current-1', 'passed-old', '[]', 1, 'ok', 't1')",
        [],
    )?;
    conn.execute(
        "INSERT INTO artifacts (id, case_id, kind, path, fingerprint, metadata_json, created_at)
        VALUES ('old-artifact', 'case-1', 'document', 'same.md', 'old-fingerprint', '{}', 't1')",
        [],
    )?;
    insert_case(&conn, "case-other", "Other evidence", "t1")?;
    let other = RuntimeDecision::new(
        "decision-other",
        "case-other",
        OperationKey("check.run/1".into()),
        ToolSetView::empty(),
        OutputEnvelope::None,
    );
    insert_runtime_decision(&conn, &other, "settled", "t1")?;
    conn.execute("INSERT INTO check_results (step_id, name, params_json, decision_id,
        evidence_fingerprint, artifact_refs_json, passed, measured, created_at)
        VALUES (1, 'current-check', '{}', 'decision-other', 'other-failure', '[]', 0, 'failed', 't1')", [])?;
    progress_bridge::record(&conn, &first, "t1")?;
    let payload: String = conn.query_row(
        "SELECT payload_json FROM state_cells
        WHERE key_label = 'progress:decision-current-1'",
        [],
        |row| row.get(0),
    )?;
    assert!(payload.contains("current-check"));
    let second = decision("decision-current-2", "default");
    insert_runtime_decision(&conn, &second, "pending", "t2")?;
    conn.execute("INSERT INTO check_results (step_id, name, params_json, decision_id,
        evidence_fingerprint, artifact_refs_json, passed, measured, created_at)
        VALUES (1, 'current-check', '{}', 'decision-current-2', 'failed-new', '[]', 0, 'failed', 't2')", [])?;
    conn.execute(
        "INSERT INTO artifacts (id, case_id, kind, path, fingerprint, metadata_json, created_at)
        VALUES ('new-artifact', 'case-1', 'document', 'same.md', 'new-fingerprint', '{}', 't2')",
        [],
    )?;
    progress_bridge::record(&conn, &second, "t2")?;
    let payload: String = conn.query_row(
        "SELECT payload_json FROM state_cells
        WHERE key_label = 'progress:decision-current-2'",
        [],
        |row| row.get(0),
    )?;
    assert!(payload.contains("new-fingerprint"));
    assert!(!payload.contains("old-fingerprint"));
    assert!(!payload.contains("current-check"));
    Ok(())
}

#[test]
#[rustfmt::skip]
fn changed_progress_releases_suspend_then_exhausts_to_visible_block() -> TestResult<()> {
    let conn = Connection::open_in_memory()?; setup(&conn)?;
    insert_case(&conn, "case-1", "Keep making useful progress", "t0")?;
    conn.execute("INSERT INTO config (key, value) VALUES ('runtime.no_progress_window', '1')", [])?;
    for (index, policy) in ["default", "default", "inspect-state", "inspect-state", "split-work",
        "split-work", "replan", "replan", "clarify", "clarify"].iter().enumerate() {
        let id = format!("decision-{}", index + 1); let item = decision(&id, policy);
        insert_runtime_decision(&conn, &item, "pending", &format!("t{}", index + 1))?;
        progress_bridge::record(&conn, &item, &format!("t{}", index + 1))?;
        if index == 0 { let count: i64 = conn.query_row("SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'recovery.no-progress'", [], |row| row.get(0))?; assert_eq!(count, 0); }
    }
    let suspend: String = conn.query_row("SELECT status FROM state_cells
        WHERE payload_schema = 'recovery.no-progress' AND json_extract(payload_json, '$.next_strategy') = 'suspend'",
        [], |row| row.get(0))?;
    assert_eq!(suspend, "Active");
    let key = StateKey::new("model", "competing-step").map_err(|error| error.message)?;
    upsert_state_cell(&conn, "case-1", &StateCell::active(key, "source-model"))?;
    let snapshot = hydrate_snapshot(&conn, "case-1")?;
    assert_eq!(selected_candidate_at(&snapshot, "t5").operation.key, "runtime.wait");
    let key = StateKey::new("index", "navigation").map_err(|error| error.message)?;
    let index = StateCell::active(key, "source-index");
    upsert_state_cell(&conn, "case-1", &index)?;
    let snapshot = hydrate_snapshot(&conn, "case-1")?;
    assert_eq!(selected_candidate_at(&snapshot, "t6").operation.key, "index.rebuild/navigation");
    let mut changed = decision("decision-11", "changed-external-condition");
    changed.operation = OperationKey("index.rebuild/navigation".to_string());
    insert_runtime_decision(&conn, &changed, "pending", "t11")?; progress_bridge::record(&conn, &changed, "t11")?;
    let (suspend, blocked): (String, i64) = conn.query_row("SELECT
        (SELECT status FROM state_cells WHERE payload_schema = 'recovery.no-progress'
            AND json_extract(payload_json, '$.next_strategy') = 'suspend'),
        (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked' AND status = 'Active')",
        [], |row| Ok((row.get(0)?, row.get(1)?)))?;
    assert_eq!(suspend, "Suppressed"); assert_eq!(blocked, 0);
    for index in 12..=13 {
        let item = decision(&format!("decision-{index}"), "post-wake");
        insert_runtime_decision(&conn, &item, "pending", &format!("t{index}"))?;
        progress_bridge::record(&conn, &item, &format!("t{index}"))?;
    }
    let blocked: i64 = conn.query_row("SELECT COUNT(*) FROM state_cells
        WHERE payload_schema = 'completion.blocked' AND status = 'Active'", [], |row| row.get(0))?;
    assert_eq!(blocked, 1); Ok(())
}

fn insert_observation(
    conn: &Connection,
    id: &str,
    decision: &str,
    now: &str,
) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO observations (id, case_id, decision_id, admission_id, effect_name,
        status, content, artifact_refs_json, contamination_class, created_at)
        VALUES (?1, 'case-1', ?2, NULL, 'fs.read', 'ok', 'same source bytes', '[]', 'Clean', ?3)",
        [id, decision, now],
    )
}

fn decision(id: &str, policy: &str) -> RuntimeDecision {
    let mut decision = RuntimeDecision::new(
        id,
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    decision.recovery_policy = policy.to_string();
    decision
}
