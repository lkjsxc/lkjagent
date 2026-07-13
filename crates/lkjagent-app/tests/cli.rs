use std::fs;

use lkjagent_app::cli;
use lkjagent_store::transactions::NativeStore;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn blocked_matter_resumes_on_owner_send_unless_new_is_forced() -> TestResult<()> {
    let parent = std::env::temp_dir().join(format!("lkjagent-resume-cli-{}", std::process::id()));
    if parent.exists() {
        fs::remove_dir_all(&parent)?
    }
    let data = parent.join("data");
    let first = cli::run([
        "--data",
        data.to_str().ok_or("data path")?,
        "send",
        "initial",
    ])?;
    let matter = first
        .split_whitespace()
        .find_map(|x| x.strip_prefix("matter="))
        .ok_or("matter id")?
        .to_string();
    let mut store = NativeStore::open(data.join("lkjagent.sqlite3"))?;
    let sequence = store.next_event_sequence(&matter)?;
    store.block_budget(
        &matter,
        None,
        "blocked",
        sequence,
        2,
        "now",
        b"used=64 limit=64",
        b"block-fp",
    )?;
    drop(store);
    let resumed = cli::run([
        "--data",
        data.to_str().ok_or("data path")?,
        "send",
        "owner correction",
    ])?;
    assert!(resumed.contains(&format!("matter={matter}")) && resumed.contains("resumed=true"));
    let connection = Connection::open(data.join("lkjagent.sqlite3"))?;
    let rows:(i64,i64,String,i64,i64)=connection.query_row("SELECT (SELECT count(*) FROM matters),(SELECT count(*) FROM owner_turns),lifecycle,(SELECT count(*) FROM state_cells WHERE matter_id=?1 AND status='active'),(SELECT count(*) FROM state_cells WHERE matter_id=?1 AND status='active' AND CAST(namespace AS TEXT)='matter' AND CAST(cell_key AS TEXT)='opened') FROM matters WHERE id=?1",[matter],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?)))?;
    assert_eq!(rows, (1, 2, "open".into(), 1, 1));
    Ok(())
}

#[test]
fn removed_commands_create_no_data_or_workspace() -> TestResult<()> {
    let parent = std::env::temp_dir().join(format!("lkjagent-removed-cli-{}", std::process::id()));
    if parent.exists() {
        fs::remove_dir_all(&parent)?;
    }
    let data = parent.join("data");
    let commands = vec![
        vec!["console"],
        vec!["workbench"],
        vec!["workspace", "--rebuild"],
        vec!["workspace", "search", "query"],
        vec!["workspace", "plan-rebalance"],
        vec!["workspace", "apply-rebalance"],
        vec!["workspace", "validate"],
        vec!["log", "--follow"],
        vec!["matter", "list"],
        vec!["queue", "list"],
        vec!["context", "resolve", "matter", "key", "item"],
        vec!["record", "list"],
        vec!["memory", "query"],
        vec!["watch"],
        vec!["today", "entry"],
        vec!["journal", "entry"],
        vec!["todo", "entry"],
        vec!["calendar", "entry"],
        vec!["finance", "entry"],
        vec!["note", "entry"],
        vec!["project", "entry"],
        vec!["artifact", "entry"],
        vec!["dev", "entry"],
    ];
    for command in commands {
        let mut args = vec!["--data", data.to_str().ok_or("non-UTF-8 data path")?];
        args.extend(command);
        assert!(cli::run(args).is_err());
        assert!(!parent.exists(), "removed command created storage");
    }
    Ok(())
}
