mod support;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn diary_record_and_find_use_store_ids() -> TestResult<()> {
    let workspace = temp_workspace("personal-diary")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let create = dispatch(
        &action(
            "diary.record",
            &[
                ("date", "2026-06-24"),
                ("title", "Chronos recovery"),
                (
                    "content",
                    "Invalid and oversized writes were refused before mutation.",
                ),
                ("tags", "lkjagent chronos"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    let find = dispatch(
        &action("diary.find", &[("query", "Chronos"), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(create.content.contains("personal_record_created"));
    assert!(create.content.contains("id=1"));
    assert!(find.content.contains("returned=1"));
    assert!(find.content.contains("kind=diary"));
    Ok(())
}

#[test]
fn schedule_add_list_and_update_status() -> TestResult<()> {
    let workspace = temp_workspace("personal-schedule")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    dispatch(
        &action(
            "schedule.add",
            &[
                ("title", "Review recovery smoke"),
                ("start", "2026-06-25T09:00:00+09:00"),
                ("end", "2026-06-25T10:00:00+09:00"),
                ("timezone", "Asia/Tokyo"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    let list = dispatch(
        &action("schedule.list", &[("status", "open"), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    let update = dispatch(
        &action("schedule.update", &[("id", "1"), ("status", "done")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(list.content.contains("returned=1"));
    assert!(list.content.contains("kind=schedule"));
    assert!(update.content.contains("personal_record_updated"));
    assert!(update.content.contains("status=done"));
    Ok(())
}

#[test]
fn todo_add_list_and_close() -> TestResult<()> {
    let workspace = temp_workspace("personal-todo")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    dispatch(
        &action(
            "todo.add",
            &[
                ("title", "Finish kernel proof"),
                (
                    "details",
                    "Prove every close path uses the central reducer.",
                ),
                ("due", "2026-06-30T18:00:00+09:00"),
                ("priority", "high"),
                ("project", "lkjagent"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    let list = dispatch(
        &action("todo.list", &[("status", "open"), ("project", "lkjagent")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    let update = dispatch(
        &action("todo.update", &[("id", "1"), ("status", "done")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(list.content.contains("returned=1"));
    assert!(list.content.contains("kind=todo"));
    assert!(update.content.contains("status=done"));
    Ok(())
}
