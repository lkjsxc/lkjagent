use std::path::Path;

pub use super::size::ScreenSize;
use super::{display, event_view, style};
use crate::error::CliError;
use crate::store::open_store;

pub struct ConsoleScreen {
    pub body: String,
    pub prompt: String,
    pub columns: usize,
}

pub fn render_screen(data_dir: &Path, notice: &str) -> Result<ConsoleScreen, CliError> {
    render_screen_for_size(data_dir, notice, ScreenSize::current())
}

pub fn render_screen_for_size(
    data_dir: &Path,
    notice: &str,
    size: ScreenSize,
) -> Result<ConsoleScreen, CliError> {
    let size = size.clamp();
    let conn = open_store(data_dir)?;
    let queue = lkjagent_store::queue::list(&conn)?;
    let pending = queue
        .iter()
        .filter(|row| row.status == "pending")
        .collect::<Vec<_>>();
    let events = lkjagent_store::events::read_events(&conn)?;
    let state = state_value(&conn, "daemon state", "stopped")?;
    let bottom = bottom_deck(&conn, &state, notice, pending.len(), size.columns)?;
    let body_limit = size.rows.saturating_sub(1);
    let body_budget = body_limit.saturating_sub(bottom.len());
    let mut lines = top_pane(&events, &pending, body_budget, size.columns);
    pad_to(&mut lines, body_budget);
    lines.extend(bottom);
    if lines.len() > body_limit {
        lines = tail(lines, body_limit);
    } else {
        pad_to(&mut lines, body_limit);
    }
    Ok(ConsoleScreen {
        body: lines.join("\n"),
        prompt: style::prompt(&state),
        columns: size.columns,
    })
}

fn top_pane(
    events: &[lkjagent_store::events::EventRow],
    pending: &[&lkjagent_store::queue::QueueRow],
    budget: usize,
    width: usize,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(style::muted(&display::truncate(
        "transcript",
        width.saturating_sub(1),
    )));
    if let Some(output) = event_view::last_output(events) {
        lines.extend(display::wrap(&format!("last: {output}"), "", width));
    }
    lines.extend(event_view::top_lines(events, pending, width));
    tail(lines, budget)
}

fn bottom_deck(
    conn: &rusqlite::Connection,
    state: &str,
    notice: &str,
    pending: usize,
    width: usize,
) -> Result<Vec<String>, CliError> {
    let mut lines = vec![style::muted(&rule(width))];
    lines.extend(wrap_limited(
        &format!(
            "state {} | pending {pending} | task {} | turns {}",
            state_label(state),
            state_value(conn, "open task", "none")?,
            state_value(conn, "turn", "0")?
        ),
        width,
        2,
    ));
    lines.extend(optional_row(conn, "question", "daemon question", width, 2)?);
    lines.extend(optional_row(conn, "error", "daemon error", width, 1)?);
    lines.extend(wrap_limited(&format!("notice {notice}"), width, 1));
    lines.extend(wrap_limited(&hint(state, pending), width, 1));
    Ok(lines)
}

fn optional_row(
    conn: &rusqlite::Connection,
    label: &str,
    key: &str,
    width: usize,
    max_lines: usize,
) -> Result<Vec<String>, CliError> {
    let value = state_value(conn, key, "none")?;
    if value == "none" {
        return Ok(Vec::new());
    }
    Ok(wrap_limited(&format!("{label} {value}"), width, max_lines))
}

fn wrap_limited(text: &str, width: usize, max_lines: usize) -> Vec<String> {
    let mut lines = display::wrap(text, "", width);
    lines.truncate(max_lines);
    lines
}

fn state_value(conn: &rusqlite::Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

fn tail(lines: Vec<String>, budget: usize) -> Vec<String> {
    let skip = lines.len().saturating_sub(budget);
    lines.into_iter().skip(skip).collect()
}

fn pad_to(lines: &mut Vec<String>, target: usize) {
    while lines.len() < target {
        lines.push(String::new());
    }
}

fn rule(width: usize) -> String {
    "-".repeat(width)
}

fn hint(state: &str, pending: usize) -> String {
    match (state, pending) {
        ("waiting", 0) => "send guidance | /refresh /help /quit".to_string(),
        ("waiting", _) => "guidance queued | /refresh /help /quit".to_string(),
        (_, 0) => "send work | /refresh /help /quit".to_string(),
        _ => "queued work pending | /refresh /help /quit".to_string(),
    }
}

fn state_label(state: &str) -> String {
    match state {
        "idle" => "IDLE",
        "working" => "WORKING",
        "waiting" => "WAITING",
        "error" => "ERROR",
        _ => "STOPPED",
    }
    .to_string()
}
