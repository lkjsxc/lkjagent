use std::fs;
use std::path::Path;

use rusqlite::Connection;

pub fn write_attempts_and_tokens(conn: &Connection, out_dir: &Path) -> Result<(), String> {
    write(out_dir, "attempts.md", &attempts(conn)?)?;
    write(out_dir, "token-usage.md", &token_usage(conn)?)
}

fn attempts(conn: &Connection) -> Result<String, String> {
    let count = count(conn, "attempts")?;
    Ok(format!("# Attempts\n\ncount={count}\n"))
}

fn token_usage(conn: &Connection) -> Result<String, String> {
    let mut statement = match conn.prepare(
        "SELECT task_id, attempt_id, input_total_tokens, input_cached_tokens,
         input_uncached_tokens, output_tokens, cache_status, raw_usage_json FROM token_usage ORDER BY id",
    ) {
        Ok(statement) => statement,
        Err(error) if error.to_string().contains("no such table") => {
            return Ok("# Token Usage\n\nnone".to_string())
        }
        Err(error) => return Err(error.to_string()),
    };
    let rows = statement
        .query_map([], |row| {
            Ok(format!(
                "- matter={} attempt={} input_total={} input_cached={} input_uncached={} output={} cache={} raw_usage={}",
                row.get::<_, i64>(0)?,
                nullable(row.get::<_, Option<i64>>(1)?),
                nullable(row.get::<_, Option<i64>>(2)?),
                nullable(row.get::<_, Option<i64>>(3)?),
                nullable(row.get::<_, Option<i64>>(4)?),
                nullable(row.get::<_, Option<i64>>(5)?),
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?
            ))
        })
        .map_err(|error| error.to_string())?;
    let mut lines = vec!["# Token Usage".to_string(), String::new()];
    for row in rows {
        lines.push(row.map_err(|error| error.to_string())?);
    }
    if lines.len() == 2 {
        lines.push("none".to_string());
    }
    Ok(lines.join("\n"))
}

fn nullable(value: Option<i64>) -> String {
    value.map_or_else(|| "unknown".to_string(), |value| value.to_string())
}

fn count(conn: &Connection, table: &str) -> Result<i64, String> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .map_err(|e| e.to_string())
}

fn write(dir: &Path, name: &str, body: &str) -> Result<(), String> {
    fs::write(dir.join(name), body).map_err(|error| error.to_string())
}
