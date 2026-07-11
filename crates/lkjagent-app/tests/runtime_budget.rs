use lkjagent_app::runtime_budget::{enforce, usage};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_core::runtime_state::{StateCell, StateKey};
use lkjagent_store::decision_rows::{insert_runtime_decision, settle_decision};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{insert_case, upsert_state_cell};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn every_durable_budget_dimension_can_block_independently() -> TestResult<()> {
    for (dimension, limits) in [
        ("tokens", [12, 1_000, 10, 10]),
        ("active-milliseconds", [1_000, 100, 10, 10]),
        ("effects", [1_000, 1_000, 1, 10]),
        ("recovery-cost", [1_000, 1_000, 10, 1]),
    ] {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        insert_case(&conn, "1", "Budget dimension", "2026-07-11T00:00:00Z")?;
        for (key, value) in [
            "runtime.case_token_budget",
            "runtime.case_active_milliseconds",
            "runtime.case_effect_budget",
            "runtime.case_recovery_budget",
        ]
        .into_iter()
        .zip(limits)
        {
            conn.execute(
                "INSERT INTO config (key, value) VALUES (?1, ?2)",
                (key, value.to_string()),
            )?;
        }
        let decision = RuntimeDecision::new(
            "decision-used",
            "1",
            OperationKey("model.call/1".into()),
            ToolSetView::empty(),
            OutputEnvelope::Message,
        );
        insert_runtime_decision(&conn, &decision, "pending", "2026-07-11T00:00:00.000Z")?;
        settle_decision(&conn, &decision.id, "settled", "2026-07-11T00:00:00.100Z")?;
        conn.execute(
            "INSERT INTO token_usage (task_id, input_total_tokens, output_tokens, created_at)
            VALUES (1, 5, 7, 'now')",
            [],
        )?;
        conn.execute("INSERT INTO observations (id, case_id, decision_id, effect_name, status, content,
            artifact_refs_json, contamination_class, created_at) VALUES
            ('observation-used', '1', 'decision-used', 'fs.read', 'ok', 'evidence', '[]', 'Clean', 'now')", [])?;
        let key = StateKey::new("recovery", "used").map_err(|error| error.message)?;
        let mut recovery = StateCell::active(key, "source-recovery");
        recovery.payload_schema = "recovery.failure".to_string();
        upsert_state_cell(&conn, "1", &recovery)?;
        assert!(enforce(&conn, "1", "2026-07-11T00:00:00.100Z", &[])?);
        let payload: String = conn.query_row(
            "SELECT payload_json FROM state_cells
            WHERE payload_schema = 'completion.blocked'",
            [],
            |row| row.get(0),
        )?;
        assert!(
            payload.contains(&format!("\"reached\":[\"{dimension}\"]")),
            "{payload}"
        );
    }
    Ok(())
}

#[test]
fn separate_case_budgets_use_durable_rows_and_block_truthfully() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "1", "Long bounded work", "2026-07-11T00:00:00Z")?;
    for (key, value) in [
        ("runtime.case_token_budget", 100),
        ("runtime.case_active_milliseconds", 50),
        ("runtime.case_effect_budget", 10),
        ("runtime.case_recovery_budget", 10),
    ] {
        conn.execute(
            "INSERT INTO config (key, value) VALUES (?1, ?2)",
            (key, value.to_string()),
        )?;
    }
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "1",
        OperationKey("model.call/1".into()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    decision.selected_state_key = Some("model:step-1".to_string());
    insert_runtime_decision(&conn, &decision, "pending", "2026-07-11T00:00:00.000Z")?;
    settle_decision(&conn, &decision.id, "settled", "2026-07-11T00:00:00.100Z")?;
    conn.execute(
        "INSERT INTO token_usage (task_id, input_total_tokens, output_tokens, created_at)
        VALUES (1, 5, 7, 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO observations (id, case_id, decision_id, effect_name, status, content,
        artifact_refs_json, contamination_class, created_at) VALUES
        ('observation-1', '1', 'decision-1', 'fs.read', 'ok', 'evidence', '[]', 'Clean', 'now')",
        [],
    )?;
    let key = StateKey::new("model", "step-1").map_err(|error| error.message)?;
    upsert_state_cell(&conn, "1", &StateCell::active(key, "source-model"))?;
    let key = StateKey::new("recovery", "failure-1").map_err(|error| error.message)?;
    let mut recovery = StateCell::active(key, "source-recovery");
    recovery.payload_schema = "recovery.failure".to_string();
    upsert_state_cell(&conn, "1", &recovery)?;

    let used = usage(&conn, "1", "2026-07-11T00:00:01Z")?;
    assert_eq!(
        (
            used.tokens,
            used.active_milliseconds,
            used.effects,
            used.recovery_cost
        ),
        (12, 100, 1, 1)
    );
    let pending = RuntimeDecision::new(
        "decision-2",
        "1",
        OperationKey("model.call/1".into()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    insert_runtime_decision(&conn, &pending, "pending", "2026-07-11T00:00:00.100Z")?;
    assert!(enforce(&conn, "1", "2026-07-11T00:00:01Z", &[pending])?);
    let (source, payload, pending): (String, String, String) = conn.query_row(
        "SELECT
        (SELECT status FROM state_cells WHERE key_label = 'model:step-1'),
        (SELECT payload_json FROM state_cells WHERE payload_schema = 'completion.blocked'),
        (SELECT status FROM runtime_decisions WHERE id = 'decision-2')",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!(source, "Suppressed");
    assert_eq!(pending, "budget-exhausted");
    assert!(payload.contains("active-milliseconds"));
    assert!(payload.contains("raise the named case budget"));
    Ok(())
}
