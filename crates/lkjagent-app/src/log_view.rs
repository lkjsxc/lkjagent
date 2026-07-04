use std::io::Write;
use std::time::Duration;

use rusqlite::{params, Connection};

pub fn log(conn: &Connection, limit: usize) -> Result<String, String> {
    let rows = latest_log_rows(conn, limit)?;
    Ok(log_output(&rows))
}

pub fn follow_log(conn: &Connection, limit: usize) -> Result<String, String> {
    let initial = latest_log_rows(conn, limit)?;
    if initial.is_empty() {
        println!("none");
    }
    for row in &initial {
        println!("{}", row.line);
    }
    let mut last_id = initial.last().map_or(0, |row| row.id);
    loop {
        for row in log_rows_after(conn, last_id)? {
            last_id = row.id;
            println!("{}", row.line);
        }
        std::io::stdout()
            .flush()
            .map_err(|error| error.to_string())?;
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LogRow {
    id: i64,
    line: String,
}

fn latest_log_rows(conn: &Connection, limit: usize) -> Result<Vec<LogRow>, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, COALESCE(task_id, 0), kind, content FROM events
             ORDER BY id DESC LIMIT ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![limit as i64], log_row)
        .map_err(|error| error.to_string())?;
    let mut output = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    output.reverse();
    Ok(output)
}

fn log_rows_after(conn: &Connection, after_id: i64) -> Result<Vec<LogRow>, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, COALESCE(task_id, 0), kind, content FROM events
             WHERE id > ?1 ORDER BY id",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![after_id], log_row)
        .map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn log_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<LogRow> {
    let id = row.get::<_, i64>(0)?;
    Ok(LogRow {
        id,
        line: format!(
            "{} task={} {} {}",
            id,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?
        ),
    })
}

fn log_output(rows: &[LogRow]) -> String {
    if rows.is_empty() {
        "none".to_string()
    } else {
        rows.iter()
            .map(|row| row.line.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_store::plan_schema::setup;

    #[test]
    fn log_rows_after_returns_only_new_rows() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        insert_event(&conn, 1, "queued")?;
        insert_event(&conn, 2, "selected")?;

        let initial = log(&conn, 1)?;
        assert!(initial.contains("selected"));
        assert!(!initial.contains("queued"));

        insert_event(&conn, 3, "observed")?;
        let rows = log_rows_after(&conn, 2)?;
        assert_eq!(rows.len(), 1);
        assert!(rows[0].line.contains("observed"));
        Ok(())
    }

    fn insert_event(conn: &Connection, id: i64, content: &str) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO events (id, task_id, kind, content, created_at)
             VALUES (?1, 1, 'notice', ?2, 'now')",
            params![id, content],
        )?;
        Ok(())
    }
}
