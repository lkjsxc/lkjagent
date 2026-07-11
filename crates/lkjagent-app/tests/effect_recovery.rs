use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_admission::{AdmissionStatus, ToolAdmission};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, mark_journal, EffectPreparation,
};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn startup_recovers_applying_write_when_target_matches_intended_bytes() -> TestResult<()> {
    let data = fixture_root()?;
    let workspace = data.join("workspace");
    fs::create_dir_all(&workspace)?;
    fs::write(workspace.join("note.md"), "completed")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let admission = ToolAdmission {
        decision_id: "decision".to_string(),
        tool_view_fingerprint: "view".to_string(),
        action_tool: "native.write_file".to_string(),
        status: AdmissionStatus::Admitted,
        reason: "harness admitted".to_string(),
    };
    let intended =
        stable_fingerprint(&"completed").map_err(|error| std::io::Error::other(error.message))?;
    let prior = stable_fingerprint(&Option::<Vec<u8>>::None)
        .map_err(|error| std::io::Error::other(error.message))?;
    insert_admission_and_prepare(
        &conn,
        &EffectPreparation {
            id: "admission",
            case_id: "case",
            admission: &admission,
            parsed_action_json: "{}",
            journal_id: "journal",
            idempotency_key: "key",
            command_ordinal: 1,
            target_path: Some("note.md"),
            prior_fingerprint: &prior,
            intended_fingerprint: &intended,
            created_at: "now",
        },
    )?;
    mark_journal(&conn, "journal", "applying", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let state: String = conn.query_row(
        "SELECT state FROM effect_journal WHERE id = 'journal'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(state, "recovered");
    Ok(())
}

fn fixture_root() -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-effect-recovery-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
