use std::path::Path;

pub use super::size::ScreenSize;
use super::{display, event_view, style};
use crate::error::CliError;
use crate::status_deck;
use crate::status_deck::StatusDeck;
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
    let deck = status_deck::load(data_dir, &conn)?;
    let bottom = bottom_deck(&deck, notice, size.columns);
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
        prompt: style::prompt(&deck.daemon_state),
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

fn bottom_deck(deck: &StatusDeck, notice: &str, width: usize) -> Vec<String> {
    let mut lines = vec![style::muted(&rule(width))];
    let mut state_line = format!(
        "state {} | pending {} | task {} | turns {}",
        deck.state_label, deck.pending, deck.open_task, deck.turns
    );
    if deck.active_states != "none" {
        state_line.push_str(&format!(" | states {}", deck.active_states));
    }
    lines.extend(wrap_limited(&state_line, width, 2));
    lines.extend(wrap_limited(&deck.accounting.context_line, width, 1));
    lines.extend(wrap_limited(&deck.accounting.token_line, width, 1));
    lines.extend(wrap_limited(&deck.accounting.prefix_line, width, 1));
    lines.extend(wrap_limited(
        &format!("model_log {}", deck.model_log),
        width,
        1,
    ));
    lines.extend(optional_value("question", &deck.question, width, 2));
    lines.extend(optional_value("error", &deck.error, width, 1));
    lines.extend(optional_value("next", &deck.next_action, width, 1));
    lines.extend(wrap_limited(&format!("notice {notice}"), width, 1));
    lines.extend(wrap_limited(
        &hint(&deck.daemon_state, deck.pending),
        width,
        1,
    ));
    lines
}

fn optional_value(label: &str, value: &str, width: usize, max_lines: usize) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        wrap_limited(&format!("{label} {value}"), width, max_lines)
    }
}

fn wrap_limited(text: &str, width: usize, max_lines: usize) -> Vec<String> {
    let mut lines = display::wrap(text, "", width);
    lines.truncate(max_lines);
    lines
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
