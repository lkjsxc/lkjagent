use rusqlite::Connection;
use std::fs;
use std::path::Path;
pub fn write_state_bundle(conn: &Connection, out_dir: &Path) -> Result<(), String> {
    write(out_dir, "state-vector.md", &state_cells(conn)?)?;
    write(out_dir, "decisions.md", &decisions(conn)?)?;
    write(out_dir, "prompt-frames.md", &prompt_frames(conn)?)?;
    write(out_dir, "prompt-cards.md", &prompt_cards(conn)?)?;
    write(
        out_dir,
        "admissions.md",
        &count_doc(conn, "tool_admissions")?,
    )?;
    write(out_dir, "observations.md", &observations(conn)?)?;
    write(out_dir, "exchanges.md", &exchanges(conn)?)?;
    write(out_dir, "artifacts.md", &artifacts(conn)?)?;
    write(out_dir, "context.md", &context(conn)?)?;
    write(out_dir, "context-edges.md", &context_edges(conn)?)
}
fn state_cells(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "State Vector",
        "SELECT case_id, key_label, status, payload_schema FROM state_cells ORDER BY case_id, key_label",
        |row| {
            Ok(format!(
                "- case={} {} status={} schema={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    )
}
fn decisions(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Decisions",
        "SELECT id, case_id, operation_key, status, tool_view_fingerprint FROM runtime_decisions ORDER BY selected_at, id",
        |row| {
            Ok(format!(
                "- {} case={} op={} status={} tools={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?
            ))
        },
    )
}
fn observations(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Observations",
        "SELECT decision_id, effect_name, status FROM observations ORDER BY id",
        |row| {
            Ok(format!(
                "- decision={} effect={} status={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?
            ))
        },
    )
}
fn prompt_frames(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Prompt Frames",
        "SELECT decision_id, prompt_fingerprint, body_ref FROM prompt_frames ORDER BY id",
        |row| {
            Ok(format!(
                "- decision={} prompt={} body={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?
            ))
        },
    )
}
fn prompt_cards(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Prompt Cards",
        "SELECT decision_id, kind, fingerprint, reason FROM prompt_cards ORDER BY id",
        |row| {
            Ok(format!(
                "- decision={} kind={} fp={} reason={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    )
}
fn exchanges(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Exchanges",
        "SELECT decision_id, exchange_ref, COALESCE(timeout_seconds, 0) FROM provider_exchanges ORDER BY id",
        |row| {
            Ok(format!(
                "- decision={} ref={} timeout={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?
            ))
        },
    )
}
fn artifacts(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Artifacts",
        "SELECT id, kind, path, fingerprint FROM artifacts ORDER BY id",
        |row| {
            Ok(format!(
                "- {} kind={} path={} fp={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    )
}
fn context(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Context",
        "SELECT id, semantic_key, contamination_class,
         COALESCE(suppression_reason, 'none') FROM context_items ORDER BY id",
        |row| {
            Ok(format!(
                "- {} key={} contamination={} suppressed={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    )
}
fn context_edges(conn: &Connection) -> Result<String, String> {
    rows(
        conn,
        "Context Edges",
        "SELECT from_item_id, to_item_id, edge_kind, reason FROM context_edges ORDER BY id",
        |row| {
            Ok(format!(
                "- {} -> {} kind={} reason={}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ))
        },
    )
}

fn count_doc(conn: &Connection, table: &str) -> Result<String, String> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    let count = conn.query_row(&sql, [], |row| row.get::<_, i64>(0));
    match count {
        Ok(count) => Ok(format!("# {table}\n\ncount={count}\n")),
        Err(error) if error.to_string().contains("no such table") => {
            Ok(format!("# {table}\n\ncount=0\n"))
        }
        Err(error) => Err(error.to_string()),
    }
}
fn rows<F>(conn: &Connection, title: &str, sql: &str, render: F) -> Result<String, String>
where
    F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<String>,
{
    let mut statement = match conn.prepare(sql) {
        Ok(statement) => statement,
        Err(error) if error.to_string().contains("no such table") => {
            return Ok(format!("# {title}\n\nnone"))
        }
        Err(error) => return Err(error.to_string()),
    };
    let mapped = statement
        .query_map([], render)
        .map_err(|error| error.to_string())?;
    let mut lines = vec![format!("# {title}"), String::new()];
    for row in mapped {
        lines.push(row.map_err(|error| error.to_string())?);
    }
    if lines.len() == 2 {
        lines.push("none".to_string());
    }
    Ok(lines.join("\n"))
}
fn write(dir: &Path, name: &str, body: &str) -> Result<(), String> {
    fs::write(dir.join(name), body).map_err(|error| error.to_string())
}
