use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::{deliver_answer, enqueue};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn queue_views_show_semantic_route_evidence() -> TestResult<()> {
    let data = fixture_root("queue-routing")?;
    let cases = [
        (
            "create an artifact report from these notes",
            "route=artifact_request durability=runtime_decision transform=true",
        ),
        (
            "show the current status",
            "route=inspection durability=read_only_report transform=false",
        ),
        (
            "run cargo test and report failures",
            "route=system_operation durability=runtime_decision transform=false",
        ),
        (
            "plan a summer trip",
            "route=new_matter durability=matter transform=true",
        ),
    ];
    for (text, _) in cases {
        cli::run(["--data", data.to_string_lossy().as_ref(), "send", text])?;
    }
    let list = cli::run(["--data", data.to_string_lossy().as_ref(), "queue", "list"])?;
    for (_, expected) in cases {
        assert!(list.contains(expected), "missing {expected} in {list}");
    }
    Ok(())
}

#[test]
fn inspection_route_closes_without_endpoint_call() -> TestResult<()> {
    let data = fixture_root("queue-inspection-effect")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "show the current status", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert_eq!(endpoint.index, 0);
    assert!(snapshot
        .task
        .summary
        .contains("inspection: pending_queue=0"));
    Ok(())
}

#[test]
fn artifact_request_writes_verified_artifact() -> TestResult<()> {
    let data = fixture_root("queue-artifact-effect")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "create an artifact report from these notes", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<content># Artifact\n\nA verified report.</content>".to_string(),
            "<final><message>artifact ready: artifacts/requests/matter-1.md</message></final>"
                .to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 4)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(data
        .join("workspace/artifacts/requests/matter-1.md")
        .exists());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert!(artifact_count(&conn)? >= 1);
    assert!(check_count(&conn)? >= 1);
    assert!(snapshot
        .task
        .summary
        .contains("artifacts/requests/matter-1.md"));
    Ok(())
}

#[test]
fn artifact_request_requires_response_path() -> TestResult<()> {
    let data = fixture_root("queue-artifact-response-path")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "create an artifact report from these notes", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<content># Artifact\n\nA verified report.</content>".to_string(),
            "<final><message>artifact ready</message></final>".to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 4)?;
    assert_eq!(snapshot.task.state, TaskState::Blocked);
    assert!(snapshot
        .task
        .summary
        .contains("artifact response path missing"));
    Ok(())
}

#[test]
fn artifact_request_does_not_close_without_output() -> TestResult<()> {
    let data = fixture_root("queue-artifact-missing")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "create an artifact report from these notes", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>promised</message></final>".to_string(); 3],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 3)?;
    assert_ne!(snapshot.task.state, TaskState::Closed);
    assert!(!data
        .join("workspace/artifacts/requests/matter-1.md")
        .exists());
    Ok(())
}

#[test]
fn system_operation_route_blocks_without_endpoint_call() -> TestResult<()> {
    let data = fixture_root("queue-system-effect")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "run cargo test and report failures", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(snapshot.task.state, TaskState::Blocked);
    assert_eq!(endpoint.index, 0);
    assert!(snapshot.task.summary.contains("unsupported_executor"));
    Ok(())
}

#[test]
fn delivered_answers_refresh_existing_matter_route() -> TestResult<()> {
    let data = fixture_root("queue-answer-routing")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "README.md", "now")?;
    let row = deliver_answer(&conn, 7, "later")?.ok_or("answer not delivered")?;
    assert_eq!(row.route_lane.as_deref(), Some("existing_matter"));
    assert_eq!(row.route_durability.as_deref(), Some("queue_answer"));
    assert_eq!(row.route_transform_allowed, Some(false));
    drop(conn);

    let show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "show",
        "1",
    ])?;
    assert!(show.contains("route=existing_matter durability=queue_answer transform=false"));
    assert!(show.contains("matter=7"));
    Ok(())
}

fn artifact_count(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM artifacts", [], |row| row.get(0))
}

fn check_count(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM check_results", [], |row| row.get(0))
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
