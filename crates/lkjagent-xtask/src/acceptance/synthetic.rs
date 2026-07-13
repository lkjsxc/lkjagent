use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::Command;

const DB: &str = "evaluation/synthetic/native-empty.sqlite3.gz";
const MANIFEST: &str = "evaluation/synthetic/manifest.tsv";

pub fn valid(root: &Path) -> bool {
    if !tracked(root, DB) || !tracked(root, MANIFEST) {
        return false;
    }
    let compressed = fs::read(root.join(DB)).unwrap_or_default();
    let manifest = fs::read_to_string(root.join(MANIFEST)).unwrap_or_default();
    let fields = manifest
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once('\t'))
        .collect::<std::collections::BTreeMap<_, _>>();
    let schema =
        fs::read(root.join("crates/lkjagent-store/src/native-schema.sql")).unwrap_or_default();
    let compressed_hash = hash(&compressed);
    let schema_hash = hash(&schema);
    if fields.get("fixture_kind") != Some(&"synthetic-native-empty")
        || fields.get("table_count") != Some(&"18")
        || fields.get("compressed_sha256") != Some(&compressed_hash.as_str())
        || fields.get("schema_sha256") != Some(&schema_hash.as_str())
        || super::secret::kind(&compressed).is_some()
    {
        return false;
    }
    let Ok(output) = Command::new("gzip")
        .args(["-cd", DB])
        .current_dir(root)
        .output()
    else {
        return false;
    };
    let database_hash = hash(&output.stdout);
    if !output.status.success()
        || output.stdout.len() > 2_000_000
        || fields.get("database_sha256") != Some(&database_hash.as_str())
        || super::secret::kind(&output.stdout).is_some()
    {
        return false;
    }
    inspect_database(&output.stdout)
}

fn inspect_database(bytes: &[u8]) -> bool {
    let path =
        std::env::temp_dir().join(format!("lkjagent-synthetic-{}.sqlite3", std::process::id()));
    let result = (|| {
        let mut file = OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(&path).ok()?;
        file.write_all(bytes).ok()?;
        file.sync_all().ok()?;
        drop(file);
        let db = Connection::open(&path).ok()?;
        let count: i64 = db.query_row("SELECT count(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'", [], |row| row.get(0)).ok()?;
        let integrity: String = db.query_row("PRAGMA integrity_check", [], |row| row.get(0)).ok()?;
        Some(count == 18 && integrity == "ok")
    })().unwrap_or(false);
    let _ = fs::remove_file(path);
    result
}

fn tracked(root: &Path, path: &str) -> bool {
    Command::new("git")
        .args(["ls-files", "--error-unmatch", "--", path])
        .current_dir(root)
        .output()
        .is_ok_and(|output| output.status.success())
}
fn hash(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
