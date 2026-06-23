mod support;

use lkjagent_store::artifact_cursor::{latest_batch_cursor, upsert_batch_cursor, BatchCursorInput};
use support::{memory_store, TestResult};

#[test]
fn batch_cursor_upserts_by_artifact_and_root() -> TestResult<()> {
    let conn = memory_store()?;
    let first_paths = vec!["a.md".to_string(), "b.md".to_string()];
    let second_paths = vec!["c.md".to_string()];
    upsert_batch_cursor(&conn, &cursor(9, &first_paths, 1, "example-a"))?;
    upsert_batch_cursor(&conn, &cursor(9, &second_paths, 2, "example-b"))?;

    let row = latest_batch_cursor(&conn, 9)?.ok_or("missing cursor")?;
    assert_eq!(row.root, "cookbooks/bread");
    assert_eq!(row.planned_paths, "c.md");
    assert_eq!(row.current_index, 2);
    assert_eq!(row.last_valid_example, "example-b");
    Ok(())
}

fn cursor<'a>(
    artifact_ledger_id: i64,
    paths: &'a [String],
    current_index: i64,
    example: &'a str,
) -> BatchCursorInput<'a> {
    BatchCursorInput {
        artifact_ledger_id,
        root: "cookbooks/bread",
        planned_paths: paths,
        completed_paths: &[],
        failed_paths: &[],
        current_index,
        last_valid_example: example,
        retry_counts: "none",
        fallback_mode: "batch-write",
        updated_at: "2026-01-01T00:00:00Z",
    }
}
