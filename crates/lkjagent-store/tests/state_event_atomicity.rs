use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_store::event_rows::append_and_apply_event;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn duplicate_runtime_event_id_does_not_apply_second_patch() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Deduplicate events.", "t0")?;
    let first = event_with_cell("event-1", custom_cell("model", "first"));
    let second = event_with_cell("event-1", custom_cell("model", "second"));

    append_and_apply_event(&conn, &first)?;
    append_and_apply_event(&conn, &second)?;
    let snapshot = hydrate_snapshot(&conn, "case-1")?;

    assert!(snapshot.cells.contains_key(&key("model", "first")));
    assert!(!snapshot.cells.contains_key(&key("model", "second")));
    assert_eq!(count_rows(&conn, "runtime_events")?, 1);
    assert_eq!(count_rows(&conn, "state_history")?, 1);
    Ok(())
}

fn event_with_cell(id: &str, mut cell: StateCell) -> RuntimeEvent {
    cell.source_event_id = id.to_string();
    RuntimeEvent {
        id: id.to_string(),
        case_id: "case-1".to_string(),
        kind: "state.cell.upsert".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "test".to_string(),
        created_at: "t1".to_string(),
        decision_id: None,
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

fn key(namespace: &str, name: &str) -> StateKey {
    match StateKey::new(namespace, name) {
        Ok(key) => key,
        Err(_) => StateKey {
            namespace: namespace.to_string(),
            name: name.to_string(),
        },
    }
}

fn count_rows(conn: &Connection, table: &str) -> TestResult<i64> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    Ok(conn.query_row(&sql, [], |row| row.get(0))?)
}
