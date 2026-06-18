use std::path::Path;

use super::style;
use crate::error::CliError;
use crate::store::open_store;

const RECENT_EVENTS: usize = 7;
const WIDTH: usize = 78;

pub struct ConsoleScreen {
    pub body: String,
    pub prompt: String,
}

pub fn render_screen(data_dir: &Path, notice: &str) -> Result<ConsoleScreen, CliError> {
    let conn = open_store(data_dir)?;
    let queue = lkjagent_store::queue::list(&conn)?;
    let pending = queue
        .iter()
        .filter(|row| row.status == "pending")
        .collect::<Vec<_>>();
    let events = lkjagent_store::events::read_events(&conn)?;
    let state = state_value(&conn, "daemon state", "stopped")?;
    let question = state_value(&conn, "daemon question", "none")?;
    let mut lines = Vec::new();
    lines.push(style::muted(&rule()));
    lines.push(format!(
        "{} {} pending {} {} turns {}",
        style::title("lkjagent console"),
        style::state_badge(&state),
        pending.len(),
        style::muted("|"),
        state_value(&conn, "turn", "0")?
    ));
    lines.push(style::muted(&rule()));
    lines.extend(rows("notice", notice));
    lines.extend(rows("task", &state_value(&conn, "open task", "none")?));
    if question != "none" {
        lines.extend(rows("question", &question));
    }
    let error = state_value(&conn, "daemon error", "none")?;
    if error != "none" {
        lines.extend(rows("error", &error));
    }
    if let Some(output) = last_output(&events) {
        lines.push(section("last output"));
        lines.extend(wrap(&output, "  "));
    }
    lines.push(section("pending queue"));
    lines.extend(render_pending(&pending));
    lines.push(section("recent transcript"));
    lines.extend(render_events(&events));
    lines.push(style::muted(&rule()));
    lines.push(hint(&state, pending.len()));
    Ok(ConsoleScreen {
        body: lines.join("\n"),
        prompt: style::prompt(&state),
    })
}

fn render_pending(rows: &[&lkjagent_store::queue::QueueRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["  none".to_string()];
    }
    rows.iter()
        .take(5)
        .map(|row| format!("  #{} {}", row.id, preview(&row.content, 68)))
        .collect()
}

fn render_events(events: &[lkjagent_store::events::EventRow]) -> Vec<String> {
    if events.is_empty() {
        return vec!["  none".to_string()];
    }
    events
        .iter()
        .rev()
        .take(RECENT_EVENTS)
        .rev()
        .map(|event| {
            format!(
                "  #{} {:<14} t={:<4} {}",
                event.id,
                event.kind,
                event
                    .turn
                    .map_or_else(|| "null".to_string(), |turn| turn.to_string()),
                preview(&event.content, 48)
            )
        })
        .collect()
}

fn last_output(events: &[lkjagent_store::events::EventRow]) -> Option<String> {
    events
        .iter()
        .rev()
        .filter(|event| event.kind == "observation")
        .filter_map(|event| observation_content(&event.content))
        .find(|text| !text.starts_with("waiting"))
}

fn observation_content(content: &str) -> Option<String> {
    let text = between(content, "<content>", "</content>")?.trim();
    let useful = text
        .lines()
        .filter(str_has_signal)
        .collect::<Vec<_>>()
        .join(" ");
    if useful.is_empty() {
        None
    } else {
        Some(useful)
    }
}

fn str_has_signal(line: &&str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && !trimmed.starts_with("exit_code=")
}

fn between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let after = text.split_once(start)?.1;
    Some(after.split_once(end)?.0)
}

fn state_value(conn: &rusqlite::Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

fn rows(label: &str, value: &str) -> Vec<String> {
    wrap(value, "")
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            if index == 0 {
                format!("{label:<8} {line}")
            } else {
                format!("{:<8} {line}", "")
            }
        })
        .collect()
}

fn rule() -> String {
    "=".repeat(WIDTH)
}

fn section(label: &str) -> String {
    style::muted(&format!("-- {label}"))
}

fn hint(state: &str, pending: usize) -> String {
    match (state, pending) {
        ("waiting", 0) => "type a message to send guidance | /refresh /help /quit".to_string(),
        ("waiting", _) => "sent guidance is pending | /refresh /help /quit".to_string(),
        (_, 0) => "type a message to queue work | /refresh /help /quit".to_string(),
        _ => "queued work is pending | /refresh /help /quit".to_string(),
    }
}

fn wrap(text: &str, prefix: &str) -> Vec<String> {
    let available = WIDTH.saturating_sub(prefix.len());
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + word.len() + 1 > available {
            lines.push(format!("{prefix}{current}"));
            current.clear();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if current.is_empty() {
        lines.push(format!("{prefix}none"));
    } else {
        lines.push(format!("{prefix}{current}"));
    }
    lines
}

fn preview(text: &str, limit: usize) -> String {
    text.replace('\n', " ").chars().take(limit).collect()
}
