use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::prompt_rows::prompt_frames;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn prompt_frame_body_ref_replays_rendered_prompt() -> TestResult<()> {
    let data = fixture_root("prompt-frame")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>An agent follows checks.</message></final>".to_string()],
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
    assert_eq!(json["prompt_profile"], "kernel-v1");
    assert_eq!(json["context_profile"], "clean-context-v1");
    assert_eq!(json["card_plan"]["cards"][0]["kind"], "kernel");
    assert!(json["card_plan"]["fingerprint"]
        .as_str()
        .unwrap_or_default()
        .starts_with("fnv1a64:"));
    assert_eq!(
        json["context_plan"]["included"][0]["item_id"],
        "case-1-objective"
    );
    assert_eq!(
        json["context_plan"]["included"][0]["reason"],
        "clean-current"
    );
    assert_eq!(
        json["context_frame_fingerprint"],
        frames[0].context_frame_fingerprint
    );
    let decision_fp: String = conn.query_row(
        "SELECT context_frame_fingerprint FROM runtime_decisions WHERE id = ?1",
        [&frames[0].decision_id],
        |row| row.get(0),
    )?;
    assert_eq!(decision_fp, frames[0].context_frame_fingerprint);
    let expected_context = "case-objective [owner:1 fp=objective-1] What is an agent?";
    assert_eq!(
        frames[0].context_frame_fingerprint,
        stable_fingerprint(&expected_context).map_err(|error| error.message)?
    );
    assert_ne!(
        frames[0].context_frame_fingerprint,
        stable_fingerprint(&String::new()).map_err(|error| error.message)?
    );
    assert!(json["system"]
        .as_str()
        .unwrap_or_default()
        .contains("Expected"));
    assert!(json["user"].as_str().unwrap_or_default().contains("Plan:"));
    let cards: i64 = conn.query_row(
        "SELECT COUNT(*) FROM prompt_cards WHERE decision_id = ?1",
        [&frames[0].decision_id],
        |row| row.get(0),
    )?;
    assert_eq!(cards, 8);
    Ok(())
}

#[test]
fn prompt_context_admits_workspace_record_and_index_evidence() -> TestResult<()> {
    let data = fixture_root("prompt-workspace-context")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "todo buy milk", "now")?;
    enqueue(&conn, "What should I remember?", "later")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>Use the workspace evidence.</message></final>".to_string()],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let frames = prompt_frames(&conn, "1")?;
    assert_eq!(frames.len(), 1);
    let body = fs::read_to_string(data.join(&frames[0].body_ref))?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let system = json["system"].as_str().unwrap_or_default();
    assert!(system.contains("workspace-record:"));
    assert!(system.contains("workspace-index:"));
    assert!(system.contains("fp=fnv1a64:"));
    assert!(json["context_plan"]
        .to_string()
        .contains("workspace-record-"));
    assert!(json["context_plan"]
        .to_string()
        .contains("workspace-index-"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-prompt-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
