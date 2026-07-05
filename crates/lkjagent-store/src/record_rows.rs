use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRow {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub state: String,
    pub path: String,
    pub fingerprint: String,
    pub archived: bool,
    pub updated_at: String,
}

pub fn upsert_record(conn: &Connection, row: &RecordRow) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO workspace_records
         (id, kind, title, state, path, fingerprint, archived, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET kind=excluded.kind, title=excluded.title,
         state=excluded.state, path=excluded.path, fingerprint=excluded.fingerprint,
         archived=excluded.archived, updated_at=excluded.updated_at",
        params![
            row.id,
            row.kind,
            row.title,
            row.state,
            row.path,
            row.fingerprint,
            i64::from(row.archived),
            row.updated_at,
        ],
    )?;
    conn.execute(
        "INSERT INTO workspace_record_history
         (record_id, path, fingerprint, state, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![row.id, row.path, row.fingerprint, row.state, row.updated_at],
    )?;
    Ok(())
}

pub fn records(
    conn: &Connection,
    kind: Option<&str>,
    include_archived: bool,
) -> StoreResult<Vec<RecordRow>> {
    let mut sql = "SELECT id, kind, title, state, path, fingerprint, archived,
         updated_at FROM workspace_records"
        .to_string();
    if kind.is_some() || !include_archived {
        sql.push_str(" WHERE ");
        let mut filters = Vec::new();
        if kind.is_some() {
            filters.push("kind = ?1");
        }
        if !include_archived {
            filters.push("archived = 0");
        }
        sql.push_str(&filters.join(" AND "));
    }
    sql.push_str(" ORDER BY updated_at DESC, id");
    let mut statement = conn.prepare(&sql)?;
    let mut output = Vec::new();
    if let Some(kind) = kind {
        for row in statement.query_map([kind], row)? {
            output.push(row?);
        }
    } else {
        for row in statement.query_map([], row)? {
            output.push(row?);
        }
    }
    Ok(output)
}

pub fn record(conn: &Connection, id: &str) -> StoreResult<Option<RecordRow>> {
    let result = conn.query_row(
        "SELECT id, kind, title, state, path, fingerprint, archived, updated_at
         FROM workspace_records WHERE id = ?1",
        [id],
        row,
    );
    match result {
        Ok(row) => Ok(Some(row)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RecordRow> {
    let archived: i64 = row.get(6)?;
    Ok(RecordRow {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        state: row.get(3)?,
        path: row.get(4)?,
        fingerprint: row.get(5)?,
        archived: archived != 0,
        updated_at: row.get(7)?,
    })
}
