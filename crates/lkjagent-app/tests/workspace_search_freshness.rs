use std::{fs, path::PathBuf};

use lkjagent_app::cli;
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
