use std::fs;

use lkjagent_app::persist_tool_admissions;
use lkjagent_core::engine::Command;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn equal_content_at_different_paths_has_distinct_artifact_identity() -> TestResult<()> {
    let workspace =
        std::env::temp_dir().join(format!("lkjagent-artifact-id-{}", std::process::id()));
    if workspace.exists() {
        fs::remove_dir_all(&workspace)?;
    }
    fs::create_dir_all(&workspace)?;
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let decision = RuntimeDecision::new(
        "decision",
        "1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::Action,
    );
    let prepared = persist_tool_admissions(
        &conn,
        &workspace,
        &decision,
        &[
            Command::WriteFile {
                path: "a.md".to_string(),
                content: "same".to_string(),
            },
            Command::WriteFile {
                path: "b.md".to_string(),
                content: "same".to_string(),
            },
        ],
        "now",
    )?;
    let parent = |index: usize| {
        prepared[index]
            .targets
            .iter()
            .flat_map(|target| &target.artifacts)
            .find(|artifact| artifact.parent_artifact_id.is_none())
            .map(|artifact| artifact.id.clone())
    };
    assert_ne!(parent(0), parent(1));
    Ok(())
}

#[test]
fn later_planning_failure_rolls_back_turn_admissions() -> TestResult<()> {
    let workspace = std::env::temp_dir().join(format!(
        "lkjagent-artifact-plan-rollback-{}",
        std::process::id()
    ));
    if workspace.exists() {
        fs::remove_dir_all(&workspace)?;
    }
    fs::create_dir_all(workspace.join("notes.parts"))?;
    fs::write(workspace.join("notes.parts/part-001.md"), "owner")?;
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let decision = RuntimeDecision::new(
        "rollback-decision",
        "1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::Action,
    );
    let long = (0..500)
        .map(|index| format!("word{index} "))
        .collect::<String>();
    let commands = [
        Command::WriteFile {
            path: "first.md".to_string(),
            content: "valid".to_string(),
        },
        Command::WriteFile {
            path: "notes.md".to_string(),
            content: long,
        },
    ];
    assert!(persist_tool_admissions(&conn, &workspace, &decision, &commands, "now").is_err());
    let admissions: i64 =
        conn.query_row("SELECT COUNT(*) FROM tool_admissions", [], |row| row.get(0))?;
    assert_eq!(admissions, 0);
    Ok(())
}

#[test]
fn overlapping_bundle_targets_are_rejected_before_admission() -> TestResult<()> {
    let workspace =
        std::env::temp_dir().join(format!("lkjagent-artifact-overlap-{}", std::process::id()));
    if workspace.exists() {
        fs::remove_dir_all(&workspace)?;
    }
    fs::create_dir_all(&workspace)?;
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let decision = RuntimeDecision::new(
        "overlap-decision",
        "1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::Action,
    );
    let commands = [
        Command::WriteFile {
            path: "ancestor".to_string(),
            content: "first".to_string(),
        },
        Command::AppendFile {
            path: "ancestor/child.md".to_string(),
            content: "second".to_string(),
        },
    ];
    assert!(persist_tool_admissions(&conn, &workspace, &decision, &commands, "now").is_err());
    let admissions: i64 =
        conn.query_row("SELECT COUNT(*) FROM tool_admissions", [], |row| row.get(0))?;
    assert_eq!(admissions, 0);
    Ok(())
}
