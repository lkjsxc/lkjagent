use std::fs;
use std::path::Path;

use lkjagent_core::engine::Command;
use lkjagent_core::model::{Event, EventKind, StepState, TaskSnapshot};
use lkjagent_store::plan_access::{deliver_matter_update, mark_recorded, next_pending};
use lkjagent_store::plan_commit::commit_turn;
use lkjagent_store::plan_rows::QueueRow;
use rusqlite::Connection;

use crate::clock::Clock;
use crate::snapshot_state::persist_snapshot_cell;

pub fn write_direct_records<C: Clock>(
    conn: &Connection,
    data_dir: &Path,
    clock: &mut C,
) -> Result<(), String> {
    loop {
        let Some(row) = next_pending(conn).map_err(|error| error.to_string())? else {
            return Ok(());
        };
        if row.force_new {
            return Ok(());
        }
        let now = clock.now();
        if row.route_lane.as_deref() == Some("inbox") {
            write_owner_trace(data_dir, &row, &now)?;
            mark_recorded(conn, row.id, &now).map_err(|error| error.to_string())?;
            continue;
        }
        let Some(intent) = lkjagent_core::owner_turn::record_intent(&row.content) else {
            return Ok(());
        };
        write_owner_trace(data_dir, &row, &now)?;
        crate::record_files::add(
            conn,
            data_dir,
            &intent.kind,
            &intent.title,
            &intent.body,
            &now,
        )?;
        mark_recorded(conn, row.id, &now).map_err(|error| error.to_string())?;
    }
}

pub fn attach_updates<C: Clock>(
    conn: &mut Connection,
    data_dir: &Path,
    snapshot: &mut TaskSnapshot,
    clock: &mut C,
) -> Result<(), String> {
    loop {
        let now = clock.now();
        let row = deliver_matter_update(conn, snapshot.task.id as i64, &now)
            .map_err(|error| error.to_string())?;
        let Some(row) = row else { return Ok(()) };
        write_owner_trace(data_dir, &row, &now)?;
        let event = Event {
            kind: EventKind::Owner,
            content: row.content.clone(),
        };
        append_update(snapshot, &row.content);
        snapshot.events.push(event.clone());
        commit_turn(conn, snapshot, &[Command::RecordEvent(event)], &now)
            .map_err(|error| error.to_string())?;
        persist_snapshot_cell(conn, snapshot, &now)?;
    }
}

pub(crate) fn write_owner_trace(
    data_dir: &Path,
    row: &QueueRow,
    written_at: &str,
) -> Result<(), String> {
    let lane = row.route_lane.as_deref().unwrap_or("unrouted");
    let inbox = lane == "inbox" || row.route_durability.as_deref() == Some("workspace_inbox");
    let rel = if inbox {
        format!("inbox/queue-{:06}.md", row.id)
    } else {
        format!("artifacts/transcripts/queue-{:06}.md", row.id)
    };
    let workspace = data_dir.join("workspace");
    crate::workspace_scaffold::ensure_for_path(&workspace, &rel)?;
    fs::write(workspace.join(&rel), trace_body(row, written_at, inbox))
        .map_err(|error| error.to_string())?;
    crate::workspace_scaffold::refresh_for_path(&workspace, &rel)
}

pub(crate) fn write_send_trace(
    data_dir: &Path,
    id: i64,
    text: &str,
    force_new: bool,
    now: &str,
) -> Result<(), String> {
    let route = lkjagent_core::owner_turn::route_turn(
        text,
        lkjagent_core::owner_turn::RouteContext {
            force_new,
            ..Default::default()
        },
    );
    let row = QueueRow {
        id,
        content: text.to_string(),
        state: "pending".to_string(),
        task_id: None,
        force_new,
        route_lane: route.as_ref().map(|value| value.lane.clone()),
        route_durability: route.as_ref().map(|value| value.desired_durability.clone()),
        route_title_seed: route.as_ref().map(|value| value.title_seed.clone()),
        route_transform_allowed: route.as_ref().map(|value| value.transformation_allowed),
    };
    write_owner_trace(data_dir, &row, now)
}

fn trace_body(row: &QueueRow, written_at: &str, inbox: bool) -> String {
    let kind = if inbox { "inbox" } else { "transcript" };
    [
        format!("# Owner Turn {}", row.id),
        String::new(),
        "Purpose: durable owner-turn workspace evidence.".to_string(),
        String::new(),
        "## Route".to_string(),
        String::new(),
        format!("- kind: {kind}"),
        format!("- queue_id: {}", row.id),
        format!("- queue_state: {}", row.state),
        format!(
            "- task_id: {}",
            row.task_id.map_or("none".to_string(), |id| id.to_string())
        ),
        format!("- force_new: {}", row.force_new),
        format!("- route_lane: {}", option_text(&row.route_lane)),
        format!("- route_durability: {}", option_text(&row.route_durability)),
        format!(
            "- transform_allowed: {}",
            bool_text(row.route_transform_allowed)
        ),
        format!("- written_at: {written_at}"),
        String::new(),
        "## Owner Text".to_string(),
        String::new(),
        row.content.clone(),
        String::new(),
    ]
    .join("\n")
}

fn option_text(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "none".to_string())
}

fn bool_text(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "none",
    }
}

fn append_update(snapshot: &mut TaskSnapshot, content: &str) {
    snapshot.task.brief.push_str("\nowner_update=");
    snapshot.task.brief.push_str(content);
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .find(|step| matches!(step.state, StepState::Pending | StepState::Active))
    {
        step.inputs.push_str("\nowner_update=");
        step.inputs.push_str(content);
    }
}
