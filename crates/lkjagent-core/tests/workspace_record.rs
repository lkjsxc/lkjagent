use lkjagent_core::workspace_record::{
    archive_path, default_state_for_kind, parse_record, record_fingerprint, record_path,
    render_record, slug, state_keys_for_record, WorkspaceRecord,
};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn record_round_trips_unknown_kind_and_lists() -> TestResult<()> {
    let mut record = WorkspaceRecord::new("rec_1", "strange-kind", "Odd Task", "now");
    record.tags = vec!["alpha".to_string(), "beta".to_string()];
    record.links = vec!["record:other".to_string()];
    record.state_keys = vec!["todo:open/rec_1".to_string()];
    record.body = "body text".to_string();

    let rendered = render_record(&record);
    let parsed = parse_record(&rendered)?;

    assert_eq!(parsed.id, "rec_1");
    assert_eq!(parsed.kind, "strange-kind");
    assert_eq!(parsed.tags, vec!["alpha", "beta"]);
    assert!(parsed.body.contains("body text"));
    let left = record_fingerprint(&rendered).map_err(|error| error.message)?;
    let right = record_fingerprint(&rendered).map_err(|error| error.message)?;
    assert_eq!(left, right);
    Ok(())
}

#[test]
fn record_families_emit_state_keys() -> TestResult<()> {
    assert_eq!(default_state_for_kind("project"), "active");
    assert_eq!(
        state_keys_for_record("todo", "rec_1", "open"),
        vec!["index:stale/records", "todo:open/rec_1"]
    );
    assert_eq!(
        state_keys_for_record("development", "rec_2", "open"),
        vec!["index:stale/records", "dev:repo-task/rec_2"]
    );
    Ok(())
}

#[test]
fn record_paths_reject_escapes() -> TestResult<()> {
    assert_eq!(record_path("todo", "rec_1")?, "records/todo/rec_1.md");
    assert_eq!(
        archive_path("todo", "rec_1")?,
        "records/archive/todo/rec_1.md"
    );
    assert!(record_path("../todo", "rec_1").is_err());
    assert!(record_path("todo", "../rec_1").is_err());
    assert_eq!(slug("Pay Electricity Bill!"), "pay-electricity-bill");
    Ok(())
}
