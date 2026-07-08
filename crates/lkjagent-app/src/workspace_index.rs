use std::fs;
use std::path::Path;

use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::runtime_state::StateKey;
use lkjagent_store::artifact_rows::{insert_artifact, ArtifactRow};
use lkjagent_store::event_rows::{append_and_apply_event, next_event_id};
use lkjagent_store::record_rows::{records, RecordRow};
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

const CASE_ID: &str = "workspace";

pub fn rebuild(conn: &Connection, data_dir: &Path, now: &str) -> Result<String, String> {
    let rows = records(conn, None, false).map_err(|error| error.to_string())?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let indexes = workspace.join("indexes");
    fs::create_dir_all(&indexes).map_err(|error| error.to_string())?;
    fs::create_dir_all(workspace.join("records")).map_err(|error| error.to_string())?;
    fs::create_dir_all(workspace.join("artifacts")).map_err(|error| error.to_string())?;
    crate::workspace_scaffold::refresh_for_path(&workspace, "records/README.md")?;
    crate::workspace_scaffold::refresh_for_path(&workspace, "artifacts/README.md")?;
    crate::workspace_scaffold::refresh_for_path(&workspace, "indexes/README.md")?;
    let specs = [
        ("today", &["today", "journal"][..]),
        ("agenda", &["calendar"][..]),
        ("open-todos", &["todo"][..]),
        ("active-projects", &["project"][..]),
        ("proof-runs", &["proof"][..]),
        ("experiments", &["experiment"][..]),
    ];
    for (name, kinds) in specs {
        write_index(conn, &indexes, name, kinds, &rows, now)?;
        crate::workspace_scaffold::refresh_for_path(&workspace, &format!("indexes/{name}.md"))?;
    }
    suppress_stale_cell(conn, now)?;
    Ok(format!("workspace indexes rebuilt: {}", specs.len()))
}

fn write_index(
    conn: &Connection,
    indexes: &Path,
    name: &str,
    kinds: &[&str],
    rows: &[RecordRow],
    now: &str,
) -> Result<(), String> {
    let selected = rows
        .iter()
        .filter(|row| kinds.iter().any(|kind| *kind == row.kind))
        .collect::<Vec<_>>();
    let body = index_body(name, &selected, now);
    let path = indexes.join(format!("{name}.md"));
    fs::write(&path, &body).map_err(|error| error.to_string())?;
    let fingerprint = stable_fingerprint(&body).map_err(|error| error.message)?;
    insert_artifact(
        conn,
        &ArtifactRow {
            id: format!("index-{name}"),
            case_id: CASE_ID.to_string(),
            kind: "workspace-index".to_string(),
            path: format!("indexes/{name}.md"),
            fingerprint,
            parent_artifact_id: None,
            metadata_json: serde_json::json!({
                "input_records": selected.iter().map(|row| &row.id).collect::<Vec<_>>(),
                "stale_reason": null_reason(),
            })
            .to_string(),
            created_at: now.to_string(),
        },
    )
    .map_err(|error| error.to_string())
}

fn index_body(name: &str, rows: &[&RecordRow], now: &str) -> String {
    let mut lines = vec![
        "---".to_string(),
        format!("generated_at: {now}"),
        format!("index: {name}"),
        format!(
            "input_records: [{}]",
            rows.iter()
                .map(|row| row.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        "stale_reason: none".to_string(),
        "---".to_string(),
        String::new(),
        format!("# {}", title(name)),
        String::new(),
    ];
    if rows.is_empty() {
        lines.push("none".to_string());
    }
    for row in rows {
        lines.push(format!(
            "- {} [{}] {} ({})",
            row.id, row.state, row.title, row.path
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn suppress_stale_cell(conn: &Connection, now: &str) -> Result<(), String> {
    insert_case(conn, CASE_ID, "workspace records", now).map_err(|error| error.to_string())?;
    let event_id = next_event_id(conn, CASE_ID, "index-rebuild").map_err(|e| e.to_string())?;
    let event = RuntimeEvent {
        id: event_id,
        case_id: CASE_ID.to_string(),
        kind: "state.cell.suppress".to_string(),
        payload: RuntimeEventPayload::SuppressCell {
            key: StateKey::new("index", "stale/records").map_err(|error| error.message)?,
            reason: "workspace indexes rebuilt".to_string(),
        },
        source: "workspace-index".to_string(),
        created_at: now.to_string(),
        decision_id: None,
    };
    append_and_apply_event(conn, &event).map_err(|error| error.to_string())
}

fn title(name: &str) -> String {
    name.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn null_reason() -> Option<String> {
    None
}
