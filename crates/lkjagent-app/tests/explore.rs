use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn explore_registry_runs_bounded_workspace_tools() -> TestResult<()> {
    let data = fixture_root("registry")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            action("fs.write", &[('p', "probe.txt"), ('c', "hello aurora")]),
            action("fs.read", &[('p', "probe.txt")]),
            memory_save("probe", "hello aurora"),
            finish("read probe"),
            "<message>done</message>".to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 6)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(data.join("workspace/probe.txt").exists());
    assert!(snapshot.steps[0].inputs.contains("<observation>"));
    assert!(snapshot.steps[0].inputs.contains("saved topic=probe"));
    assert!(cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "memory",
        "aurora"
    ])?
    .contains("hello aurora"));
    Ok(())
}

fn action(tool: &str, params: &[(char, &str)]) -> String {
    let mut body = format!("<tool>{tool}</tool>");
    for (kind, value) in params {
        let name = if *kind == 'p' { "path" } else { "content" };
        body.push_str(&format!("<{name}>{value}</{name}>"));
    }
    format!("<action>{body}</action>")
}

fn memory_save(topic: &str, content: &str) -> String {
    format!(
        "<action><tool>memory.save</tool><topic>{topic}</topic><content>{content}</content></action>"
    )
}

fn finish(summary: &str) -> String {
    format!("<action><tool>finish</tool><summary>{summary}</summary></action>")
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-explore-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
