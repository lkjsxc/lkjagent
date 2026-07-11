use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_record::{parse_record, record_fingerprint};
use lkjagent_store::record_rows::{records, upsert_record, RecordRow};
use lkjagent_store::workspace_search::{replace_chunks, SearchChunk};
use rusqlite::Connection;

const CHUNK_BYTES: usize = 2_048;
const CHUNK_OVERLAP: usize = 128;
const EXCLUDED_ROOTS: &[&str] = &["archive", "indexes", "system"];

struct Document {
    id: String,
    fingerprint: String,
    path: String,
    title: String,
    body: String,
    kind: String,
    state: String,
    project: String,
    date: String,
}

type InventoryDocument = (Document, Option<RecordRow>);

#[rustfmt::skip]
pub fn rebuild(conn: &Connection, workspace: &Path) -> Result<String, String> {
    let existing = records(conn, None, true).map_err(|error| error.to_string())?.into_iter()
        .map(|row| (row.id.clone(), row)).collect::<BTreeMap<_, _>>();
    let (paths, mut excluded) = markdown_paths(workspace)?;
    let known_paths = existing.values()
        .map(|row| row.path.to_ascii_lowercase()).collect::<BTreeSet<_>>();
    let (mut chunks, mut managed, mut ids) = (Vec::new(), Vec::new(), BTreeMap::new());
    let mut invalid_paths = BTreeSet::new();
    for path in paths {
        let key = path.to_ascii_lowercase();
        let text = match read_visible(workspace, &path) {
            Ok(text) => text,
            Err(_) => { invalid_paths.insert(key); excluded += 1; continue; }
        };
        let (document, row) = match document(&path, text) {
            Ok(document) => document,
            Err(_) => { invalid_paths.insert(key); excluded += 1; continue; }
        };
        if row.is_none() && known_paths.contains(&key) {
            invalid_paths.insert(key); excluded += 1; continue;
        }
        if let Some(prior) = ids.insert(document.id.clone(), path.clone()) {
            return Err(format!("duplicate workspace document id at {prior} and {path}"));
        }
        append_chunks(&mut chunks, &document)?;
        if let Some(row) = row { managed.push(row); }
    }
    chunks.sort_by(|left, right| left.id.cmp(&right.id));
    let seen = managed.iter().map(|row| row.id.clone()).collect::<BTreeSet<_>>();
    let mut missing = 0;
    for row in existing.values().filter(|row| !row.archived && !seen.contains(&row.id)) {
        let mut stale = row.clone();
        stale.archived = true; stale.state = if invalid_paths.contains(&row.path.to_ascii_lowercase()) { "import-review" } else { "missing" }.to_string();
        stale.updated_at = crate::clock::utc_now();
        managed.push(stale); missing += 1;
    }
    let tx = conn.unchecked_transaction().map_err(|error| error.to_string())?;
    let mut synced = 0;
    for row in &managed {
        if existing.get(&row.id) != Some(row) {
            upsert_record(&tx, row).map_err(|error| error.to_string())?; synced += 1;
        }
    }
    replace_chunks(&tx, &chunks).map_err(|error| error.to_string())?;
    if synced > 0 { super::mark_navigation_stale(&tx, &crate::clock::utc_now())?; }
    tx.commit().map_err(|error| error.to_string())?;
    Ok(format!("workspace search rebuilt: indexed={} documents={} synced={synced} missing={missing} excluded={excluded}", chunks.len(), ids.len()))
}

#[rustfmt::skip]
fn markdown_paths(workspace: &Path) -> Result<(Vec<String>, usize), String> {
    let mut pending = vec![workspace.to_path_buf()];
    let (mut paths, mut excluded) = (Vec::new(), 0);
    while let Some(dir) = pending.pop() {
        let mut entries = fs::read_dir(&dir).map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let full = entry.path();
            let relative = full.strip_prefix(workspace).map_err(|error| error.to_string())?;
            if !visible(relative) { continue; }
            let metadata = fs::symlink_metadata(&full).map_err(|error| error.to_string())?;
            if metadata.file_type().is_symlink() { excluded += 1; continue; }
            if metadata.is_dir() { pending.push(full); continue; }
            if metadata.is_file() && extension(relative) {
                let path = relative.to_str().ok_or_else(|| "workspace path is not UTF-8".to_string())?;
                paths.push(path.replace('\\', "/"));
            }
        }
    }
    paths.sort();
    let mut folded = BTreeSet::new();
    for path in &paths {
        let key = collision_key(path)?;
        if !folded.insert(key) { return Err(format!("workspace path case collision: {path}")); }
    }
    Ok((paths, excluded))
}

pub(super) fn read_visible(workspace: &Path, path: &str) -> Result<String, String> {
    let relative = Path::new(path);
    if !visible(relative) || relative.is_absolute() {
        return Err("workspace path is not visible".to_string());
    }
    crate::effect_files::read_text(workspace, path)
}

#[rustfmt::skip]
fn visible(path: &Path) -> bool {
    let mut names = Vec::new();
    for part in path.components() {
        let Component::Normal(name) = part else { return false; };
        let Some(name) = name.to_str() else { return false; };
        if name.starts_with('.') || name.chars().any(char::is_control) { return false; }
        names.push(name);
    }
    !names.is_empty() && !EXCLUDED_ROOTS.iter().any(|root| names[0].eq_ignore_ascii_case(root))
}

fn collision_key(path: &str) -> Result<String, String> {
    if path
        .chars()
        .any(|ch| !ch.is_ascii() && (ch.is_lowercase() || ch.is_uppercase()))
    {
        return Err(format!(
            "workspace path has non-ASCII case ambiguity: {path}"
        ));
    }
    Ok(path.to_ascii_lowercase())
}

#[rustfmt::skip]
fn extension(path: &Path) -> bool {
    path.extension().and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("md"))
}

#[rustfmt::skip]
fn document(path: &str, text: String) -> Result<InventoryDocument, String> {
    let fingerprint = record_fingerprint(&text).map_err(|error| error.message)?;
    if managed_candidate(&text) {
        let record = parse_record(&text)?;
        let project = record.tags.iter().find_map(|tag| tag.strip_prefix("project:")).unwrap_or("").to_string();
        let date = record.created_at.split('T').next().unwrap_or("").to_string();
        let row = RecordRow { id: record.id.clone(), kind: record.kind.clone(), title: record.title.clone(),
            state: record.state.clone(), path: path.to_string(), fingerprint: fingerprint.clone(),
            archived: false, updated_at: record.updated_at.clone() };
        return Ok((Document { id: record.id, fingerprint, path: path.to_string(), title: record.title,
            body: record.body, kind: record.kind, state: record.state, project, date }, Some(row)));
    }
    let title = text.lines().find_map(|line| line.strip_prefix("# ")).unwrap_or_else(||
        Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or(path)).to_string();
    let project = Path::new(path).components().collect::<Vec<_>>().windows(2).find_map(|parts| {
        (parts[0].as_os_str() == "projects").then(|| parts[1].as_os_str().to_string_lossy().to_string())
    }).unwrap_or_default();
    let id = format!("file-{}", stable_fingerprint(&path).map_err(|error| error.message)?);
    Ok((Document { id, fingerprint, path: path.to_string(), title, body: text,
        kind: "markdown".to_string(), state: "current".to_string(), project, date: String::new() }, None))
}

#[rustfmt::skip]
fn managed_candidate(text: &str) -> bool {
    let Some(rest) = text.strip_prefix("---\n") else { return false; };
    rest.split("\n---\n").next().unwrap_or(rest).lines().any(|line| line.starts_with("id: "))
}

#[rustfmt::skip]
fn append_chunks(output: &mut Vec<SearchChunk>, document: &Document) -> Result<(), String> {
    for (field, content) in [("title", document.title.as_str()), ("body", document.body.as_str())] {
        let mut start = 0;
        while start < content.len() {
            let mut end = (start + CHUNK_BYTES).min(content.len());
            while end > start && !content.is_char_boundary(end) { end -= 1; }
            let seed = format!("{}\0{}\0{field}\0{start}\0{end}", document.id, document.fingerprint);
            output.push(SearchChunk { id: stable_fingerprint(&seed).map_err(|error| error.message)?,
                document_id: document.id.clone(), revision_fingerprint: document.fingerprint.clone(),
                path: document.path.clone(), field: field.to_string(), start_byte: start, end_byte: end,
                kind: document.kind.clone(), state: document.state.clone(), project: document.project.clone(),
                effective_date: document.date.clone(), content: content[start..end].to_string() });
            if end == content.len() { break; }
            start = super::floor_boundary(content, end.saturating_sub(CHUNK_OVERLAP));
        }
    }
    Ok(())
}
