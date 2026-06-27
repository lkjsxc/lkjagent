mod support;

use std::fs;
use std::path::Path;

use lkjagent_cli::run_cli;
use lkjagent_store::personal::{create, PersonalRecordInput};
use support::{open_store, temp_data, TestResult};

#[test]
fn personal_list_reads_store_records() -> TestResult<()> {
    let data = temp_data("personal-list")?;
    let conn = open_store(&data)?;
    let id = create(&conn, &todo("Finish projection", "lkjagent"))?;
    create(&conn, &todo("Other task", "other"))?;

    let output = run_cli(vec![
        "--data".to_string(),
        data.to_string_lossy().to_string(),
        "personal".to_string(),
        "list".to_string(),
        "--kind".to_string(),
        "todo".to_string(),
        "--project".to_string(),
        "lkjagent".to_string(),
    ]);

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("personal_records"));
    assert!(output.stdout.contains(&format!("id={id}")));
    assert!(!output.stdout.contains("Other task"));
    Ok(())
}

#[test]
fn personal_render_writes_bounded_markdown_projection() -> TestResult<()> {
    let data = temp_data("personal-render")?;
    let conn = open_store(&data)?;
    let diary_id = create(&conn, &diary())?;
    let schedule_id = create(&conn, &schedule())?;
    let todo_id = create(&conn, &todo("Finish projection", "lkjagent"))?;

    let output = run_cli(vec![
        "--data".to_string(),
        data.to_string_lossy().to_string(),
        "personal".to_string(),
        "render".to_string(),
    ]);

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("personal_projection"));
    assert_file(&data, "personal/journal/2026/06/2026-06-24.md", diary_id)?;
    assert_file(
        &data,
        &format!("personal/schedule/events/{schedule_id}-review-recovery-smoke.md"),
        schedule_id,
    )?;
    assert_file(&data, "personal/todos/open.md", todo_id)?;
    assert_file(&data, "personal/todos/projects/lkjagent.md", todo_id)?;
    Ok(())
}

fn assert_file(data: &Path, relative: &str, id: i64) -> TestResult<()> {
    let text = fs::read_to_string(data.join(relative))?;
    assert!(text.contains(&format!("id={id}")));
    assert!(text.lines().count() <= 200);
    Ok(())
}

fn diary<'a>() -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "diary",
        title: "Chronos recovery notes",
        body: "Weak content repair stayed store-backed.",
        status: "open",
        tags: "lkjagent diary",
        timezone: None,
        start_at: Some("2026-06-24"),
        end_at: None,
        due_at: None,
        recurrence: None,
        priority: None,
        project: Some("lkjagent"),
        source_case_id: None,
        now: "2026-06-24T12:00:00Z",
    }
}

fn schedule<'a>() -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "schedule",
        title: "Review recovery smoke",
        body: "Check logs.",
        status: "open",
        tags: "lkjagent schedule",
        timezone: Some("Asia/Tokyo"),
        start_at: Some("2026-06-25T09:00:00+09:00"),
        end_at: Some("2026-06-25T10:00:00+09:00"),
        due_at: None,
        recurrence: None,
        priority: None,
        project: Some("lkjagent"),
        source_case_id: None,
        now: "2026-06-24T12:00:00Z",
    }
}

fn todo<'a>(title: &'a str, project: &'a str) -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "todo",
        title,
        body: "Write projection tests.",
        status: "open",
        tags: "lkjagent todo",
        timezone: None,
        start_at: None,
        end_at: None,
        due_at: Some("2026-06-30T18:00:00+09:00"),
        recurrence: None,
        priority: Some("high"),
        project: Some(project),
        source_case_id: None,
        now: "2026-06-24T12:00:00Z",
    }
}
