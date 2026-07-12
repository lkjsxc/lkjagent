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
type Selector = fn(&RecordRow, &str) -> bool;

pub fn rebuild(conn: &Connection, data_dir: &Path, now: &str) -> Result<String, String> {
    let workspace = crate::config::workspace_root(data_dir)?;
    for rel in [
        "indexes/.directory",
        "records/.directory",
        "artifacts/.directory",
    ] {
        let _ = crate::effect_files::open_parent(&workspace, rel, true)?;
    }
    crate::workspace_root::refresh_for_path(&workspace, "records/README.md")?;
    crate::workspace_root::refresh_for_path(&workspace, "artifacts/README.md")?;
    crate::workspace_root::refresh_for_path(&workspace, "indexes/README.md")?;
    let search = crate::workspace_search::rebuild(conn, &workspace)?;
    let rows = records(conn, None, false).map_err(|error| error.to_string())?;
    let specs: [(&str, Selector); 7] = [
        ("today", today),
        ("agenda", agenda),
        ("open-todos", open_todo),
        ("budget-month", finance),
        ("active-projects", active_project),
        ("proof-runs", proof),
        ("experiments", experiment),
    ];
    for (name, select) in specs {
        write_index(conn, &workspace, name, select, &rows, now)?;
        crate::workspace_root::refresh_for_path(&workspace, &format!("indexes/{name}.md"))?;
    }
    suppress_stale_cell(conn, now)?;
    Ok(format!(
        "workspace indexes rebuilt: {}; {search}",
        specs.len()
    ))
}

fn write_index(
    conn: &Connection,
    workspace: &Path,
    name: &str,
    select: Selector,
    rows: &[RecordRow],
    now: &str,
) -> Result<(), String> {
    let selected = rows
        .iter()
        .filter(|row| select(row, now))
        .collect::<Vec<_>>();
    let body = index_body(name, &selected);
    let path = format!("indexes/{name}.md");
    crate::effect_files::write_bytes(workspace, &path, body.as_bytes())?;
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

fn index_body(name: &str, rows: &[&RecordRow]) -> String {
    let mut lines = vec![
        "---".to_string(),
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

fn today(row: &RecordRow, now: &str) -> bool {
    matches!(row.kind.as_str(), "today" | "journal") && same_date(&row.updated_at, now)
}

fn agenda(row: &RecordRow, now: &str) -> bool {
    row.kind == "calendar" && actionable(&row.state) && row.updated_at.as_str() >= date(now)
}

fn open_todo(row: &RecordRow, _now: &str) -> bool {
    row.kind == "todo" && actionable(&row.state)
}

fn active_project(row: &RecordRow, _now: &str) -> bool {
    row.kind == "project" && row.state == "active"
}

fn finance(row: &RecordRow, _now: &str) -> bool {
    row.kind == "finance"
}

fn proof(row: &RecordRow, _now: &str) -> bool {
    row.kind == "proof"
}

fn experiment(row: &RecordRow, _now: &str) -> bool {
    row.kind == "experiment"
}

fn actionable(state: &str) -> bool {
    matches!(state, "open" | "ready" | "due")
}

fn same_date(value: &str, now: &str) -> bool {
    value.split('T').next() == now.split('T').next()
}

fn date(value: &str) -> &str {
    value.split('T').next().unwrap_or_default()
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
