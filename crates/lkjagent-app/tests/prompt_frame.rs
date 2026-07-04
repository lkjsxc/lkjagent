use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::prompt_rows::prompt_frames;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn prompt_frame_body_ref_replays_rendered_prompt() -> TestResult<()> {
    let data = fixture_root("prompt-frame")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>An agent follows checks.</message>".to_string()],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let frames = prompt_frames(&conn, "1")?;
    assert_eq!(frames.len(), 1);
    let body = fs::read_to_string(data.join(&frames[0].body_ref))?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    assert_eq!(json["decision_id"], frames[0].decision_id);
    assert_eq!(json["prompt_fingerprint"], frames[0].prompt_fingerprint);
    assert!(json["system"]
        .as_str()
        .unwrap_or_default()
        .contains("Expected"));
    assert!(json["user"].as_str().unwrap_or_default().contains("Plan:"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-prompt-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
