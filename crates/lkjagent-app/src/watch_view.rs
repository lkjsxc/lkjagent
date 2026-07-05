use rusqlite::Connection;

use crate::state::load_snapshot;

pub fn watch(conn: &Connection) -> Result<String, String> {
    let status = crate::status::status(conn)?;
    let events = crate::log_view::log(conn, 8)?;
    let trace = task_trace(conn)?;
    let proof = proof_line(conn)?;
    Ok([
        "watch: rerun to refresh; use log --follow to stream".to_string(),
        "== status ==".to_string(),
        status,
        "== recent events ==".to_string(),
        events,
        "== task trace ==".to_string(),
        trace,
        "== proof rows ==".to_string(),
        proof,
    ]
    .join("\n"))
}

fn task_trace(conn: &Connection) -> Result<String, String> {
    if let Some(snapshot) = load_snapshot(conn).map_err(|error| error.to_string())? {
        return Ok(crate::status::task_show(&snapshot));
    }
    let Some(id) = latest_task_id(conn)? else {
        return Ok("task: none".to_string());
    };
    lkjagent_store::plan_hydrate::snapshot_by_id(conn, id)
        .map_err(|error| error.to_string())?
        .map_or_else(
            || Ok("task: none".to_string()),
            |snapshot| Ok(crate::status::task_show(&snapshot)),
        )
}

fn latest_task_id(conn: &Connection) -> Result<Option<i64>, String> {
    let row = conn.query_row("SELECT id FROM tasks ORDER BY id DESC LIMIT 1", [], |row| {
        row.get(0)
    });
    match row {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

fn proof_line(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "proof: prompt_frames={} checks={} artifacts={} exchanges={}",
        count(conn, "prompt_frames")?,
        count(conn, "check_results")?,
        count(conn, "artifacts")?,
        count(conn, "provider_exchanges")?
    ))
}

fn count(conn: &Connection, table: &str) -> Result<i64, String> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .map_err(|error| error.to_string())
}
