mod support;

use lkjagent_store::personal::{
    create, get, list, search, update, update_status, PersonalListFilter, PersonalRecordInput,
    PersonalRecordUpdate,
};
use support::{memory_store, TestResult};

#[test]
fn schema_creates_personal_record_tables() -> TestResult<()> {
    let conn = memory_store()?;
    for table in [
        "personal_records",
        "personal_record_events",
        "personal_record_links",
        "personal_records_fts",
    ] {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name = ?1",
            [table],
            |row| row.get(0),
        )?;
        assert_eq!(count, 1, "missing {table}");
    }
    Ok(())
}

#[test]
fn diary_create_and_search_returns_stable_id() -> TestResult<()> {
    let conn = memory_store()?;
    let id = create(&conn, &diary_input())?;
    let found = search(&conn, "Chronos recovery", 5)?;

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id, id);
    assert_eq!(found[0].kind, "diary");
    assert_eq!(get(&conn, id)?.title, "Chronos recovery notes");
    Ok(())
}

#[test]
fn schedule_rejects_invalid_time_range_before_mutation() -> TestResult<()> {
    let conn = memory_store()?;
    let mut input = schedule_input();
    input.end_at = Some("2026-06-25T08:00:00+09:00");
    let result = create(&conn, &input);
    let Err(error) = result else {
        return Err("invalid range accepted".into());
    };

    assert!(error.to_string().contains("end_at must be after start_at"));
    assert!(list(&conn, &PersonalListFilter::default())?.is_empty());
    Ok(())
}

#[test]
fn todo_field_update_changes_body_and_search_index() -> TestResult<()> {
    let conn = memory_store()?;
    let id = create(&conn, &todo_input())?;
    update(
        &conn,
        &PersonalRecordUpdate {
            id,
            title: Some("Finish transition proof"),
            body: Some("Reducer proof finished with focused tests."),
            status: None,
            tags: Some("runtime proof"),
            timezone: None,
            start_at: None,
            end_at: None,
            due_at: None,
            recurrence: None,
            priority: Some("urgent"),
            project: Some("lkjagent"),
            now: "2026-06-25T09:00:00Z",
        },
    )?;

    assert_eq!(get(&conn, id)?.title, "Finish transition proof");
    assert_eq!(search(&conn, "Reducer", 5)?[0].id, id);
    Ok(())
}

#[test]
fn project_filter_limits_personal_lists() -> TestResult<()> {
    let conn = memory_store()?;
    create(&conn, &todo_input())?;
    let mut other = todo_input();
    other.title = "Other project";
    other.project = Some("other");
    create(&conn, &other)?;

    let records = list(
        &conn,
        &PersonalListFilter {
            kind: Some("todo"),
            status: None,
            project: Some("lkjagent"),
            start: None,
            end: None,
            limit: 10,
        },
    )?;

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].project.as_deref(), Some("lkjagent"));
    Ok(())
}

#[test]
fn todo_status_update_closes_record_and_appends_event() -> TestResult<()> {
    let conn = memory_store()?;
    let id = create(&conn, &todo_input())?;
    update_status(&conn, id, "done", "2026-06-25T10:00:00Z")?;
    let record = get(&conn, id)?;
    let events: i64 = conn.query_row(
        "SELECT COUNT(*) FROM personal_record_events WHERE record_id = ?1",
        [id],
        |row| row.get(0),
    )?;

    assert_eq!(record.status, "done");
    assert_eq!(record.closed_at.as_deref(), Some("2026-06-25T10:00:00Z"));
    assert_eq!(events, 2);
    Ok(())
}

fn diary_input<'a>() -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "diary",
        title: "Chronos recovery notes",
        body: "Invalid and oversized writes were refused before mutation.",
        status: "open",
        tags: "lkjagent chronos recovery",
        timezone: None,
        start_at: Some("2026-06-24"),
        end_at: None,
        due_at: None,
        recurrence: None,
        priority: None,
        project: Some("lkjagent"),
        source_case_id: Some(1),
        now: "2026-06-24T12:00:00Z",
    }
}

fn schedule_input<'a>() -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "schedule",
        title: "Review recovery smoke",
        body: "Check logs and workspace files.",
        status: "open",
        tags: "lkjagent review",
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

fn todo_input<'a>() -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: "todo",
        title: "Finish kernel proof",
        body: "Prove close paths share the central reducer.",
        status: "open",
        tags: "runtime completion",
        timezone: None,
        start_at: None,
        end_at: None,
        due_at: Some("2026-06-30T18:00:00+09:00"),
        recurrence: None,
        priority: Some("high"),
        project: Some("lkjagent"),
        source_case_id: Some(2),
        now: "2026-06-24T12:00:00Z",
    }
}
