use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_admission::{AdmissionStatus, ToolAdmission};
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, mark_journal, EffectPreparation,
};
use lkjagent_store::observation_rows::{settle_effect_observation, ObservationRow};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn accepted_explore_effect_has_prepared_journal_and_linked_observation() -> TestResult<()> {
    let data = fixture_root("effect-journal")?;
    fs::create_dir_all(data.join("workspace"))?;
    fs::write(data.join("workspace/probe.txt"), "journal evidence")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![support::action_chars("fs.read", &[('p', "probe.txt")])],
        index: 0,
    };

    run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let journal: (String, String, String) = conn.query_row(
        "SELECT admission_id, state, outcome_fingerprint FROM effect_journal",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let observed: String = conn.query_row("SELECT admission_id FROM observations", [], |row| {
        row.get(0)
    })?;
    assert_eq!(journal.0, observed);
    assert_eq!(journal.1, "committed");
    assert!(journal.2.starts_with("fnv1a64:"));
    Ok(())
}

#[test]
fn failed_explore_effect_has_failed_journal_and_error_observation() -> TestResult<()> {
    let data = fixture_root("effect-journal-failure")?;
    fs::create_dir_all(data.join("workspace"))?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![support::action_chars("fs.read", &[('p', "missing.txt")])],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (String, String) = conn.query_row(
        "SELECT effect_journal.state, observations.status FROM effect_journal
         JOIN observations ON observations.id = effect_journal.observation_id",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row, ("failed".to_string(), "error".to_string()));
    Ok(())
}

#[test]
fn startup_settles_unresolved_effects_once_without_replay() -> TestResult<()> {
    let data = fixture_root("effect-journal-recovery")?;
    fs::create_dir_all(data.join("workspace"))?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    prepare(&conn, "prepared", 1)?;
    prepare(&conn, "applying", 2)?;
    mark_journal(&conn, "applying-effect", "applying", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;
    run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let states = conn
        .prepare("SELECT state FROM effect_journal ORDER BY id")?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    let observations: i64 =
        conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    assert_eq!(states, ["failed", "recovered"]);
    assert_eq!(observations, 2);
    Ok(())
}

#[test]
fn settlement_binds_one_immutable_observation() -> TestResult<()> {
    let data = fixture_root("effect-journal-settlement")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    prepare(&conn, "once", 1)?;
    mark_journal(&conn, "once-effect", "applying", "now")?;
    let first = observation("first", "once-admission");
    settle_effect_observation(&mut conn, "once-effect", "committed", &first)?;
    let second = observation("second", "once-admission");
    assert!(settle_effect_observation(&mut conn, "once-effect", "committed", &second).is_err());
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    let bound: String = conn.query_row(
        "SELECT observation_id FROM effect_journal WHERE id = 'once-effect'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 1);
    assert_eq!(bound, "first");
    Ok(())
}

fn observation(id: &str, admission_id: &str) -> ObservationRow {
    ObservationRow {
        id: id.to_string(),
        case_id: "recovery-case".to_string(),
        decision_id: "recovery-decision".to_string(),
        admission_id: Some(admission_id.to_string()),
        effect_name: "fs.once".to_string(),
        status: "ok".to_string(),
        content: "settled".to_string(),
        artifact_refs_json: "[]".to_string(),
        contamination_class: "Clean".to_string(),
        created_at: "now".to_string(),
    }
}

fn prepare(conn: &Connection, id: &str, ordinal: i64) -> TestResult<()> {
    let admission = ToolAdmission {
        decision_id: "recovery-decision".to_string(),
        tool_view_fingerprint: "view".to_string(),
        action_tool: format!("fs.{id}"),
        status: AdmissionStatus::Admitted,
        reason: "harness admitted".to_string(),
    };
    let journal_id = format!("{id}-effect");
    insert_admission_and_prepare(
        conn,
        &EffectPreparation {
            id: &format!("{id}-admission"),
            case_id: "recovery-case",
            admission: &admission,
            parsed_action_json: "{}",
            journal_id: &journal_id,
            idempotency_key: &format!("recovery:{ordinal}"),
            command_ordinal: ordinal,
            target_path: None,
            prior_fingerprint: "prior",
            intended_fingerprint: "intended",
            created_at: "now",
        },
    )?;
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
