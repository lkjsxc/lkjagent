use rusqlite::{params, Connection};

use crate::status::task_show as render_task;

pub fn log(conn: &Connection, limit: usize) -> Result<String, String> {
    crate::log_view::log(conn, limit)
}

pub fn follow_log(conn: &Connection, limit: usize) -> Result<String, String> {
    crate::log_view::follow_log(conn, limit)
}

pub fn task_list(conn: &Connection) -> Result<String, String> {
    let mut statement = conn
        .prepare("SELECT id, state, template, summary FROM tasks ORDER BY id")
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| {
            Ok(format!(
                "task {} {} {} {}",
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        })
        .map_err(|error| error.to_string())?;
    collect(rows)
}

pub fn task_show(conn: &Connection, id: u64) -> Result<String, String> {
    let snapshot = lkjagent_store::plan_hydrate::snapshot_by_id(conn, id as i64)
        .map_err(|error| error.to_string())?;
    snapshot.map_or_else(
        || Ok(format!("task {id}: not found")),
        |snap| Ok(render_task(&snap)),
    )
}

pub fn queue_list(conn: &Connection) -> Result<String, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, state, force_new, COALESCE(task_id, 0), content FROM queue ORDER BY id",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| {
            Ok(format!(
                "queue {} {} force_new={} task={} {}",
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)? != 0,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?
            ))
        })
        .map_err(|error| error.to_string())?;
    collect(rows)
}

pub fn queue_show(conn: &Connection, id: i64) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT id, content, state, force_new, COALESCE(task_id, 0) FROM queue WHERE id = ?1",
        params![id],
        |row| {
            Ok(format!(
                "queue {} state={} force_new={} task={} content={}",
                row.get::<_, i64>(0)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, i64>(4)?,
                row.get::<_, String>(1)?
            ))
        },
    );
    match row {
        Ok(row) => Ok(row),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(format!("queue {id}: not found")),
        Err(error) => Err(error.to_string()),
    }
}

pub fn memory(conn: &Connection, query: &str) -> Result<String, String> {
    if query.trim().is_empty() {
        return Ok("memory: query required".to_string());
    }
    let rows = lkjagent_store::memory::search_memory(conn, query, 20)
        .map_err(|error| error.to_string())?;
    if rows.is_empty() {
        return Ok("none".to_string());
    }
    Ok(rows
        .iter()
        .map(|row| format!("memory {} {} {}", row.id, row.topic, row.content))
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn watch(conn: &Connection) -> Result<String, String> {
    crate::watch_view::watch(conn)
}

fn collect(rows: impl Iterator<Item = rusqlite::Result<String>>) -> Result<String, String> {
    let output = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(if output.is_empty() {
        "none".to_string()
    } else {
        output.join("\n")
    })
}
