use std::{fs, path::PathBuf};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_admission::{AdmissionStatus, ToolAdmission};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, mark_journal, EffectPreparation, EffectTargetRevision,
};
use lkjagent_store::artifact_rows::ArtifactRow;
use lkjagent_store::observation_rows::{settle_effect_observation, ObservationRow};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn late_decision_failure_rolls_back_turn_settlement() -> TestResult<()> {
    let data = fixture_root("atomic-turn-settlement")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "create an artifact report from these notes", "now")?;
    conn.execute_batch(
        "CREATE TRIGGER fail_turn_settlement AFTER UPDATE OF status ON runtime_decisions
         WHEN NEW.status <> 'pending' BEGIN UPDATE runtime_decisions
         SET status = 'pending', settled_at = NULL WHERE id = NEW.id; END;",
    )?;
    drop(conn);
    let body = (0..900)
        .map(|index| format!(" word{index}"))
        .collect::<String>();
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![format!("<content># Atomic report\n\n{body}</content>")],
        index: 0,
    };
    let error = run_until_idle(&data, &mut endpoint, 1)
        .err()
        .ok_or("artifact turn unexpectedly settled")?;
    assert!(error.contains("decision settlement status remained pending"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let state: String = conn.query_row("SELECT state FROM effect_journal", [], |row| row.get(0))?;
    let decision: String =
        conn.query_row("SELECT status FROM runtime_decisions", [], |row| row.get(0))?;
    let count = |table: &str| -> rusqlite::Result<i64> {
        conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
            row.get(0)
        })
    };
    assert_eq!((state.as_str(), decision.as_str()), ("applying", "pending"));
    let context: i64 = conn.query_row(
        "SELECT COUNT(*) FROM context_items WHERE source_type = 'observation'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(
        (
            count("observations")?,
            count("artifacts")?,
            count("attempts")?,
            context
        ),
        (0, 0, 0, 0),
    );
    assert!(data
        .join("workspace/artifacts/requests/matter-1.md")
        .exists());
    conn.execute_batch("DROP TRIGGER fail_turn_settlement")?;
    drop(conn);
    let calls = endpoint.index;
    let restart = run_until_idle(&data, &mut endpoint, 1)
        .err()
        .ok_or("interrupted effect unexpectedly replayed")?;
    assert!(restart.contains("automatic replay blocked"));
    assert_eq!(endpoint.index, calls);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let recovered: String =
        conn.query_row("SELECT state FROM effect_journal", [], |row| row.get(0))?;
    assert_eq!(recovered, "recovered");
    Ok(())
}

#[test]
fn mismatched_observation_refs_roll_back_artifact_settlement() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    prepare(&conn, vec![artifact("parent", None)])?;
    let row = observation("[]");
    assert!(settle_effect_observation(&conn, "journal", "committed", &row).is_err());
    assert_unsettled(&conn)?;
    Ok(())
}

#[test]
fn orphan_artifact_intent_rolls_back_settlement() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    prepare(&conn, vec![artifact("child", Some("missing-parent"))])?;
    let row = observation("[]");
    assert!(settle_effect_observation(&conn, "journal", "committed", &row).is_err());
    assert_unsettled(&conn)?;
    Ok(())
}

fn prepare(conn: &Connection, artifacts: Vec<ArtifactRow>) -> TestResult<()> {
    let none = Option::<Vec<u8>>::None;
    let intended = Some(b"body".to_vec());
    let target = EffectTargetRevision {
        target_ordinal: 1,
        role: "main".to_string(),
        path: "out.md".to_string(),
        prior_fingerprint: stable_fingerprint(&none).map_err(|error| error.message)?,
        intended_fingerprint: stable_fingerprint(&intended).map_err(|error| error.message)?,
        prior_bytes: none,
        intended_bytes: intended,
        artifacts,
    };
    let admission = ToolAdmission {
        decision_id: "decision".to_string(),
        tool_view_fingerprint: "view".to_string(),
        action_tool: "native.write_file".to_string(),
        status: AdmissionStatus::Admitted,
        reason: "harness admitted".to_string(),
    };
    insert_admission_and_prepare(
        conn,
        &EffectPreparation {
            id: "admission",
            case_id: "1",
            admission: &admission,
            parsed_action_json: "{}",
            journal_id: "journal",
            idempotency_key: "key",
            command_ordinal: 1,
            target_path: Some("out.md"),
            prior_fingerprint: "prior",
            intended_fingerprint: "intended",
            targets: &[target],
            created_at: "now",
        },
    )?;
    mark_journal(conn, "journal", "applying", "now")?;
    Ok(())
}

fn artifact(id: &str, parent: Option<&str>) -> ArtifactRow {
    ArtifactRow {
        id: id.to_string(),
        case_id: "1".to_string(),
        kind: if parent.is_some() { "unit" } else { "file" }.to_string(),
        path: "out.md".to_string(),
        fingerprint: format!("fp-{id}"),
        parent_artifact_id: parent.map(str::to_string),
        metadata_json: "{}".to_string(),
        created_at: "now".to_string(),
    }
}

fn observation(refs: &str) -> ObservationRow {
    ObservationRow {
        id: "observation".to_string(),
        case_id: "1".to_string(),
        decision_id: "decision".to_string(),
        admission_id: Some("admission".to_string()),
        effect_name: "native.write_file".to_string(),
        status: "ok".to_string(),
        content: "done".to_string(),
        artifact_refs_json: refs.to_string(),
        contamination_class: "Clean".to_string(),
        created_at: "now".to_string(),
    }
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn assert_unsettled(conn: &Connection) -> TestResult<()> {
    let state: String = conn.query_row("SELECT state FROM effect_journal", [], |row| row.get(0))?;
    let artifacts: i64 = conn.query_row("SELECT COUNT(*) FROM artifacts", [], |row| row.get(0))?;
    let observations: i64 =
        conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    assert_eq!(state, "applying");
    assert_eq!((artifacts, observations), (0, 0));
    Ok(())
}
