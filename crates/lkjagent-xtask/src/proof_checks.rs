use std::fs;
use std::path::Path;

use rusqlite::Connection;

pub fn write_checks(conn: &Connection, out_dir: &Path) -> Result<(), String> {
    fs::write(out_dir.join("checks.md"), checks(conn)?).map_err(|error| error.to_string())
}

fn checks(conn: &Connection) -> Result<String, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, step_id, name, params_json, passed, measured, created_at
             FROM check_results ORDER BY id",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| {
            let params = row.get::<_, String>(3)?;
            let measured = row.get::<_, String>(5)?;
            Ok(format!(
                "- id={} step={} name={} passed={} params={} measured={} created_at={}",
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(4)? != 0,
                bounded(&params),
                bounded(&measured),
                row.get::<_, String>(6)?
            ))
        })
        .map_err(|error| error.to_string())?;
    let mut lines = vec!["# Checks".to_string(), String::new()];
    for row in rows {
        lines.push(row.map_err(|error| error.to_string())?);
    }
    if lines.len() == 2 {
        lines.push("none".to_string());
    }
    Ok(lines.join("\n"))
}

fn bounded(value: &str) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= 120 {
        return compact;
    }
    let head = compact.chars().take(117).collect::<String>();
    format!("{head}...")
}
