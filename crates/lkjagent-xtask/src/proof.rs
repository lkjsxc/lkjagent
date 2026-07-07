use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

pub fn run(args: &[String], root: &Path) -> i32 {
    match parse(args, root).and_then(collect) {
        Ok(path) => {
            println!("ok proof collect artifact={}", path.display());
            0
        }
        Err(error) => fail("proof collect", &error),
    }
}

fn parse(args: &[String], root: &Path) -> Result<Options, String> {
    let mut data_dir = root.join("data");
    let mut out_dir = root.join("tmp/proof-current");
    let mut index = 0;
    if args.first().is_some_and(|arg| arg == "collect") {
        index = 1;
    }
    while index < args.len() {
        match args[index].as_str() {
            "--data" => {
                data_dir = path_arg(args, index + 1, root, "--data")?;
                index += 2;
            }
            "--out" => {
                out_dir = path_arg(args, index + 1, root, "--out")?;
                index += 2;
            }
            other => return Err(format!("unknown proof argument: {other}")),
        }
    }
    Ok(Options { data_dir, out_dir })
}

fn collect(options: Options) -> Result<PathBuf, String> {
    fs::create_dir_all(&options.out_dir).map_err(|e| e.to_string())?;
    let conn =
        Connection::open(options.data_dir.join("lkjagent.sqlite3")).map_err(|e| e.to_string())?;
    write(&options.out_dir, "summary.md", &summary(&conn)?)?;
    write(&options.out_dir, "status.md", &status(&conn)?)?;
    crate::proof_state::write_state_bundle(&conn, &options.out_dir)?;
    crate::proof_records::write_record_selector_bundle(&conn, &options.out_dir)?;
    crate::proof_checks::write_checks(&conn, &options.out_dir)?;
    crate::proof_tokens::write_attempts_and_tokens(&conn, &options.out_dir)?;
    write(
        &options.out_dir,
        "workspace-tree.md",
        &tree(&options.data_dir.join("workspace"))?,
    )?;
    write(
        &options.out_dir,
        "word-counts.md",
        &word_counts(&options.data_dir.join("workspace"))?,
    )?;
    write(&options.out_dir, "warnings.md", "# Warnings\n\nnone\n")?;
    Ok(options.out_dir.join("summary.md"))
}

fn summary(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "# Proof Summary\n\nmatters={}\noperations={}\nchecks={}\n",
        count(conn, "tasks")?,
        count(conn, "steps")?,
        count(conn, "check_results")?
    ))
}

fn status(conn: &Connection) -> Result<String, String> {
    let mut statement = conn
        .prepare("SELECT id, state, template, budget_used, budget FROM tasks ORDER BY id")
        .map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([], |row| {
            Ok(format!(
                "- matter={} state={} template={} budget={}/{}",
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?
            ))
        })
        .map_err(|e| e.to_string())?;
    let mut lines = vec!["# Status".to_string(), String::new()];
    for row in rows {
        lines.push(row.map_err(|e| e.to_string())?);
    }
    Ok(lines.join("\n"))
}

fn count(conn: &Connection, table: &str) -> Result<i64, String> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .map_err(|e| e.to_string())
}

fn tree(root: &Path) -> Result<String, String> {
    let mut lines = vec!["# Workspace Tree".to_string(), String::new()];
    walk(root, root, &mut lines)?;
    Ok(lines.join("\n"))
}

fn word_counts(root: &Path) -> Result<String, String> {
    let mut lines = vec!["# Word Counts".to_string(), String::new()];
    for path in markdown_files(root)? {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        lines.push(format!(
            "- {} words={}",
            rel(root, &path),
            text.split_whitespace().count()
        ));
    }
    Ok(lines.join("\n"))
}

fn markdown_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_md(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_md(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
        files.push(path.to_path_buf());
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
        {
            collect_md(&entry.path(), files)?;
        }
    }
    Ok(())
}

fn walk(root: &Path, path: &Path, lines: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        lines.push(format!("- file {}", rel(root, path)));
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
        {
            walk(root, &entry.path(), lines)?;
        }
    }
    Ok(())
}

fn path_arg(args: &[String], index: usize, root: &Path, flag: &str) -> Result<PathBuf, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("{flag} needs a path"))?;
    let path = PathBuf::from(value);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn write(dir: &Path, name: &str, body: &str) -> Result<(), String> {
    fs::write(dir.join(name), body).map_err(|e| e.to_string())
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

fn fail(name: &str, message: &str) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}

struct Options {
    data_dir: PathBuf,
    out_dir: PathBuf,
}
