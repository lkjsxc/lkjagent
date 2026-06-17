mod support;

use lkjagent_store::queue as store_queue;
use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn queue_tools_list_and_mutate_with_store_errors() -> TestResult<()> {
    let workspace = temp_workspace("queue")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let enqueued = dispatch(
        &action(
            "queue.enqueue",
            &[("content", "draft"), ("reason", "owner-send")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(enqueued.content.contains("queued id=1"));

    let listed = dispatch(
        &action("queue.list", &[("status", "pending"), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(listed.content.contains("created_at=2026-01-01T00:00:00Z"));
    assert!(listed.content.contains("preview=draft"));

    let edited = dispatch(
        &action(
            "queue.edit",
            &[("id", "1"), ("content", "edited"), ("reason", "fix")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(edited.content.contains("edited id=1"));

    let deleted = dispatch(
        &action("queue.delete", &[("id", "1"), ("reason", "done")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(deleted.content.contains("deleted id=1"));

    let redelivered = dispatch(
        &action(
            "queue.redeliver",
            &[("id", "1"), ("reason", "retry"), ("content", "again")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(redelivered.content.contains("source_id=1"));

    let bad_status = dispatch(
        &action("queue.list", &[("status", "stale")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&bad_status));
    assert!(bad_status.content.contains("unknown queue status"));
    Ok(())
}

#[test]
fn queue_mutations_refuse_empty_and_non_pending_rows() -> TestResult<()> {
    let workspace = temp_workspace("queue-errors")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let empty = dispatch(
        &action("queue.enqueue", &[("content", ""), ("reason", "owner")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&empty));

    let id = store_queue::enqueue(&mut conn, "done", "seed", "2026-01-01T00:00:00Z")?;
    store_queue::delete(&mut conn, id, "seed", "2026-01-01T00:00:01Z")?;
    let non_pending = dispatch(
        &action(
            "queue.edit",
            &[("id", "1"), ("content", "no"), ("reason", "late")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&non_pending));
    assert!(non_pending.content.contains("not pending"));

    let missing = dispatch(
        &action("queue.redeliver", &[("id", "99"), ("reason", "missing")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&missing));
    assert!(missing.content.contains("not found"));
    Ok(())
}

#[test]
fn memory_tools_save_find_and_validate_errors() -> TestResult<()> {
    let workspace = temp_workspace("memory")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let saved = dispatch(
        &action(
            "memory.save",
            &[
                ("kind", "fact"),
                ("title", "tool runtime"),
                ("tags", "tools"),
                ("content", "tool runtime stores facts"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(saved.content.contains("memory_id=1"));

    let found = dispatch(
        &action("memory.find", &[("query", "runtime"), ("limit", "2")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(found.content.contains("kind=fact"));
    assert!(found.content.contains("snippet=tool runtime"));

    let bad_kind = dispatch(
        &action(
            "memory.save",
            &[("kind", "note"), ("title", "bad"), ("content", "bad")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&bad_kind));

    let empty_query = dispatch(
        &action("memory.find", &[("query", "")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&empty_query));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
