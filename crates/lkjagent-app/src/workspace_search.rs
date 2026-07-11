mod query;

use std::fs;
use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_record::{parse_record, record_fingerprint};
use lkjagent_store::record_rows::records;
use lkjagent_store::workspace_search::{replace_chunks, SearchChunk, SearchFilter, SearchMode};
use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub query: String,
    pub kind: Option<String>,
    pub state: Option<String>,
    pub project: Option<String>,
    pub date: Option<String>,
    pub mode: String,
}

impl Request {
    pub fn filter(&self) -> SearchFilter {
        SearchFilter {
            kind: self.kind.clone(),
            state: self.state.clone(),
            project: self.project.clone(),
            date: self.date.clone(),
        }
    }

    pub fn mode(&self) -> Result<SearchMode, String> {
        match self.mode.as_str() {
            "lexical" => Ok(SearchMode::Lexical),
            "trigram" => Ok(SearchMode::Trigram),
            _ => Err("workspace search mode must be lexical or trigram".to_string()),
        }
    }
}

pub fn rebuild(conn: &Connection, workspace: &Path) -> Result<String, String> {
    let rows = records(conn, None, false).map_err(|error| error.to_string())?;
    let mut chunks = Vec::new();
    let mut excluded = 0;
    for row in rows {
        match chunks_for_row(workspace, &row) {
            Ok(mut current) => chunks.append(&mut current),
            Err(_) => excluded += 1,
        }
    }
    chunks.sort_by(|left, right| left.id.cmp(&right.id));
    replace_chunks(conn, &chunks).map_err(|error| error.to_string())?;
    Ok(format!(
        "workspace search rebuilt: indexed={} excluded_drifted={excluded}",
        chunks.len()
    ))
}

pub fn search(conn: &Connection, workspace: &Path, request: &Request) -> Result<String, String> {
    query::run(conn, workspace, request)
}

fn chunks_for_row(
    workspace: &Path,
    row: &lkjagent_store::record_rows::RecordRow,
) -> Result<Vec<SearchChunk>, String> {
    let text = fs::read_to_string(workspace.join(&row.path)).map_err(|error| error.to_string())?;
    let fingerprint = record_fingerprint(&text).map_err(|error| error.message)?;
    let record = parse_record(&text)?;
    if fingerprint != row.fingerprint || !matches_row(&record, row) {
        return Err("workspace record drifted".to_string());
    }
    let project = record
        .tags
        .iter()
        .find_map(|tag| tag.strip_prefix("project:").map(str::to_string))
        .unwrap_or_default();
    let date = record
        .created_at
        .split('T')
        .next()
        .unwrap_or_default()
        .to_string();
    Ok(vec![
        chunk(
            row,
            "title",
            0,
            record.title.len(),
            &record.title,
            &project,
            &date,
        )?,
        chunk(
            row,
            "body",
            0,
            record.body.len(),
            &record.body,
            &project,
            &date,
        )?,
    ])
}

fn matches_row(
    record: &lkjagent_core::workspace_record::WorkspaceRecord,
    row: &lkjagent_store::record_rows::RecordRow,
) -> bool {
    record.id == row.id && record.kind == row.kind && record.state == row.state
}

fn chunk(
    row: &lkjagent_store::record_rows::RecordRow,
    field: &str,
    start_byte: usize,
    end_byte: usize,
    content: &str,
    project: &str,
    date: &str,
) -> Result<SearchChunk, String> {
    let seed = format!(
        "{}\0{}\0{field}\0{start_byte}\0{end_byte}",
        row.id, row.fingerprint
    );
    Ok(SearchChunk {
        id: stable_fingerprint(&seed).map_err(|error| error.message)?,
        document_id: row.id.clone(),
        revision_fingerprint: row.fingerprint.clone(),
        path: row.path.clone(),
        field: field.to_string(),
        start_byte,
        end_byte,
        kind: row.kind.clone(),
        state: row.state.clone(),
        project: project.to_string(),
        effective_date: date.to_string(),
        content: content.to_string(),
    })
}
