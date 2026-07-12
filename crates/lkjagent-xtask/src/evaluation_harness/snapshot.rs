use super::hash;
use rusqlite::Connection;
use std::collections::BTreeMap;
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
static SERIAL: AtomicU64 = AtomicU64::new(1);

pub struct Capture {
    pub root: PathBuf,
    pub data: PathBuf,
    pub workspace: PathBuf,
    pub raw: PathBuf,
    pub binary: PathBuf,
}
impl Drop for Capture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
pub fn create() -> Result<Capture, String> {
    let root = std::env::temp_dir().join(format!(
        "lkjagent-evaluation-{}-{}",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ));
    if root.exists() {
        return Err("fresh capture root already exists".into());
    }
    let data = root.join("data");
    let workspace = root.join("workspace");
    let raw = root.join("raw");
    let bin = root.join("bin");
    for path in [&root, &data, &workspace, &raw, &bin] {
        fs::create_dir(path).map_err(|error| error.to_string())?;
        fs::set_permissions(path, Permissions::from_mode(0o700))
            .map_err(|error| error.to_string())?;
    }
    Ok(Capture {
        root,
        data,
        workspace,
        raw,
        binary: bin.join("lkjagent"),
    })
}
pub fn copy_seed(source: &Path, target: &Path) -> Result<(), String> {
    for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_symlink() {
            return Err("scenario seed contains a symlink".into());
        }
        let destination = target.join(entry.file_name());
        if kind.is_dir() {
            fs::create_dir(&destination).map_err(|error| error.to_string())?;
            copy_seed(&entry.path(), &destination)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), destination).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}
pub fn manifest(root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect(root, root, &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));
    let mut body = String::from("path\tbytes\tsha256\n");
    for (path, bytes) in files {
        body.push_str(&format!(
            "{}\t{}\t{}\n",
            path,
            bytes.len(),
            hash::bytes(&bytes)
        ));
    }
    Ok(body)
}
pub fn diff(before: &str, after: &str) -> String {
    let prior = rows(before);
    let next = rows(after);
    let mut body = String::from("path\tchange\tbefore_sha256\tafter_sha256\n");
    for path in prior
        .keys()
        .chain(next.keys())
        .collect::<std::collections::BTreeSet<_>>()
    {
        let left = prior.get(*path);
        let right = next.get(*path);
        if left != right {
            let change = match (left, right) {
                (None, _) => "added",
                (_, None) => "removed",
                _ => "changed",
            };
            body.push_str(&format!(
                "{path}\t{change}\t{}\t{}\n",
                left.copied().unwrap_or("absent"),
                right.copied().unwrap_or("absent")
            ));
        }
    }
    body
}
fn rows(text: &str) -> BTreeMap<&str, &str> {
    text.lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some((fields.next()?, fields.nth(1)?))
        })
        .collect()
}
fn collect(base: &Path, path: &Path, files: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_symlink() {
            return Err(format!(
                "manifest refuses symlink: {}",
                entry.path().display()
            ));
        }
        if kind.is_dir() {
            collect(base, &entry.path(), files)?;
        } else if kind.is_file() {
            let relative = entry
                .path()
                .strip_prefix(base)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            files.push((relative, fs::read(entry.path()).map_err(|e| e.to_string())?));
        }
    }
    Ok(())
}
pub fn sqlite_facts(repo: &Path, database: &Path, backup: &Path) -> Result<String, String> {
    if !database.is_file() {
        return Err("public runtime did not create SQLite state".into());
    }
    let output = Command::new("python3")
        .env_clear()
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(repo.join("evaluation/sqlite-online-backup.py"))
        .arg(database)
        .arg(backup)
        .output()
        .map_err(|error| format!("start SQLite Online Backup: {error}"))?;
    if !output.status.success()
        || output.stdout.len() > 16_384
        || output.stderr.len() > 16_384
        || !String::from_utf8_lossy(&output.stdout)
            .contains("snapshot_method\tsqlite-online-backup")
    {
        return Err("SQLite Online Backup failed".into());
    }
    let stable = Connection::open(backup).map_err(|error| error.to_string())?;
    let integrity: String = stable
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if integrity != "ok" {
        return Err("SQLite backup integrity check failed".into());
    }
    let mut names = stable.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .map_err(|error| error.to_string())?;
    let tables = names
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let mut body = String::from("table\trow_count\n");
    for table in tables {
        if !table
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err("unsafe SQLite table name".into());
        }
        let count: i64 = stable
            .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .map_err(|error| error.to_string())?;
        body.push_str(&format!("{table}\t{count}\n"));
    }
    Ok(body)
}
