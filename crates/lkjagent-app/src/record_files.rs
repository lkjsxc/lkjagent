use std::fs;
use std::path::Path;

use lkjagent_core::workspace_record::{
    archive_path, parse_record, record_fingerprint, record_path, render_record, slug,
    WorkspaceRecord,
};
use lkjagent_store::record_rows::{record, records, upsert_record, RecordRow};
use rusqlite::Connection;

pub fn add(
    conn: &Connection,
    data_dir: &Path,
    kind: &str,
    title: &str,
    body: &str,
    now: &str,
) -> Result<String, String> {
    ensure_dirs(data_dir)?;
    let id = record_id(now, title);
    let mut record = WorkspaceRecord::new(&id, kind, title, now);
    record.body = body.to_string();
    write_record(conn, data_dir, &record, false)
}

pub fn list(conn: &Connection, kind: Option<&str>) -> Result<String, String> {
    let rows = records(conn, kind, false).map_err(|error| error.to_string())?;
    if rows.is_empty() {
        return Ok("records: none".to_string());
    }
    Ok(rows
        .iter()
        .map(|row| format_record_row("record", row))
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn show(conn: &Connection, data_dir: &Path, id: &str) -> Result<String, String> {
    let row = record(conn, id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {id}"))?;
    let text = fs::read_to_string(data_dir.join("workspace").join(&row.path))
        .map_err(|error| error.to_string())?;
    Ok(format!("{}\n{}", format_record_row("record", &row), text))
}

pub fn archive(conn: &Connection, data_dir: &Path, id: &str, now: &str) -> Result<String, String> {
    let row = record(conn, id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {id}"))?;
    let workspace = data_dir.join("workspace");
    let old = workspace.join(&row.path);
    let new_rel = archive_path(&row.kind, &row.id)?;
    let new = workspace.join(&new_rel);
    if let Some(parent) = new.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(old, &new).map_err(|error| error.to_string())?;
    let text = fs::read_to_string(&new).map_err(|error| error.to_string())?;
    upsert_record(
        conn,
        &record_row(
            &row.id, &row.kind, &row.title, "archived", &new_rel, &text, true, now,
        )?,
    )
    .map_err(|error| error.to_string())?;
    Ok(format!("archived record: {id}"))
}

pub fn link(
    conn: &Connection,
    data_dir: &Path,
    id: &str,
    target: &str,
    now: &str,
) -> Result<String, String> {
    let row = record(conn, id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {id}"))?;
    let path = data_dir.join("workspace").join(&row.path);
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let mut parsed = parse_record(&text)?;
    if !parsed.links.iter().any(|link| link == target) {
        parsed.links.push(target.to_string());
    }
    parsed.updated_at = now.to_string();
    let output = render_record(&parsed);
    fs::write(&path, &output).map_err(|error| error.to_string())?;
    upsert_record(
        conn,
        &record_row(
            &row.id, &row.kind, &row.title, &row.state, &row.path, &output, false, now,
        )?,
    )
    .map_err(|error| error.to_string())?;
    Ok(format!("linked record: {id} -> {target}"))
}

fn write_record(
    conn: &Connection,
    data_dir: &Path,
    record: &WorkspaceRecord,
    archived: bool,
) -> Result<String, String> {
    let rel = record_path(&record.kind, &record.id)?;
    let path = data_dir.join("workspace").join(&rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let text = render_record(record);
    fs::write(&path, &text).map_err(|error| error.to_string())?;
    let row = record_row(
        &record.id,
        &record.kind,
        &record.title,
        &record.state,
        &rel,
        &text,
        archived,
        &record.updated_at,
    )?;
    upsert_record(conn, &row).map_err(|error| error.to_string())?;
    Ok(format!("record: {} path={rel}", record.id))
}

fn record_row(
    id: &str,
    kind: &str,
    title: &str,
    state: &str,
    path: &str,
    text: &str,
    archived: bool,
    updated_at: &str,
) -> Result<RecordRow, String> {
    Ok(RecordRow {
        id: id.to_string(),
        kind: kind.to_string(),
        title: title.to_string(),
        state: state.to_string(),
        path: path.to_string(),
        fingerprint: record_fingerprint(text).map_err(|error| error.message)?,
        archived,
        updated_at: updated_at.to_string(),
    })
}

fn record_id(now: &str, title: &str) -> String {
    let stamp = now
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>();
    let suffix = slug(title);
    format!("rec_{}_{}", stamp, suffix)
}

fn ensure_dirs(data_dir: &Path) -> Result<(), String> {
    let workspace = data_dir.join("workspace");
    fs::create_dir_all(workspace.join("records")).map_err(|error| error.to_string())?;
    write_if_missing(
        &workspace.join("README.md"),
        "# Workspace\n\nOwner-readable files.\n",
    )?;
    write_if_missing(
        &workspace.join("records/README.md"),
        "# Records\n\nGeneric owner-readable records.\n",
    )
}

fn write_if_missing(path: &Path, body: &str) -> Result<(), String> {
    if !path.exists() {
        fs::write(path, body).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn format_record_row(prefix: &str, row: &RecordRow) -> String {
    format!(
        "{prefix} {} kind={} state={} title={} path={} fp={}",
        row.id, row.kind, row.state, row.title, row.path, row.fingerprint
    )
}
