use std::path::Path;

use lkjagent_core::engine::Command;
use lkjagent_core::model::{Event, EventKind, StepState, TaskSnapshot};
use lkjagent_store::plan_access::{deliver_matter_update, mark_recorded, next_pending};
use lkjagent_store::plan_commit::commit_turn;
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
        let Some(intent) = lkjagent_core::owner_turn::record_intent(&row.content) else {
            return Ok(());
        };
        let now = clock.now();
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
    snapshot: &mut TaskSnapshot,
    clock: &mut C,
) -> Result<(), String> {
    loop {
        let now = clock.now();
        let row = deliver_matter_update(conn, snapshot.task.id as i64, &now)
            .map_err(|error| error.to_string())?;
        let Some(row) = row else { return Ok(()) };
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
