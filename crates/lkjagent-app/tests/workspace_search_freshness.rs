use std::{fs, path::PathBuf};

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::workspace_search::canonical_rows;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn case_insensitive_match_is_centered_in_excerpt() -> TestResult<()> {
    let data = fixture_root("excerpt")?;
    fs::create_dir_all(data.join("workspace/knowledge"))?;
    fs::write(
        data.join("workspace/knowledge/case.md"),
        format!("{}NEEDLE excerpt target", "prefix ".repeat(100)),
    )?;
    let data_arg = data.to_string_lossy();
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;
    let found = cli::run(["--data", data_arg.as_ref(), "workspace", "search", "needle"])?;
    assert!(found
        .lines()
        .any(|line| line.contains("NEEDLE excerpt target")));
    Ok(())
}

#[test]
fn stale_first_page_does_not_hide_current_lower_ranked_hit() -> TestResult<()> {
    let data = fixture_root("stale")?;
    let workspace = data.join("workspace/knowledge");
    fs::create_dir_all(&workspace)?;
    for index in 0..60 {
        fs::write(
            workspace.join(format!("doc-{index:03}.md")),
            "common freshness needle",
        )?;
    }
    let data_arg = data.to_string_lossy();
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let mut body = canonical_rows(&conn)?
        .into_iter()
        .filter(|row| row.field == "body" && row.content.contains("freshness"))
        .collect::<Vec<_>>();
    body.sort_by(|left, right| left.document_id.cmp(&right.document_id));
    let keep = body.last().ok_or("missing body chunks")?.path.clone();
    drop(conn);
    for row in body {
        if row.path != keep {
            fs::write(data.join("workspace").join(row.path), "changed stale bytes")?;
        }
    }
    let found = cli::run([
        "--data",
        data_arg.as_ref(),
        "workspace",
        "search",
        "freshness",
    ])?;
    assert!(found.contains(&format!("path={keep}")));
    assert!(found.contains("excluded_drifted=59"));
    Ok(())
}

#[test]
fn unchanged_inventory_is_debounced_until_manifest_changes() -> TestResult<()> {
    let data = fixture_root("debounce")?;
    fs::write(
        data.join("lkjagent.json"),
        r#"{"workspace_scan_debounce_milliseconds":50,"workspace_reconcile_seconds":30}"#,
    )?;
    let source = data.join("workspace/knowledge/debounce.md");
    fs::create_dir_all(source.parent().ok_or("source parent missing")?)?;
    fs::write(&source, "# Before\n\nstable inventory")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 0)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_redundant_scan BEFORE DELETE ON workspace_search_chunks
        BEGIN SELECT RAISE(FAIL, 'scan should be debounced'); END;",
    )?;
    drop(conn);
    run_until_idle(&data, &mut endpoint, 0)?;
    fs::write(&source, "# After\n\nchanged inventory")?;
    std::thread::sleep(std::time::Duration::from_millis(60));
    assert!(run_until_idle(&data, &mut endpoint, 0).is_err());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute("DROP TRIGGER fail_redundant_scan", [])?;
    drop(conn);
    run_until_idle(&data, &mut endpoint, 0)?;
    let arg = data.to_string_lossy();
    let found = cli::run(["--data", arg.as_ref(), "workspace", "search", "changed"])?;
    assert!(found.contains("knowledge/debounce.md"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-search-freshness-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
