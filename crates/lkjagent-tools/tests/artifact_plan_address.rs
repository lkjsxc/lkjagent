mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_plan_refuses_markdown_leaf_root_before_ledger_identity() -> TestResult<()> {
    let workspace = temp_workspace("artifact-plan-md-root")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", "stories/sf-novel/characters.md"),
                ("title", "Characters"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("artifact address refused"));
    assert!(output.contains("address_status=root_ends_with_markdown_suffix"));
    assert_eq!(artifact_rows(&conn)?, 0);
    Ok(())
}

#[test]
fn artifact_plan_accepts_directory_shaped_missing_root() -> TestResult<()> {
    let workspace = temp_workspace("artifact-plan-dir-root")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.plan",
            &[
                ("root", "stories/sf-novel"),
                ("title", "SF Novel"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("document plan created"));
    assert_eq!(artifact_rows(&conn)?, 1);
    Ok(())
}

fn artifact_rows(conn: &rusqlite::Connection) -> TestResult<i64> {
    Ok(conn.query_row("SELECT COUNT(*) FROM artifact_ledger", [], |row| row.get(0))?)
}
