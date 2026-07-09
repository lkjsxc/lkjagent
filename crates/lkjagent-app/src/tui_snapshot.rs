use std::path::Path;

use rusqlite::Connection;

use crate::tui_types::{TranscriptEntry, TranscriptSource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiSnapshot {
    pub status: String,
    pub logs: String,
    pub transcript: String,
    pub transcript_entries: Vec<TranscriptEntry>,
    pub tasks: String,
    pub queue: String,
    pub proof: String,
    pub workspace: String,
    pub tools: String,
}

impl TuiSnapshot {
    pub fn empty() -> Self {
        Self {
            status: "status: unavailable".to_string(),
            logs: "none".to_string(),
            transcript: "".to_string(),
            transcript_entries: Vec::new(),
            tasks: "none".to_string(),
            queue: "none".to_string(),
            proof: "proof: unavailable".to_string(),
            workspace: "workspace: unavailable".to_string(),
            tools: "tools: unavailable".to_string(),
        }
    }
}

pub fn load(conn: &Connection, data_dir: &Path) -> Result<TuiSnapshot, String> {
    let status = crate::status::status(conn)?;
    let transcript_entries = transcript_entries(conn, 40)?;
    Ok(TuiSnapshot {
        logs: crate::log_view::log(conn, 12)?,
        transcript: transcript_text(&transcript_entries),
        transcript_entries,
        tasks: crate::inspect::matter_list(conn)?,
        queue: crate::inspect::queue_list(conn)?,
        proof: proof(conn)?,
        workspace: crate::diagnostics::workspace(conn, data_dir, false)?,
        tools: tools(conn)?,
        status,
    })
}

pub fn transcript(conn: &Connection, limit: usize) -> Result<String, String> {
    let entries = transcript_entries(conn, limit)?;
    Ok(transcript_text(&entries))
}

fn transcript_entries(conn: &Connection, limit: usize) -> Result<Vec<TranscriptEntry>, String> {
    let mut statement = conn
        .prepare(
            "SELECT entry_id, source, content, source_path FROM (
                 SELECT created_at AS moment, COALESCE(task_id, id) AS matter_order,
                        0 AS source_order, id AS row_id,
                        'queue:' || id AS entry_id, 'owner' AS source, content,
                        'sqlite:queue:' || id AS source_path FROM queue
                 UNION ALL
                 SELECT created_at AS moment, COALESCE(task_id, 0) AS matter_order,
                        1 AS source_order, id AS row_id,
                        'event:' || id AS entry_id, kind AS source, content,
                        'sqlite:events:' || id AS source_path FROM events
                        WHERE kind IN ('taskclosed', 'taskblocked', 'question')
             ) ORDER BY moment DESC, matter_order DESC, source_order DESC,
                      row_id DESC LIMIT ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([limit as i64], |row| {
            let source = row.get::<_, String>(1)?;
            Ok(TranscriptEntry {
                id: row.get(0)?,
                source: transcript_source(&source),
                text: row.get::<_, String>(2)?.trim().to_string(),
                path: Some(row.get(3)?),
            })
        })
        .map_err(|error| error.to_string())?;
    let mut entries = rows
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| error.to_string())?;
    entries.reverse();
    Ok(entries)
}

fn transcript_text(entries: &[TranscriptEntry]) -> String {
    entries
        .iter()
        .map(|entry| {
            format!(
                "{}: {}",
                crate::tui_types::source_label(entry.source),
                entry.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn transcript_source(kind: &str) -> TranscriptSource {
    match kind {
        "owner" | "answer" => TranscriptSource::Owner,
        "stepdone" | "taskclosed" | "question" => TranscriptSource::Agent,
        "stepblocked" | "taskblocked" => TranscriptSource::Error,
        _ => TranscriptSource::State,
    }
}

fn proof(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "prompt_frames={} checks={} artifacts={} exchanges={}",
        count(conn, "prompt_frames")?,
        count(conn, "check_results")?,
        count(conn, "artifacts")?,
        count(conn, "provider_exchanges")?
    ))
}

fn tools(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "admissions={} rejected={} stale_edges={} decisions={} observations={} latest={}",
        count(conn, "tool_admissions")?,
        count_sql(
            conn,
            "SELECT COUNT(*) FROM tool_admissions WHERE status = 'Rejected'"
        )?,
        count_sql(
            conn,
            "SELECT COUNT(*) FROM state_edges WHERE status = 'Suppressed'"
        )?,
        count(conn, "runtime_decisions")?,
        count(conn, "observations")?,
        latest_decision(conn)?
    ))
}

fn latest_decision(conn: &Connection) -> Result<String, String> {
    let row = conn.query_row(
        "SELECT id, operation_key, status FROM runtime_decisions
         ORDER BY selected_at DESC, id DESC LIMIT 1",
        [],
        |row| {
            Ok(format!(
                "{} {} {}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?
            ))
        },
    );
    match row {
        Ok(line) => Ok(line),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("none".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn count(conn: &Connection, table: &str) -> Result<i64, String> {
    count_sql(conn, &format!("SELECT COUNT(*) FROM {table}"))
}

fn count_sql(conn: &Connection, sql: &str) -> Result<i64, String> {
    conn.query_row(sql, [], |row| row.get(0))
        .map_err(|error| error.to_string())
}
