use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_app::{cli, console};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn console_line_handler_routes_owner_input_without_daemon_state() -> TestResult<()> {
    let data = fixture_root("console")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;

    let sent = console::handle_line(&conn, "hello from console", "now")?;
    assert_eq!(sent.output, "queue: 1 new=false");
    assert!(!sent.quit);
    assert!(console::handle_line(&conn, "/status", "now")?
        .output
        .contains("daemon:"));
    assert!(console::handle_line(&conn, "/new fresh matter", "now")?
        .output
        .contains("new=true"));
    assert!(console::handle_line(&conn, "/help", "now")?
        .output
        .contains("/send TEXT"));
    assert!(console::handle_line(&conn, "/quit", "now")?.quit);
    Ok(())
}

#[test]
fn record_cli_manages_generic_records_while_daemon_is_stopped() -> TestResult<()> {
    let data = fixture_root("records")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Odd",
        "Work",
        "--body",
        "body",
    ])?;
    assert!(added.contains("record: rec_"));
    let id = added
        .split_whitespace()
        .nth(1)
        .ok_or("missing record id")?
        .to_string();
    let old_path = added
        .split(" path=")
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("");
    assert!(cli::run(["--data", data_arg.as_ref(), "record", "list"])?.contains(&id));
    assert!(cli::run(["--data", data_arg.as_ref(), "record", "show", &id])?.contains("body"));
    assert!(cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "link",
        &id,
        "record:other"
    ])?
    .contains("linked record"));
    assert!(
        cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?
            .contains("archived record")
    );
    assert!(!cli::run(["--data", data_arg.as_ref(), "record", "list"])?.contains(&id));
    assert!(
        cli::run(["--data", data_arg.as_ref(), "record", "show", old_path])?
            .contains("state=archived")
    );
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let counts: (i64, i64) = conn.query_row("SELECT (SELECT COUNT(*) FROM workspace_path_aliases), (SELECT COUNT(*) FROM workspace_rebalance_audit)", [], |row| Ok((row.get(0)?, row.get(1)?)))?;
    assert_eq!(counts, (1, 1));
    Ok(())
}

#[test]
fn cli_inspection_reads_store_rows() -> TestResult<()> {
    let data = fixture_root("cli")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "What is an agent?", "now")?;
    conn.execute(
        "INSERT INTO memory (topic, content, created_at) VALUES ('agent', 'row memory', 'now')",
        [],
    )?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>done</message></final>".to_string()],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 3)?;
    let matter_list = cli::run(["--data", data.to_string_lossy().as_ref(), "matter", "list"])?;
    assert!(matter_list.contains("matter 1 closed"));
    let matter_show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "matter",
        "show",
        "1",
    ])?;
    assert!(matter_show.contains("matter 1 Closed"));
    assert!(matter_show.contains("state: active="));
    assert!(matter_show.contains("decisions:"));
    assert!(matter_show.contains("prompt_frames: 1"));
    assert!(matter_show.contains("checks: total=0"));
    assert!(matter_show.contains("exchanges: 1"));
    let watch = cli::run(["--data", data.to_string_lossy().as_ref(), "watch"])?;
    assert!(watch.contains("== status =="));
    assert!(watch.contains("== recent events =="));
    assert!(watch.contains("matter 1 Closed"));
    assert!(watch.contains("proof: prompt_frames="));
    assert!(watch.contains("exchanges=1"));
    assert!(cli::run(["--data", data.to_string_lossy().as_ref(), "log"])?.contains("taskclosed"));
    assert!(
        cli::run(["--data", data.to_string_lossy().as_ref(), "memory", "row"])?
            .contains("row memory")
    );
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-cli-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
