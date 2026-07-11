use lkjagent_core::workspace_record::{
    default_state_for_kind, parse_record, record_fingerprint, record_path_at, render_record,
    state_keys_for_record, WorkspaceRecord,
};
use lkjagent_store::record_rows::{record, records, upsert_record, RecordRow};
use rusqlite::Connection;
use std::{fs, path::Path};
pub fn add(
    conn: &Connection,
    data_dir: &Path,
    kind: &str,
    title: &str,
    body: &str,
    now: &str,
) -> Result<String, String> {
    let kind = crate::record_identity::normalized_kind(kind);
    let id = crate::record_identity::record_id(kind, now, title);
    let mut record = WorkspaceRecord::new(&id, kind, title, now);
    record.state = default_state_for_kind(kind).to_string();
    record.state_keys = state_keys_for_record(kind, &id, &record.state);
    record.body = body.to_string();
    write_rec(conn, data_dir, &record)
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
    let row = record_or_alias(conn, id)?;
    let text = fs::read_to_string(crate::config::workspace_root(data_dir)?.join(&row.path))
        .map_err(|error| error.to_string())?;
    Ok(format!("{}\n{}", format_record_row("record", &row), text))
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
    let path = crate::config::workspace_root(data_dir)?.join(&row.path);
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let mut parsed = parse_record(&text)?;
    if !parsed.links.iter().any(|link| link == target) {
        parsed.links.push(target.to_string());
    }
    parsed.updated_at = now.to_string();
    let output = render_record(&parsed);
    fs::write(&path, &output).map_err(|error| error.to_string())?;
    let updated = record_row(
        (&row.id, &row.kind, &row.title, &row.state),
        &row.path,
        &output,
        false,
        now,
    )?;
    upsert_record(conn, &updated).map_err(|error| error.to_string())?;
    crate::record_state::upsert_record_cells(conn, &parsed, &row.path, &updated.fingerprint)?;
    Ok(format!("linked record: {id} -> {target}"))
}

fn record_or_alias(conn: &Connection, id: &str) -> Result<RecordRow, String> {
    if let Some(row) = record(conn, id).map_err(|error| error.to_string())? {
        return Ok(row);
    }
    let alias = lkjagent_store::workspace_rows::resolve_alias(conn, id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {id}"))?;
    record(conn, &alias.entity_id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found for alias: {id}"))
}

fn write_rec(
    conn: &Connection,
    data_dir: &Path,
    record: &WorkspaceRecord,
) -> Result<String, String> {
    let rel = record_path_at(
        &record.kind,
        &record.id,
        &record.updated_at,
        &record.title,
        &record.state,
    )?;
    let workspace = crate::config::workspace_root(data_dir)?;
    crate::workspace_scaffold::ensure_for_path(&workspace, &rel)?;
    let path = workspace.join(&rel);
    let prepared = crate::record_identity::prepare_record_text(&rel, record);
    for part in &prepared.parts {
        crate::workspace_scaffold::ensure_for_path(&workspace, &part.rel)?;
        fs::write(workspace.join(&part.rel), &part.text).map_err(|error| error.to_string())?;
        crate::workspace_scaffold::refresh_for_path(&workspace, &part.rel)?;
    }
    let text = prepared.main_text;
    fs::write(&path, &text).map_err(|error| error.to_string())?;
    crate::workspace_scaffold::refresh_for_path(&workspace, &rel)?;
    let row = record_row(
        (&record.id, &record.kind, &record.title, &record.state),
        &rel,
        &text,
        false,
        &record.updated_at,
    )?;
    upsert_record(conn, &row).map_err(|error| error.to_string())?;
    crate::record_state::upsert_record_cells(conn, record, &rel, &row.fingerprint)?;
    let index = crate::workspace_index::rebuild(conn, data_dir, &record.updated_at)?;
    Ok(format!(
        "record: {} path={rel} fp={} index={index}",
        record.id, row.fingerprint
    ))
}

fn record_row(
    fields: (&str, &str, &str, &str),
    path: &str,
    text: &str,
    archived: bool,
    updated_at: &str,
) -> Result<RecordRow, String> {
    let (id, kind, title, state) = fields;
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

pub(crate) fn archive_preimage(row: &RecordRow) -> String {
    serde_json::json!({
        "id": row.id,
        "path": row.path,
        "fingerprint": row.fingerprint,
        "state": row.state,
        "archived": row.archived,
    })
    .to_string()
}

pub(crate) fn archive_intended(row: &RecordRow, path: &str) -> String {
    serde_json::json!({"id": row.id, "path": path, "state": "archived"}).to_string()
}

fn format_record_row(prefix: &str, row: &RecordRow) -> String {
    format!(
        "{prefix} {} kind={} state={} title={} path={} fp={}",
        row.id, row.kind, row.state, row.title, row.path, row.fingerprint
    )
}
