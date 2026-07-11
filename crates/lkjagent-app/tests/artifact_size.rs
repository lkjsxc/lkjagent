use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn long_generated_artifact_writes_manifest_and_checked_parts() -> TestResult<()> {
    let data = fixture_root("artifact-size")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "create an artifact report from these notes", "now")?;
    drop(conn);
    let body = long_body(900);
    let authored = format!("# Long Artifact\n\n{body}");
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            format!("<content>{authored}</content>"),
            "<message>artifact ready: artifacts/requests/matter-1.md</message>".to_string(),
        ],
        index: 0,
    };

    let snapshot = run_until_idle(&data, &mut endpoint, 4)?;

    assert_eq!(snapshot.task.state, TaskState::Closed);
    let workspace = data.join("workspace");
    let manifest_path = workspace.join("artifacts/requests/matter-1.md");
    let manifest = fs::read_to_string(&manifest_path)?;
    assert!(manifest.contains("Size justification:"));
    assert!(manifest.contains("matter-1.parts/part-001.md"));
    assert!(manifest.len() < body.len());
    assert!(workspace
        .join("artifacts/requests/matter-1.parts/part-001.md")
        .exists());
    assert!(workspace
        .join("artifacts/requests/matter-1.parts/part-002.md")
        .exists());
    let mut parts = fs::read_dir(workspace.join("artifacts/requests/matter-1.parts"))?
        .collect::<Result<Vec<_>, _>>()?;
    parts.sort_by_key(|entry| entry.path());
    let reconstructed = parts
        .iter()
        .map(|entry| fs::read_to_string(entry.path()))
        .collect::<Result<Vec<_>, _>>()?
        .concat();
    assert_eq!(reconstructed, authored);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let parent_metadata: String = conn.query_row(
        "SELECT metadata_json FROM artifacts WHERE kind = 'file' LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert!(parent_metadata.contains("size_justification"));
    let unit_part_paths: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artifacts WHERE kind = 'unit' AND path LIKE '%matter-1.parts/part-%'",
        [],
        |row| row.get(0),
    )?;
    assert!(unit_part_paths >= 2);
    let targets: i64 =
        conn.query_row("SELECT COUNT(*) FROM effect_target_revisions", [], |row| {
            row.get(0)
        })?;
    let refs: String = conn.query_row(
        "SELECT artifact_refs_json FROM observations
         WHERE effect_name = 'native.write_file' LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert!(targets >= 3);
    assert_ne!(refs, "[]");
    Ok(())
}

fn long_body(words: usize) -> String {
    (0..words)
        .map(|index| format!("{}word{index}", if index % 75 == 0 { "\n  " } else { " " }))
        .collect::<String>()
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
