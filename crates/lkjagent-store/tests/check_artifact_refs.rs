use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::{CheckResult, StepKind};
use lkjagent_store::artifact_rows::{insert_artifact, ArtifactRow};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_hydrate::snapshot_by_id;
use lkjagent_store::plan_schema::setup;
use rusqlite::{params, Connection};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn hydrated_checks_suppress_stale_artifact_refs() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(11, "Write notes/out.md with setup notes.");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_file_artifact(&conn, "artifact-old", "old-fingerprint", "001")?;
    commit_turn(
        &conn,
        &snapshot,
        &[record_checks(&snapshot, "artifact-old")],
        "002",
    )?;

    let row: String = conn.query_row(
        "SELECT artifact_refs_json FROM check_results LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(row, "[\"artifact-old\"]");
    assert_eq!(hydrated_by_id(&conn, 11)?.check_results.len(), 1);
    assert_eq!(edge_count(&conn, "Active")?, 1);

    insert_file_artifact(&conn, "artifact-new", "new-fingerprint", "003")?;

    assert!(hydrated_by_id(&conn, 11)?.check_results.is_empty());
    assert_eq!(edge_count(&conn, "Active")?, 0);
    assert_eq!(edge_count(&conn, "Suppressed")?, 1);
    Ok(())
}

#[test]
fn passed_rows_without_native_completion_cells_do_not_hydrate() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(12, "Write notes/out.md with setup notes.");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    insert_file_artifact(&conn, "artifact-old", "old-fingerprint", "001")?;
    conn.execute(
        "INSERT INTO check_results (step_id, name, params_json, decision_id,
         evidence_fingerprint, artifact_refs_json, passed, measured, created_at)
         VALUES (?1, 'file_exists', ?2, 'decision-check', 'evidence-fp',
                 '[\"artifact-old\"]', 1, 'true', '002')",
        params![verify_step_id(&snapshot) as i64, params_json(&snapshot)?],
    )?;

    assert!(hydrated_by_id(&conn, 12)?.check_results.is_empty());
    Ok(())
}

fn edge_count(conn: &Connection, status: &str) -> TestResult<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM state_edges WHERE status = ?1",
        [status],
        |row| row.get(0),
    )?)
}

fn hydrated_by_id(conn: &Connection, id: i64) -> TestResult<lkjagent_core::model::TaskSnapshot> {
    match snapshot_by_id(conn, id)? {
        Some(snapshot) => Ok(snapshot),
        None => Err("missing snapshot".into()),
    }
}

fn record_checks(snapshot: &lkjagent_core::model::TaskSnapshot, artifact_id: &str) -> Command {
    Command::RecordChecks {
        step_id: verify_step_id(snapshot),
        decision_id: Some("decision-check".to_string()),
        results: vec![CheckResult {
            name: "file_exists".to_string(),
            params: None,
            decision_id: None,
            evidence_fingerprint: Some("evidence-fp".to_string()),
            artifact_refs: vec![artifact_id.to_string()],
            passed: true,
            measured: "true".to_string(),
        }],
    }
}

fn params_json(snapshot: &lkjagent_core::model::TaskSnapshot) -> TestResult<String> {
    let Some(spec) = snapshot.task.checks.first().cloned() else {
        return Err("missing check spec".into());
    };
    Ok(serde_json::to_string(&Some(spec))?)
}

fn verify_step_id(snapshot: &lkjagent_core::model::TaskSnapshot) -> u64 {
    snapshot
        .steps
        .iter()
        .find(|step| step.kind == StepKind::Verify)
        .map(|step| step.id)
        .unwrap_or(2)
}

fn insert_file_artifact(
    conn: &Connection,
    id: &str,
    fingerprint: &str,
    created_at: &str,
) -> TestResult<()> {
    insert_artifact(
        conn,
        &ArtifactRow {
            id: id.to_string(),
            case_id: "11".to_string(),
            kind: "file".to_string(),
            path: "notes/out.md".to_string(),
            fingerprint: fingerprint.to_string(),
            parent_artifact_id: None,
            metadata_json: "{}".to_string(),
            created_at: created_at.to_string(),
        },
    )?;
    Ok(())
}
