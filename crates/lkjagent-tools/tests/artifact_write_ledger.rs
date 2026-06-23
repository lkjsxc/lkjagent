mod support;

use lkjagent_store::artifact_cursor::latest_batch_cursor;
use lkjagent_store::artifact_ledger::latest_for_case;
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn fs_write_marks_artifact_cursor_path_completed() -> TestResult<()> {
    let (runtime, mut conn, mut dispatch_state, artifact_id) = prepared_cursor("artifact-write")?;
    let cursor = latest_batch_cursor(&conn, artifact_id)?.ok_or("missing cursor")?;
    let path = first_path(&cursor.planned_paths)?;

    dispatch(
        &action(
            "fs.write",
            &[("path", path), ("content", meaningful_content())],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let updated = latest_batch_cursor(&conn, artifact_id)?.ok_or("missing updated cursor")?;
    assert!(updated.completed_paths.contains(path));
    Ok(())
}

#[test]
fn fs_batch_write_marks_artifact_cursor_paths_completed() -> TestResult<()> {
    let (runtime, mut conn, mut dispatch_state, artifact_id) = prepared_cursor("artifact-batch")?;
    let cursor = latest_batch_cursor(&conn, artifact_id)?.ok_or("missing cursor")?;
    let files = cursor
        .planned_paths
        .lines()
        .take(2)
        .map(|path| format!("path: {path}\ncontent:\n{}", meaningful_content()))
        .collect::<Vec<_>>()
        .join("\n-- lkjagent-next-file --\n");

    dispatch(
        &action("fs.batch_write", &[("files", &files)]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );

    let updated = latest_batch_cursor(&conn, artifact_id)?.ok_or("missing updated cursor")?;
    for path in cursor.planned_paths.lines().take(2) {
        assert!(updated.completed_paths.contains(path));
    }
    Ok(())
}

fn prepared_cursor(
    name: &str,
) -> TestResult<(
    lkjagent_tools::dispatch::ToolRuntime,
    rusqlite::Connection,
    lkjagent_tools::dispatch::DispatchState,
    i64,
)> {
    let workspace = temp_workspace(name)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let root = "cookbooks/bread";
    dispatch(
        &action(
            "artifact.apply",
            &[("root", root), ("title", "Bread"), ("kind", "cookbook")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    dispatch_state.reset_repeat_tracking();
    dispatch(
        &action("artifact.next", &[("root", root), ("kind", "cookbook")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    );
    let artifact = latest_for_case(&conn, 0)?.ok_or("missing artifact")?;
    Ok((runtime, conn, dispatch_state, artifact.id))
}

fn first_path(paths: &str) -> TestResult<&str> {
    paths
        .lines()
        .next()
        .ok_or_else(|| "missing planned path".into())
}

fn meaningful_content() -> &'static str {
    "# Rice Foundation\n\n## Purpose\n\nThis section gives concrete Japanese cooking guidance with ingredients, timing, and verification details.\n"
}
