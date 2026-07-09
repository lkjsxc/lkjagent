use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use rusqlite::Connection;

use super::hash;

static NEXT_CAPTURE: AtomicU64 = AtomicU64::new(1);

pub struct Capture {
    pub root: PathBuf,
}

impl Drop for Capture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

pub fn create(repo: &Path) -> Result<Capture, String> {
    let serial = NEXT_CAPTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "lkjagent-evaluation-capture-{}-{serial}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root.join("workspace/project/src")).map_err(|error| error.to_string())?;
    fs::write(
        root.join("workspace/note.md"),
        "# Verified Note\n\nraw bytes\n",
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("workspace/project/src/lib.rs"),
        "pub fn captured() -> bool { true }\n",
    )
    .map_err(|error| error.to_string())?;
    let source_path = root.join("source.sqlite3");
    let backup_path = root.join("run.sqlite3");
    let source = Connection::open(&source_path).map_err(|error| error.to_string())?;
    source
        .execute_batch(
            "PRAGMA journal_mode=WAL;
             CREATE TABLE raw_events(sequence INTEGER PRIMARY KEY, kind TEXT NOT NULL);
             INSERT INTO raw_events VALUES(1, 'session.start');
             INSERT INTO raw_events VALUES(2, 'owner.turn');",
        )
        .map_err(|error| error.to_string())?;
    let output = Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(repo.join("evaluation/sqlite-online-backup.py"))
        .arg(&source_path)
        .arg(&backup_path)
        .output()
        .map_err(|error| format!("run SQLite backup helper: {error}"))?;
    let log = String::from_utf8_lossy(&output.stdout).to_string();
    fs::write(root.join("snapshot.log"), &log).map_err(|error| error.to_string())?;
    if !output.status.success()
        || !log.contains("snapshot_method\tsqlite-online-backup")
        || !log.contains("integrity\tok")
    {
        return Err(format!(
            "SQLite Online Backup failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    source
        .execute("INSERT INTO raw_events VALUES(3, 'post.boundary')", [])
        .map_err(|error| error.to_string())?;
    let backup = Connection::open(&backup_path).map_err(|error| error.to_string())?;
    let backup_count: i64 = backup
        .query_row("SELECT COUNT(*) FROM raw_events", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    let source_count: i64 = source
        .query_row("SELECT COUNT(*) FROM raw_events", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    let integrity: String = backup
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if backup_count != 2 || source_count != 3 || integrity != "ok" {
        return Err("SQLite backup did not preserve the quiesced boundary".into());
    }
    write_workspace_manifest(&root)?;
    Ok(Capture { root })
}

fn write_workspace_manifest(root: &Path) -> Result<(), String> {
    let workspace = root.join("workspace");
    let mut files = Vec::new();
    collect(&workspace, &mut files)?;
    files.sort();
    let mut body = String::from("path\tdocument_id\trevision_id\tsha256\n");
    for path in files {
        let relative = path
            .strip_prefix(&workspace)
            .map_err(|error| error.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let fingerprint = hash::bytes(&fs::read(&path).map_err(|error| error.to_string())?);
        body.push_str(&format!(
            "{relative}\tdoc-{}\trev-1\t{fingerprint}\n",
            &fingerprint[7..19]
        ));
    }
    fs::write(root.join("workspace-manifest.tsv"), body).map_err(|error| error.to_string())
}

fn collect(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_symlink() {
            return Err(format!(
                "workspace manifest refuses symlink: {}",
                entry.path().display()
            ));
        }
        if kind.is_dir() {
            collect(&entry.path(), files)?;
        } else if kind.is_file() {
            files.push(entry.path());
        }
    }
    Ok(())
}

pub fn write_raw_manifest(capture: &Capture, scenario: &str) -> Result<usize, String> {
    let names = [
        "run.sqlite3",
        "snapshot.log",
        "workspace-manifest.tsv",
        "terminal.cast",
        "terminal-replay.tsv",
        "pty-recorder.log",
    ];
    let mut body = format!("artifact\tsha256\nscenario-bundle\t{scenario}\n");
    for name in names {
        let bytes = fs::read(capture.root.join(name)).map_err(|error| error.to_string())?;
        if bytes.is_empty() {
            return Err(format!("raw artifact is empty: {name}"));
        }
        body.push_str(&format!("{name}\t{}\n", hash::bytes(&bytes)));
    }
    fs::write(capture.root.join("raw-manifest.tsv"), body).map_err(|error| error.to_string())?;
    Ok(names.len() + 1)
}

pub fn validate_raw_manifest(capture: &Capture, scenario: &str) -> Result<(), String> {
    let text = fs::read_to_string(capture.root.join("raw-manifest.tsv"))
        .map_err(|error| error.to_string())?;
    let mut rows = 0;
    for line in text.lines().skip(1) {
        rows += 1;
        let (name, expected) = line
            .split_once('\t')
            .ok_or_else(|| "raw manifest row is malformed".to_string())?;
        let found = if name == "scenario-bundle" {
            scenario.to_string()
        } else {
            hash::bytes(&fs::read(capture.root.join(name)).map_err(|error| error.to_string())?)
        };
        if expected != found {
            return Err(format!("raw manifest fingerprint differs: {name}"));
        }
    }
    if rows < 7 {
        return Err("raw manifest has fewer than seven bound artifacts".into());
    }
    Ok(())
}
