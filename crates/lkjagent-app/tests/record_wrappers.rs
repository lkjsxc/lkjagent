use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn friendly_wrappers_write_generic_records() -> TestResult<()> {
    let data = fixture_root("record-wrappers")?;

    let added = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "todo",
        "Buy",
        "milk",
    ])?;
    assert!(added.contains("path=records/todo/"));
    let id = added.split_whitespace().nth(1).ok_or("missing id")?;

    let listed = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "record",
        "list",
        "todo",
    ])?;
    assert!(listed.contains("kind=todo"));
    assert!(listed.contains("title=Buy milk"));

    let shown = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "record",
        "show",
        id,
    ])?;
    assert!(shown.contains("kind: todo"));
    assert!(shown.contains("## Body\n\nBuy milk"));

    let dev = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "dev",
        "Fix",
        "parser",
    ])?;
    assert!(dev.contains("path=records/development/"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
