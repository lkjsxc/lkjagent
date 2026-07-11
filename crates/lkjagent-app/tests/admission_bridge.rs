use std::{
    fs,
    path::{Path, PathBuf},
};

use lkjagent_app::persist_tool_admissions;
use lkjagent_core::engine::Command;
use lkjagent_core::parse::Action;
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn repeat_guard_rejects_previously_admitted_action() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let decision = decision();
    let command = Command::RunExplore(action("fs.read", "path", "README.md"));

    persist_tool_admissions(
        &conn,
        Path::new("."),
        &decision,
        std::slice::from_ref(&command),
        "now",
    )?;
    let error = match persist_tool_admissions(&conn, Path::new("."), &decision, &[command], "later")
    {
        Err(error) => error,
        Ok(_) => return Err("repeated action was admitted".into()),
    };

    assert!(error.contains("repeated tool call"));
    assert_eq!(admission_count(&conn, "Admitted")?, 1);
    assert_eq!(admission_count(&conn, "Rejected")?, 1);
    Ok(())
}

#[test]
fn native_workspace_effect_has_harness_admission_and_prepared_journal() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let prepared = persist_tool_admissions(
        &conn,
        Path::new("."),
        &decision(),
        &[Command::WriteFile {
            path: "notes.md".to_string(),
            content: "durable".to_string(),
        }],
        "now",
    )?;
    let row: (String, String) = conn.query_row(
        "SELECT action_tool, state FROM tool_admissions JOIN effect_journal
         ON effect_journal.admission_id = tool_admissions.id",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(prepared.len(), 1);
    assert_eq!(
        row,
        ("native.write_file".to_string(), "prepared".to_string())
    );
    Ok(())
}

#[test]
fn model_workspace_write_prepares_target_fingerprints() -> TestResult<()> {
    let workspace = fixture_root()?;
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let prepared = persist_tool_admissions(
        &conn,
        &workspace,
        &write_decision(),
        &[Command::RunExplore(Action {
            tool: "fs.write".to_string(),
            params: vec![
                ("path".to_string(), "note.md".to_string()),
                ("content".to_string(), "body".to_string()),
            ],
        })],
        "now",
    )?;
    let row: (String, String) = conn.query_row(
        "SELECT target_path, intended_fingerprint FROM effect_journal",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(
        prepared
            .first()
            .and_then(|effect| effect.target_path.as_deref()),
        Some("note.md")
    );
    assert_eq!(row.0, "note.md");
    assert!(row.1.starts_with("fnv1a64:"));
    Ok(())
}

#[test]
fn mismatch_reason_persists_for_hidden_tool() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let error = match persist_tool_admissions(
        &conn,
        Path::new("."),
        &decision(),
        &[Command::RunExplore(action("shell.run", "cmd", "date"))],
        "now",
    ) {
        Err(error) => error,
        Ok(_) => return Err("hidden tool was admitted".into()),
    };

    assert!(error.contains("tool-view mismatch"));
    let result: String = conn.query_row("SELECT result_json FROM tool_admissions", [], |row| {
        row.get(0)
    })?;
    assert!(result.contains("tool-view mismatch"));
    Ok(())
}

fn decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    )
}

fn write_decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "write-decision",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![ToolViewEntry::new("fs.write", "write")
            .with_params(vec!["path", "content"], Vec::new())]),
        OutputEnvelope::Action,
    )
}

fn fixture_root() -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-admission-write-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn action(tool: &str, name: &str, value: &str) -> Action {
    Action {
        tool: tool.to_string(),
        params: vec![(name.to_string(), value.to_string())],
    }
}

fn admission_count(conn: &Connection, status: &str) -> TestResult<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM tool_admissions WHERE status = ?1",
        [status],
        |row| row.get(0),
    )?)
}
