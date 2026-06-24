use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_store::graph::snapshots::{latest_compaction_snapshot, record_compaction_snapshot};
use lkjagent_store::schema::setup;
use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn latest_compaction_snapshot_survives_store_reopen() -> TestResult<()> {
    let path = temp_db_path()?;
    {
        let conn = Connection::open(&path)?;
        setup(&conn)?;
        record_compaction_snapshot(
            &conn,
            9,
            "recovery",
            "recover-by-artifact-plan",
            "Create artifact",
            &[
                "stage=pre".to_string(),
                "write_batch_cursor=a.md".to_string(),
            ],
            "2026-01-01T00:00:00Z",
        )?;
        record_compaction_snapshot(
            &conn,
            9,
            "recovery",
            "recover-by-artifact-plan",
            "Create artifact",
            &[
                "stage=post".to_string(),
                "write_batch_cursor=b.md".to_string(),
            ],
            "2026-01-01T00:00:01Z",
        )?;
    }

    let reopened = Connection::open(&path)?;
    setup(&reopened)?;
    let latest = latest_compaction_snapshot(&reopened, 9)?.ok_or("missing snapshot")?;
    fs::remove_file(path)?;

    assert_eq!(latest.phase, "recovery");
    assert_eq!(latest.active_node, "recover-by-artifact-plan");
    assert!(latest.preserved_fields.contains("stage=post"));
    assert!(latest.preserved_fields.contains("write_batch_cursor=b.md"));
    Ok(())
}

fn temp_db_path() -> TestResult<std::path::PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-store-compaction-{}-{stamp}.sqlite",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(path)
}
