use std::{
    fs,
    path::{Path, PathBuf},
};

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::record;
use lkjagent_store::workspace_search::canonical_rows;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn visible_external_files_reconcile_with_bounded_equivalent_results() -> TestResult<()> {
    let data = fixture_root("visible")?;
    let workspace = data.join("workspace");
    fs::create_dir_all(workspace.join("knowledge"))?;
    fs::create_dir_all(workspace.join("system"))?;
    fs::create_dir_all(workspace.join("System"))?;
    fs::create_dir_all(workspace.join("archive"))?;
    fs::create_dir_all(workspace.join("indexes"))?;
    let guide = format!(
        "# Unicode Guide\n\n{} aurora inventory tail",
        "界".repeat(1_500)
    );
    fs::write(workspace.join("knowledge/guide.md"), guide)?;
    fs::write(workspace.join("system/private.md"), "aurora system")?;
    fs::write(workspace.join("System/upper.md"), "aurora uppercase system")?;
    fs::write(workspace.join("archive/old.md"), "aurora archive")?;
    fs::write(workspace.join("indexes/derived.md"), "aurora index")?;
    fs::write(
        workspace.join("knowledge/boundary.md"),
        format!("{} needle boundary", "x".repeat(2_044)),
    )?;
    let outside = data.join("outside.md");
    fs::write(&outside, "aurora outside")?;
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, workspace.join("knowledge/link.md"))?;

    rebuild(&data)?;
    let first_output = search(&data, "aurora", &[])?;
    assert!(first_output.contains("path=knowledge/guide.md"));
    assert!(!first_output.contains("private.md"));
    assert!(!first_output.contains("old.md"));
    assert!(!first_output.contains("derived.md"));
    assert!(!first_output.contains("link.md"));
    assert!(search(&data, "needle", &[])?.contains("knowledge/boundary.md"));
    let first = rows(&data)?;
    assert!(first
        .iter()
        .filter(|row| row.path == "knowledge/guide.md")
        .all(|row| row.content.len() <= 2_048));
    assert!(first_output
        .lines()
        .filter(|line| !line.starts_with("path="))
        .all(|line| line.len() <= 240));
    rebuild(&data)?;
    assert_eq!(first, rows(&data)?);
    assert_eq!(first_output, search(&data, "aurora", &[])?);

    fs::write(
        workspace.join("knowledge/guide.md"),
        "# Changed\n\nnebula external edit",
    )?;
    reconcile_startup(&data)?;
    assert!(search(&data, "nebula", &[])?.contains("knowledge/guide.md"));
    assert_eq!(search(&data, "aurora", &[])?, "no matches");
    fs::remove_file(workspace.join("knowledge/guide.md"))?;
    reconcile_startup(&data)?;
    assert_eq!(search(&data, "nebula", &[])?, "no matches");

    fs::write(workspace.join("knowledge/Ä.md"), "# One\n\ncase one")?;
    fs::write(workspace.join("knowledge/ä.md"), "# Two\n\ncase two")?;
    let before = rows(&data)?;
    let error = rebuild(&data)
        .err()
        .ok_or("case collision unexpectedly rebuilt")?;
    assert!(error.contains("case"));
    assert_eq!(before, rows(&data)?);
    Ok(())
}

#[test]
#[rustfmt::skip]
fn managed_external_edit_and_move_update_record_projection() -> TestResult<()> {
    let data = fixture_root("managed")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "todo",
        "Original",
        "--body",
        "old managed body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let workspace = data.join("workspace");
    let old = workspace.join(&old_path);
    let text = fs::read_to_string(&old)?
        .replace("title: Original", "title: Externally Moved")
        .replace("tags: []", "tags: [project:alpha]")
        .replace("old managed body", "comet managed external body");
    fs::create_dir_all(workspace.join("projects/alpha"))?;
    let moved = workspace.join("projects/alpha/moved.md");
    fs::write(&moved, &text)?;
    fs::remove_file(old)?;
    reconcile_startup(&data)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row = record(&conn, &id)?.ok_or("managed row missing")?;
    assert_eq!(row.path, "projects/alpha/moved.md");
    assert_eq!(row.title, "Externally Moved");
    assert_eq!(
        row.fingerprint,
        record_fingerprint(&text).map_err(|error| error.message)?
    );
    drop(conn);
    let found = search(&data, "comet", &["--project", "alpha"])?;
    assert!(found.contains("projects/alpha/moved.md"));
    let first = rows(&data)?;
    rebuild(&data)?;
    assert_eq!(first, rows(&data)?);
    assert_eq!(found, search(&data, "comet", &["--project", "alpha"])?);
    let recased = workspace.join("projects/alpha/MOVED.md");
    fs::rename(&moved, &recased)?;
    fs::write(&recased, "---\r\nkind: todo\r\ntitle: malformed")?;
    reconcile_startup(&data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let invalid = record(&conn, &id)?.ok_or("invalid tombstone missing")?;
    assert!(invalid.archived); assert_eq!(invalid.state, "import-review"); drop(conn);
    reconcile_startup(&data)?; assert_eq!(search(&data, "malformed", &[])?, "no matches");
    fs::write(&recased, &text)?; reconcile_startup(&data)?;
    assert!(search(&data, "comet", &[])?.contains("projects/alpha/MOVED.md"));
    fs::remove_file(recased)?;
    reconcile_startup(&data)?;
    assert_eq!(search(&data, "comet", &[])?, "no matches");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let missing = record(&conn, &id)?.ok_or("missing tombstone")?;
    assert!(missing.archived); assert_eq!(missing.state, "missing");
    let stale: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE key_label = 'index:stale/records' AND status = 'Active'", [], |row| row.get(0),
    )?;
    assert_eq!(stale, 1); drop(conn); rebuild(&data)?;
    let todos = fs::read_to_string(workspace.join("indexes/open-todos.md"))?;
    assert!(!todos.contains("Externally Moved"));
    Ok(())
}

fn rebuild(data: &Path) -> Result<String, String> {
    let arg = data.to_string_lossy();
    cli::run(["--data", arg.as_ref(), "workspace", "--rebuild"])
}

fn search(data: &Path, query: &str, extra: &[&str]) -> Result<String, String> {
    let arg = data.to_string_lossy();
    let mut args = vec!["--data", arg.as_ref(), "workspace", "search", query];
    args.extend_from_slice(extra);
    cli::run(args)
}

#[rustfmt::skip]
fn reconcile_startup(data: &Path) -> TestResult<()> {
    let mut endpoint = ScriptedEndpoint { outputs: Vec::new(), index: 0 };
    run_until_idle(data, &mut endpoint, 0)?; Ok(())
}

fn rows(data: &Path) -> TestResult<Vec<lkjagent_store::workspace_search::SearchChunk>> {
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    Ok(canonical_rows(&conn)?)
}

#[rustfmt::skip]
fn field(output: &str, marker: &str) -> Result<String, String> {
    output.split(marker).nth(1).and_then(|value| value.split_whitespace().next())
        .map(str::to_string).ok_or_else(|| format!("missing {marker} in {output}"))
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-inventory-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
