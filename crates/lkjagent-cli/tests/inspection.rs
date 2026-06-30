mod support;

use lkjagent_cli::run_cli;
use support::{open_store, temp_data, TestResult};

#[test]
fn queue_inspection_commands_read_store_rows() -> TestResult<()> {
    let data = temp_data("queue-inspect")?;
    let first = run_cli(["--data", data.to_string_lossy().as_ref(), "send", "first"]);
    let second = run_cli(["--data", data.to_string_lossy().as_ref(), "send", "second"]);
    assert_eq!(first.code, 0);
    assert_eq!(second.code, 0);

    let list = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "list",
        "--limit",
        "1",
    ]);
    let show = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "show",
        "1",
    ]);

    assert_eq!(list.code, 0);
    assert!(list.stdout.contains("queue_rows=1"));
    assert!(list.stdout.contains("preview=second"));
    assert!(!list.stdout.contains("preview=first"));
    assert_eq!(show.code, 0);
    assert!(show.stdout.contains("queue_id=1"));
    assert!(show.stdout.contains("content=first"));
    Ok(())
}

#[test]
fn task_inspection_commands_read_graph_cases() -> TestResult<()> {
    let data = temp_data("task-inspect")?;
    let conn = open_store(&data)?;
    let opened = lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured docs.",
        "2026-06-20T00:00:00Z",
    )?;
    let case_id = opened.case_id.ok_or("missing case id")?;

    let list = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "task",
        "list",
        "--status",
        "active",
    ]);
    let show = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "task",
        "show",
        &case_id.to_string(),
    ]);

    assert_eq!(list.code, 0);
    assert!(list.stdout.contains("task_rows=1"));
    assert!(list.stdout.contains("status=active"));
    assert_eq!(show.code, 0);
    assert!(show.stdout.contains(&format!("task_id={case_id}")));
    assert!(show.stdout.contains("objective="));
    Ok(())
}
