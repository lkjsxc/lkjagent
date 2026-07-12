use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, StalenessClass, TrustClass};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_core::runtime_tool_catalog::tool_view_for_names;
use lkjagent_store::context_rows::{context_items, insert_context_item};
use lkjagent_store::decision_rows::{
    insert_runtime_decision, settle_decision, unfinished_decisions,
};
use lkjagent_store::event_rows::append_and_apply_event;
use lkjagent_store::plan_inspect::application_tables;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case, upsert_state_cell};
use lkjagent_store::state_schema::STATE_LEDGER_TABLES;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn state_ledger_tables_are_created_with_plan_schema() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let tables = application_tables(&conn)?;
    for table in STATE_LEDGER_TABLES {
        assert!(tables.contains(*table), "missing {table}");
    }
    Ok(())
}

#[test]
fn unknown_state_cells_round_trip_from_sqlite() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Preserve unknown cells.", "t0")?;
    let cell = custom_cell("custom", "alpha/unit-7");

    upsert_state_cell(&conn, "case-1", &cell)?;
    let snapshot = hydrate_snapshot(&conn, "case-1")?;

    assert_eq!(snapshot.case_id, "case-1");
    assert_eq!(snapshot.cells.get(&cell.key), Some(&cell));
    let history: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_history WHERE key_label = ?1",
        [cell.key.as_label()],
        |row| row.get(0),
    )?;
    assert_eq!(history, 1);
    Ok(())
}

#[test]
fn events_append_and_apply_reducer_patches() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Apply events.", "t0")?;
    let mut cell = custom_cell("model", "1");
    cell.source_event_id = "event-1".to_string();
    let event = RuntimeEvent {
        id: "event-1".to_string(),
        case_id: "case-1".to_string(),
        kind: "state.cell.upsert".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell.clone())),
        source: "test".to_string(),
        created_at: "t1".to_string(),
        decision_id: None,
    };

    append_and_apply_event(&conn, &event)?;
    let snapshot = hydrate_snapshot(&conn, "case-1")?;

    assert_eq!(snapshot.cells.get(&cell.key), Some(&cell));
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM runtime_events", [], |row| row.get(0))?;
    assert_eq!(count, 1);
    Ok(())
}

#[test]
fn runtime_decisions_persist_and_unfinished_hydrate_first() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Persist decisions.", "t0")?;
    let early = decision("decision-a");
    let late = decision("decision-b");

    insert_runtime_decision(&conn, &late, "pending", "t2")?;
    insert_runtime_decision(&conn, &early, "pending", "t1")?;
    let pending = unfinished_decisions(&conn, "case-1")?;
    assert_eq!(ids(&pending), vec!["decision-a", "decision-b"]);
    assert_eq!(
        pending[0].selected_state_key.as_deref(),
        Some("model:from-decision")
    );

    let tool_fp: String = conn.query_row(
        "SELECT tool_view_fingerprint FROM runtime_decisions WHERE id = 'decision-a'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(tool_fp, tool_fingerprint(&early));
    assert_eq!(settle_decision(&conn, "decision-a", "settled", "t3")?, 1);
    assert_eq!(
        ids(&unfinished_decisions(&conn, "case-1")?),
        vec!["decision-b"]
    );
    Ok(())
}

#[test]
fn context_items_preserve_semantic_key_and_contamination() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Persist context.", "t0")?;
    let mut item = ContextItem::clean_fact("ctx-1", "target-root", "workspace/reports");
    item.trust = TrustClass::Model;
    item.staleness = StalenessClass::Current;
    item.contamination = ContaminationClass::FailedModelOutput;
    item.artifact_refs = vec!["artifact-1".to_string()];
    item.decision_id = Some("decision-a".to_string());
    item.created_at = "t1".to_string();

    insert_context_item(&conn, "case-1", &item)?;
    let rows = context_items(&conn, "case-1")?;

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].semantic_key, "target-root");
    assert_eq!(rows[0].contamination, ContaminationClass::FailedModelOutput);
    assert_eq!(rows[0].artifact_refs, vec!["artifact-1"]);
    Ok(())
}

fn key(namespace: &str, name: &str) -> StateKey {
    match StateKey::new(namespace, name) {
        Ok(key) => key,
        Err(_) => StateKey {
            namespace: namespace.to_string(),
            name: name.to_string(),
        },
    }
}

fn custom_cell(namespace: &str, name: &str) -> StateCell {
    let mut cell = StateCell::active(key(namespace, name), "event-1");
    cell.payload_schema = "custom.schema".to_string();
    cell.payload_json = "{\"value\":7}".to_string();
    cell.created_at = "t1".to_string();
    cell.updated_at = "t1".to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "owner".to_string(),
        source_id: "msg-1".to_string(),
        fingerprint: "fp-1".to_string(),
    }];
    cell
}

fn decision(id: &str) -> RuntimeDecision {
    let mut decision = RuntimeDecision::new(
        id,
        "case-1",
        OperationKey("model.call".to_string()),
        tool_view_for_names(&["fs.read"]),
        OutputEnvelope::Action,
    );
    decision.selected_state_key = Some("model:from-decision".to_string());
    decision.snapshot_fingerprint = "snapshot-fp".to_string();
    decision.state_vector_fingerprint = "state-fp".to_string();
    decision.context_frame_fingerprint = "context-fp".to_string();
    decision.model_budget_tokens = Some(512);
    decision.evidence_requirements = vec!["fresh artifact check".to_string()];
    decision.recovery_policy = "retry-same-decision".to_string();
    decision
}

fn ids(decisions: &[RuntimeDecision]) -> Vec<&str> {
    decisions
        .iter()
        .map(|decision| decision.id.as_str())
        .collect()
}

fn tool_fingerprint(decision: &RuntimeDecision) -> String {
    decision.tool_view_fingerprint().unwrap_or_default()
}
