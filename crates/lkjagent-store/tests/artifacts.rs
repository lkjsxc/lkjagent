use lkjagent_store::artifact_rows::{artifacts, insert_artifact, ArtifactRow};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn artifact_rows_preserve_fingerprints_and_unit_metadata() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Assemble checked units.", "t0")?;
    insert_artifact(
        &conn,
        &ArtifactRow {
            id: "unit-1".to_string(),
            case_id: "case-1".to_string(),
            kind: "unit".to_string(),
            path: "stories/chapter.md".to_string(),
            fingerprint: "fp-unit".to_string(),
            parent_artifact_id: Some("file-1".to_string()),
            metadata_json: "{\"ordinal\":1,\"target_tokens\":512}".to_string(),
            created_at: "t1".to_string(),
        },
    )?;

    let rows = artifacts(&conn, "case-1")?;

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].kind, "unit");
    assert_eq!(rows[0].fingerprint, "fp-unit");
    assert!(rows[0].metadata_json.contains("target_tokens"));
    Ok(())
}
