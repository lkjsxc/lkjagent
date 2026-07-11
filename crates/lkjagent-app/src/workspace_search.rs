pub(crate) mod inventory;

use std::collections::BTreeMap;
use std::path::Path;

use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey, StateStatus};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use lkjagent_store::state_rows::insert_case;
use lkjagent_store::workspace_search::{
    search as store_search, SearchFilter, SearchHit, SearchMode,
};
use rusqlite::Connection;

const EXCERPT_BYTES: usize = 240;
const EXCERPT_CONTENT_BYTES: usize = EXCERPT_BYTES - 6;

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
    #[rustfmt::skip]
    pub fn filter(&self) -> SearchFilter {
        SearchFilter { kind: self.kind.clone(), state: self.state.clone(), project: self.project.clone(), date: self.date.clone() }
    }

    #[rustfmt::skip]
    pub fn mode(&self) -> Result<SearchMode, String> {
        match self.mode.as_str() { "lexical" => Ok(SearchMode::Lexical), "trigram" => Ok(SearchMode::Trigram),
            _ => Err("workspace search mode must be lexical or trigram".to_string()) }
    }
}

pub fn rebuild(conn: &Connection, workspace: &Path) -> Result<String, String> {
    crate::workspace_scan::rebuild(conn, workspace)
}

pub fn reconcile_entry(
    conn: &Connection,
    workspace: &Path,
    data_dir: &Path,
) -> Result<String, String> {
    crate::workspace_scan::reconcile_entry(conn, workspace, data_dir)
}

#[rustfmt::skip]
fn mark_navigation_stale(conn: &Connection, now: &str) -> Result<(), String> {
    let case_id = "workspace"; insert_case(conn, case_id, "workspace records", now).map_err(|error| error.to_string())?;
    let event_id = next_event_id(conn, case_id, "index-stale").map_err(|error| error.to_string())?;
    let mut cell = StateCell::active(StateKey::new("index", "stale/records").map_err(|error| error.message)?, event_id.clone());
    cell.payload_schema = "workspace.index-stale".to_string();
    cell.payload_json = serde_json::json!({"reason":"external managed source change"}).to_string();
    cell.created_at = now.to_string(); cell.updated_at = now.to_string();
    append_and_apply_event(conn, &RuntimeEvent { id: event_id, case_id: case_id.to_string(),
        kind: "state.cell.upsert".to_string(), payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "workspace-scanner".to_string(), created_at: now.to_string(), decision_id: None })
        .map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn sync_diagnostics(conn: &Connection, diagnostics: &[(String, String)], now: &str) -> Result<(), String> {
    insert_case(conn, "workspace", "workspace records", now).map_err(|error| error.to_string())?;
    let mut statement = conn.prepare("SELECT key_label, cell_json FROM state_cells
        WHERE case_id = 'workspace' AND key_label LIKE 'workspace:diagnostic/%'").map_err(|error| error.to_string())?;
    let rows = statement.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))).map_err(|error| error.to_string())?;
    let mut existing = BTreeMap::new();
    for row in rows { let (label, json) = row.map_err(|error| error.to_string())?;
        existing.insert(label, serde_json::from_str::<StateCell>(&json).map_err(|error| error.to_string())?); }
    let persist = |mut cell: StateCell| -> Result<(), String> {
        let event_id = next_event_id(conn, "workspace", "diagnostic").map_err(|error| error.to_string())?;
        cell.source_event_id = event_id.clone(); cell.updated_at = now.to_string();
        append_and_apply_event(conn, &RuntimeEvent { id: event_id, case_id: "workspace".to_string(),
            kind: "workspace.diagnostic".to_string(), payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
            source: "workspace-scanner".to_string(), created_at: now.to_string(), decision_id: None })
            .map_err(|error| error.to_string())
    };
    for (path, error) in diagnostics {
        let identity = path.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let key = StateKey::new("workspace", format!("diagnostic/{identity}")).map_err(|error| error.message)?;
        let label = key.as_label(); let prior = existing.remove(&label);
        let payload = serde_json::json!({"path":path,"error":error}).to_string();
        let evidence = vec![EvidenceRef { source_type: "workspace-path".to_string(), source_id: path.clone(),
            fingerprint: stable_fingerprint(error).map_err(|error| error.message)? }];
        if prior.as_ref().is_some_and(|cell| cell.status == StateStatus::Active
            && cell.payload_json == payload && cell.evidence_refs == evidence) { continue; }
        let mut cell = prior.unwrap_or_else(|| StateCell::active(key, "workspace-diagnostic-pending"));
        cell.status = StateStatus::Active; cell.payload_schema = "workspace.import-diagnostic".to_string();
        cell.payload_json = payload; cell.evidence_refs = evidence;
        if cell.created_at.is_empty() { cell.created_at = now.to_string(); } persist(cell)?;
    }
    for (_, mut cell) in existing { if cell.status == StateStatus::Active {
        cell.status = StateStatus::Resolved; persist(cell)?; } }
    Ok(())
}

pub fn search(conn: &Connection, workspace: &Path, request: &Request) -> Result<String, String> {
    let (filter, mode) = (request.filter(), request.mode()?);
    let (mut accepted, mut excluded, mut offset) = (Vec::new(), 0, 0);
    while accepted.len() < 50 {
        let hits = store_search(conn, &request.query, &filter, mode, 50, offset)
            .map_err(|error| error.to_string())?;
        if hits.is_empty() {
            break;
        }
        offset += hits.len();
        for hit in hits {
            if current(workspace, &hit) {
                accepted.push(render(&hit, &request.query));
            } else {
                excluded += 1;
            }
            if accepted.len() == 50 {
                break;
            }
        }
    }
    if accepted.is_empty() {
        return Ok(no_matches(excluded));
    }
    if excluded > 0 {
        accepted.push(format!("excluded_drifted={excluded}"));
    }
    Ok(accepted.join("\n"))
}

fn current(workspace: &Path, hit: &SearchHit) -> bool {
    let Ok(text) = inventory::read_visible(workspace, &hit.chunk.path) else {
        return false;
    };
    lkjagent_core::workspace_record::record_fingerprint(&text)
        .is_ok_and(|fingerprint| fingerprint == hit.chunk.revision_fingerprint)
}

fn render(hit: &SearchHit, query: &str) -> String {
    format!(
        "path={} document={} field={} kind={} state={} score={:.6}\n{}",
        hit.chunk.path,
        hit.chunk.document_id,
        hit.chunk.field,
        hit.chunk.kind,
        hit.chunk.state,
        hit.score,
        excerpt(&hit.chunk.content, query),
    )
}

#[rustfmt::skip]
fn no_matches(excluded: usize) -> String {
    if excluded == 0 { "no matches".to_string() } else { format!("no matches\nexcluded_drifted={excluded}") }
}

fn excerpt(content: &str, query: &str) -> String {
    let needle = query.split_whitespace().next().unwrap_or_default();
    let index = match_index(content, needle).unwrap_or(0);
    let start = floor_boundary(content, index.saturating_sub(EXCERPT_CONTENT_BYTES / 3));
    let end = floor_boundary(content, (start + EXCERPT_CONTENT_BYTES).min(content.len()));
    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < content.len() { "..." } else { "" };
    format!("{prefix}{}{suffix}", &content[start..end])
}

fn match_index(content: &str, needle: &str) -> Option<usize> {
    if content.is_ascii() && needle.is_ascii() {
        return content
            .to_ascii_lowercase()
            .find(&needle.to_ascii_lowercase());
    }
    let folded = needle.to_lowercase();
    content
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(content.len()))
        .find(|index| content[*index..].to_lowercase().starts_with(&folded))
}

fn floor_boundary(text: &str, mut index: usize) -> usize {
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}
