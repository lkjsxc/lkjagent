use rusqlite::{params, Connection, Row};

use crate::error::{StoreError, StoreResult};
use crate::personal::model::{PersonalListFilter, PersonalRecord};

pub fn get(conn: &Connection, id: i64) -> StoreResult<PersonalRecord> {
    conn.query_row(
        "SELECT id,kind,title,body,status,tags,timezone,start_at,end_at,due_at,
        recurrence,priority,project,source_case_id,created_at,updated_at,closed_at
        FROM personal_records WHERE id = ?1",
        [id],
        row_to_record,
    )
    .map_err(|error| match error {
        rusqlite::Error::QueryReturnedNoRows => {
            StoreError::NotFound(format!("personal record {id}"))
        }
        other => StoreError::from(other),
    })
}

pub fn list(
    conn: &Connection,
    filter: &PersonalListFilter<'_>,
) -> StoreResult<Vec<PersonalRecord>> {
    let limit = bounded_limit(filter.limit, 20);
    let scan_limit = 100;
    let mut records = match (filter.kind, filter.status, filter.project) {
        (Some(kind), Some(status), Some(project)) => query(
            conn,
            "WHERE kind = ?1 AND status = ?2 AND project = ?3",
            params![kind, status, project],
            scan_limit,
        )?,
        (Some(kind), Some(status), None) => query(
            conn,
            "WHERE kind = ?1 AND status = ?2",
            params![kind, status],
            scan_limit,
        )?,
        (Some(kind), None, Some(project)) => query(
            conn,
            "WHERE kind = ?1 AND project = ?2",
            params![kind, project],
            scan_limit,
        )?,
        (Some(kind), None, None) => query(conn, "WHERE kind = ?1", params![kind], scan_limit)?,
        (None, Some(status), Some(project)) => query(
            conn,
            "WHERE status = ?1 AND project = ?2",
            params![status, project],
            scan_limit,
        )?,
        (None, Some(status), None) => {
            query(conn, "WHERE status = ?1", params![status], scan_limit)?
        }
        (None, None, Some(project)) => {
            query(conn, "WHERE project = ?1", params![project], scan_limit)?
        }
        _ => query(conn, "", params![], scan_limit)?,
    };
    records.retain(|record| in_range(record.start_at.as_deref(), filter));
    records.truncate(limit);
    Ok(records)
}

pub fn search(
    conn: &Connection,
    query_text: &str,
    limit: usize,
) -> StoreResult<Vec<PersonalRecord>> {
    let query_text = normalize_query(query_text);
    if query_text.is_empty() {
        return Ok(Vec::new());
    }
    let limit = bounded_limit(limit, 10);
    let mut statement = conn.prepare(
        "SELECT r.id,r.kind,r.title,r.body,r.status,r.tags,r.timezone,r.start_at,
        r.end_at,r.due_at,r.recurrence,r.priority,r.project,r.source_case_id,
        r.created_at,r.updated_at,r.closed_at
        FROM personal_records_fts f
        JOIN personal_records r ON r.id = f.rowid
        WHERE personal_records_fts MATCH ?1
        ORDER BY rank LIMIT ?2",
    )?;
    let records = rows(statement.query_map(params![query_text, limit as i64], row_to_record)?)?;
    Ok(records)
}

fn query<P: rusqlite::Params>(
    conn: &Connection,
    where_clause: &str,
    params: P,
    limit: usize,
) -> StoreResult<Vec<PersonalRecord>> {
    let sql = format!(
        "SELECT id,kind,title,body,status,tags,timezone,start_at,end_at,due_at,
        recurrence,priority,project,source_case_id,created_at,updated_at,closed_at
        FROM personal_records {where_clause}
        ORDER BY COALESCE(start_at, due_at, created_at), id LIMIT {limit}"
    );
    let mut statement = conn.prepare(&sql)?;
    let records = rows(statement.query_map(params, row_to_record)?)?;
    Ok(records)
}

fn rows<I>(mapped: I) -> StoreResult<Vec<PersonalRecord>>
where
    I: IntoIterator<Item = rusqlite::Result<PersonalRecord>>,
{
    let mut out = Vec::new();
    for row in mapped {
        out.push(row?);
    }
    Ok(out)
}

fn in_range(value: Option<&str>, filter: &PersonalListFilter<'_>) -> bool {
    let Some(value) = value else {
        return filter.start.is_none() && filter.end.is_none();
    };
    if filter.start.is_some_and(|start| value < start) {
        return false;
    }
    if filter.end.is_some_and(|end| value > end) {
        return false;
    }
    true
}

fn row_to_record(row: &Row<'_>) -> rusqlite::Result<PersonalRecord> {
    Ok(PersonalRecord {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        body: row.get(3)?,
        status: row.get(4)?,
        tags: row.get(5)?,
        timezone: row.get(6)?,
        start_at: row.get(7)?,
        end_at: row.get(8)?,
        due_at: row.get(9)?,
        recurrence: row.get(10)?,
        priority: row.get(11)?,
        project: row.get(12)?,
        source_case_id: row.get(13)?,
        created_at: row.get(14)?,
        updated_at: row.get(15)?,
        closed_at: row.get(16)?,
    })
}

fn bounded_limit(limit: usize, default_limit: usize) -> usize {
    let value = if limit == 0 { default_limit } else { limit };
    value.clamp(1, 100)
}

fn normalize_query(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
}
