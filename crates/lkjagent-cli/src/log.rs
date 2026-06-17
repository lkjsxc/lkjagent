use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::error::CliError;
use crate::store::open_store;

pub fn log(data_dir: &Path, follow: bool, full: bool) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    if follow {
        thread::sleep(Duration::from_millis(250));
    }
    render_events(&conn, full)
}

fn render_events(conn: &rusqlite::Connection, full: bool) -> Result<String, CliError> {
    let events = lkjagent_store::events::read_events(conn)?;
    let lines = events
        .iter()
        .map(|event| {
            if full {
                format!(
                    "id={} kind={} turn={} tokens={} created_at={}\n{}",
                    event.id,
                    event.kind,
                    turn(event.turn),
                    event.tokens,
                    event.created_at,
                    event.content
                )
            } else {
                format!(
                    "id={} kind={} turn={} preview={}",
                    event.id,
                    event.kind,
                    turn(event.turn),
                    preview(&event.content)
                )
            }
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

fn turn(turn: Option<i64>) -> String {
    turn.map_or_else(|| "null".to_string(), |value| value.to_string())
}

fn preview(content: &str) -> String {
    content.chars().take(80).collect()
}
