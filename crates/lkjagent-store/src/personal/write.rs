use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
use crate::personal::model::{PersonalRecord, PersonalRecordInput, PersonalRecordUpdate};
use crate::personal::read::get;
use crate::personal::validate::{validate_input, validate_status};

pub fn create(conn: &Connection, input: &PersonalRecordInput<'_>) -> StoreResult<i64> {
    validate_input(input)?;
    conn.execute(
        "INSERT INTO personal_records
        (kind,title,body,status,tags,timezone,start_at,end_at,due_at,recurrence,
         priority,project,source_case_id,created_at,updated_at,closed_at)
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?14,?15)",
        params![
            input.kind,
            input.title,
            input.body,
            input.status,
            input.tags,
            input.timezone,
            input.start_at,
            input.end_at,
            input.due_at,
            input.recurrence,
            input.priority,
            input.project,
            input.source_case_id,
            input.now,
            closed_at(input.status, input.now),
        ],
    )?;
    let id = conn.last_insert_rowid();
    upsert_fts(conn, id, input.title, input.body, input.tags, input.project)?;
    record_event(conn, id, "created", "record created", input.now)?;
    Ok(id)
}

pub fn update_status(conn: &Connection, id: i64, status: &str, now: &str) -> StoreResult<()> {
    validate_status(status)?;
    let changed = conn.execute(
        "UPDATE personal_records
         SET status = ?1, updated_at = ?2, closed_at = ?3
         WHERE id = ?4",
        params![status, now, closed_at(status, now), id],
    )?;
    if changed == 0 {
        return Err(StoreError::NotFound(format!("personal record {id}")));
    }
    record_event(conn, id, "status_changed", &format!("status={status}"), now)
}

pub fn update(conn: &Connection, input: &PersonalRecordUpdate<'_>) -> StoreResult<()> {
    let existing = get(conn, input.id)?;
    let merged = merged_input(&existing, input);
    validate_input(&merged)?;
    let changed = conn.execute(
        "UPDATE personal_records
        SET title=?1, body=?2, status=?3, tags=?4, timezone=?5,
            start_at=?6, end_at=?7, due_at=?8, recurrence=?9, priority=?10,
            project=?11, updated_at=?12, closed_at=?13
        WHERE id=?14",
        params![
            merged.title,
            merged.body,
            merged.status,
            merged.tags,
            merged.timezone,
            merged.start_at,
            merged.end_at,
            merged.due_at,
            merged.recurrence,
            merged.priority,
            merged.project,
            merged.now,
            closed_at(merged.status, merged.now),
            input.id,
        ],
    )?;
    if changed == 0 {
        return Err(StoreError::NotFound(format!(
            "personal record {}",
            input.id
        )));
    }
    refresh_fts(conn, input.id, &existing, &merged)?;
    let event_kind = if input.status.is_some() {
        "status_changed"
    } else {
        "amended"
    };
    record_event(conn, input.id, event_kind, "record updated", input.now)
}

pub fn link(
    conn: &Connection,
    source_record_id: i64,
    relation: &str,
    target_record_id: i64,
    now: &str,
) -> StoreResult<i64> {
    if relation.trim().is_empty() {
        return Err(StoreError::InvalidState(
            "relation must not be empty".to_string(),
        ));
    }
    conn.execute(
        "INSERT INTO personal_record_links
        (source_record_id, relation, target_record_id, created_at)
        VALUES (?1, ?2, ?3, ?4)",
        params![source_record_id, relation, target_record_id, now],
    )?;
    Ok(conn.last_insert_rowid())
}

fn merged_input<'a>(
    existing: &'a PersonalRecord,
    update: &'a PersonalRecordUpdate<'a>,
) -> PersonalRecordInput<'a> {
    PersonalRecordInput {
        kind: &existing.kind,
        title: update.title.unwrap_or(&existing.title),
        body: update.body.unwrap_or(&existing.body),
        status: update.status.unwrap_or(&existing.status),
        tags: update.tags.unwrap_or(&existing.tags),
        timezone: update.timezone.or(existing.timezone.as_deref()),
        start_at: update.start_at.or(existing.start_at.as_deref()),
        end_at: update.end_at.or(existing.end_at.as_deref()),
        due_at: update.due_at.or(existing.due_at.as_deref()),
        recurrence: update.recurrence.or(existing.recurrence.as_deref()),
        priority: update.priority.or(existing.priority.as_deref()),
        project: update.project.or(existing.project.as_deref()),
        source_case_id: existing.source_case_id,
        now: update.now,
    }
}

fn refresh_fts(
    conn: &Connection,
    id: i64,
    old: &PersonalRecord,
    input: &PersonalRecordInput<'_>,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO personal_records_fts
        (personal_records_fts, rowid, title, body, tags, project)
        VALUES ('delete', ?1, ?2, ?3, ?4, ?5)",
        params![
            id,
            old.title,
            old.body,
            old.tags,
            old.project.as_deref().unwrap_or("")
        ],
    )?;
    upsert_fts(conn, id, input.title, input.body, input.tags, input.project)
}

pub(crate) fn record_event(
    conn: &Connection,
    record_id: i64,
    event_kind: &str,
    summary: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO personal_record_events
        (record_id, event_kind, summary, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![record_id, event_kind, summary, now],
    )?;
    Ok(())
}

fn upsert_fts(
    conn: &Connection,
    id: i64,
    title: &str,
    body: &str,
    tags: &str,
    project: Option<&str>,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO personal_records_fts(rowid, title, body, tags, project)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, title, body, tags, project.unwrap_or("")],
    )?;
    Ok(())
}

fn closed_at(status: &str, now: &str) -> Option<String> {
    matches!(status, "done" | "canceled").then(|| now.to_string())
}
