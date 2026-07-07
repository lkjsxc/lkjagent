use std::path::Path;

use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiSnapshot {
    pub status: String,
    pub logs: String,
    pub transcript: String,
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
    Ok(TuiSnapshot {
        logs: crate::log_view::log(conn, 12)?,
        transcript: transcript(conn, 40)?,
        tasks: crate::inspect::matter_list(conn)?,
        queue: crate::inspect::queue_list(conn)?,
        proof: proof(conn)?,
        workspace: crate::diagnostics::workspace(conn, data_dir, false)?,
        tools: tools(conn)?,
        status,
    })
}

fn transcript(conn: &Connection, limit: usize) -> Result<String, String> {
    let mut statement = conn
        .prepare(
            "SELECT source, content FROM (
                 SELECT created_at AS moment, 0 AS source_order, id AS row_id,
                        'owner' AS source, content FROM queue
                 UNION ALL
                 SELECT created_at AS moment, 1 AS source_order, id AS row_id,
                        kind AS source, content FROM events
                        WHERE kind NOT IN ('owner', 'answer')
             ) ORDER BY moment DESC, source_order DESC, row_id DESC LIMIT ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([limit as i64], |row| {
            Ok(transcript_line(
                &row.get::<_, String>(0)?,
                &row.get::<_, String>(1)?,
            ))
        })
        .map_err(|error| error.to_string())?;
    let mut lines = rows
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| error.to_string())?;
    lines.reverse();
    Ok(lines.join("\n"))
}

fn transcript_line(kind: &str, content: &str) -> String {
    format!("{}: {}", transcript_source(kind), content.trim())
}

fn transcript_source(kind: &str) -> &'static str {
    match kind {
        "owner" | "answer" => "owner",
        "stepdone" | "taskclosed" | "question" => "agent",
        "stepblocked" | "taskblocked" => "error",
        _ => "state",
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
        "admissions={} decisions={} observations={} latest={}",
        count(conn, "tool_admissions")?,
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
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_store::plan_access::enqueue_with_force;
    use lkjagent_store::plan_schema::setup;

    #[test]
    fn snapshot_reads_durable_queue_and_proof_rows() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        enqueue_with_force(&conn, "hello", false, "001")?;
        enqueue_with_force(&conn, "follow up", false, "003")?;
        conn.execute(
            "INSERT INTO events (task_id, kind, content, created_at)
             VALUES (1, 'stepdone', 'AI answered', '002'),
                    (1, 'taskclosed', 'done', '004')",
            [],
        )?;

        let snapshot = load(&conn, Path::new("data"))?;

        assert!(snapshot.status.contains("queue: 2 pending"));
        assert!(snapshot.queue.contains("queue 1"));
        assert!(snapshot.proof.contains("prompt_frames=0"));
        assert!(snapshot.workspace.contains("workspace: root="));
        assert_order(
            &snapshot.transcript,
            &[
                "owner: hello",
                "agent: AI answered",
                "owner: follow up",
                "agent: done",
            ],
        )?;
        Ok(())
    }

    fn assert_order(text: &str, expected: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let mut cursor = 0;
        for needle in expected {
            let Some(offset) = text[cursor..].find(needle) else {
                return Err(format!("missing {needle} after byte {cursor} in {text}").into());
            };
            cursor += offset + needle.len();
        }
        Ok(())
    }
}
