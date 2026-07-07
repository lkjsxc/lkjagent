use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn continuation_turn_attaches_to_open_matter() -> TestResult<()> {
    let data = fixture_root("matter-continuation")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let snapshot = instantiate(1, "create an artifact report");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    enqueue(&conn, "also add this evidence to this matter", "later")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 0)?;
    assert_eq!(endpoint.index, 0);
    assert!(snapshot.task.brief.contains("owner_update="));
    assert!(snapshot.steps[0].inputs.contains("owner_update="));

    let show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "show",
        "1",
    ])?;
    assert!(show.contains("route=existing_matter durability=matter_update"));
    assert!(show.contains("matter=1"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
