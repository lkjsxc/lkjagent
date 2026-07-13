use rusqlite::{Connection, OptionalExtension};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::evaluation_harness::sha256;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ManifestRow {
    pub bytes: u64,
    pub sha256: String,
}

pub fn manifest_rows(text: &str) -> BTreeMap<String, ManifestRow> {
    text.lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some((
                fields.next()?.to_string(),
                ManifestRow {
                    bytes: fields.next()?.parse().ok()?,
                    sha256: fields.next()?.to_string(),
                },
            ))
        })
        .collect()
}

pub fn changed_paths(before: &str, after: &str) -> Vec<String> {
    let left = manifest_rows(before);
    let right = manifest_rows(after);
    left.keys()
        .chain(right.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|path| left.get(*path) != right.get(*path))
        .map(|path| path.to_string())
        .collect()
}

pub fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative)).map_err(|error| error.to_string())
}

pub fn count(connection: &Connection, sql: &str) -> Result<u64, String> {
    connection
        .query_row(sql, [], |row| row.get::<_, i64>(0))
        .map(|value| value.max(0) as u64)
        .map_err(|error| error.to_string())
}

pub fn text(connection: &Connection, sql: &str) -> Result<Option<String>, String> {
    connection
        .query_row(sql, [], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|error| error.to_string())
}

pub fn scalar_with(connection: &Connection, sql: &str, arg: &str) -> Result<u64, String> {
    connection
        .query_row(sql, [arg], |row| row.get::<_, i64>(0))
        .map(|value| value.max(0) as u64)
        .map_err(|error| error.to_string())
}

pub fn text_with(connection: &Connection, sql: &str, arg: &str) -> Result<Option<String>, String> {
    connection
        .query_row(sql, [arg], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|error| error.to_string())
}

pub fn revision_for_path(connection: &Connection, path: &str) -> Result<Option<String>, String> {
    text_with(
        connection,
        "SELECT CAST(current_revision_id AS TEXT) FROM workspace_documents WHERE CAST(current_path AS TEXT)=?1 AND status='active'",
        path,
    )
}

pub fn parent_revision(connection: &Connection, revision: &str) -> Result<Option<String>, String> {
    text_with(
        connection,
        "SELECT CAST(parent_id AS TEXT) FROM workspace_revisions WHERE id=?1",
        revision,
    )
}

pub fn token_units(text: &str) -> u64 {
    text.len().div_ceil(4).max(text.chars().count()) as u64
}

pub fn word_count(text: &str) -> u64 {
    text.split(|char: char| !char.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .count() as u64
}

pub fn placeholder_count(text: &str) -> u64 {
    [
        "todo",
        "tbd",
        "placeholder",
        "lorem ipsum",
        "generated content",
        "...",
    ]
    .into_iter()
    .filter(|needle| text.to_ascii_lowercase().contains(needle))
    .count() as u64
}

pub fn fingerprint(paths: &[String]) -> String {
    sha256(paths.join("\n").as_bytes())
}

pub fn cast_path(raw: &Path) -> PathBuf {
    raw.join("terminal.cast")
}
