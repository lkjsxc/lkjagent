use lkjagent_store::plan_inspect::application_tables;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{record, records, upsert_record, RecordRow};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn record_rows_round_trip_and_history() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    assert!(application_tables(&conn)?.contains("workspace_records"));
    let row = RecordRow {
        id: "rec_1".to_string(),
        kind: "todo".to_string(),
        title: "Pay bill".to_string(),
        state: "open".to_string(),
        path: "records/todo/rec_1.md".to_string(),
        fingerprint: "fp1".to_string(),
        archived: false,
        updated_at: "t1".to_string(),
    };

    upsert_record(&conn, &row)?;
    assert_eq!(record(&conn, "rec_1")?, Some(row.clone()));
    assert_eq!(records(&conn, Some("todo"), false)?, vec![row]);
    let history: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_record_history WHERE record_id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(history, 1);
    Ok(())
}
