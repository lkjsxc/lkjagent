use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use rusqlite::Connection;

use crate::workbench_state::{reduce, UiEvent, UiState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkbenchBackend {
    Line,
    Ratatui,
}

pub fn run(conn: &Connection, data_dir: &Path) -> Result<String, String> {
    match backend_for(crate::tui_terminal::can_run_tty()) {
        WorkbenchBackend::Ratatui => {
            crate::tui_terminal::run(conn, data_dir)?;
            Ok(String::new())
        }
        WorkbenchBackend::Line => crate::workbench_line::run(conn),
    }
}

fn backend_for(tty: bool) -> WorkbenchBackend {
    if tty {
        WorkbenchBackend::Ratatui
    } else {
        WorkbenchBackend::Line
    }
}

pub fn run_with_input<W>(
    conn: &Connection,
    input: Receiver<String>,
    output: &mut W,
    refresh_every: Duration,
) -> Result<(), String>
where
    W: std::io::Write,
{
    crate::workbench_line::run_with_input(conn, input, output, refresh_every)
}

pub fn render_once(conn: &Connection) -> Result<String, String> {
    crate::workbench_line::render_once(conn)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkbenchCommand {
    Scroll(isize),
    Top,
    Follow(bool),
    Search(String),
}

pub fn apply(state: &mut UiState, command: WorkbenchCommand) -> String {
    match command {
        WorkbenchCommand::Scroll(delta) => {
            *state = reduce(state.clone(), UiEvent::Scroll(delta));
            format!("workbench: scroll={}", state.scroll)
        }
        WorkbenchCommand::Top => {
            *state = reduce(state.clone(), UiEvent::Top);
            "workbench: scroll=0".to_string()
        }
        WorkbenchCommand::Follow(enabled) => {
            *state = reduce(state.clone(), UiEvent::Follow(enabled));
            format!("workbench: follow={enabled}")
        }
        WorkbenchCommand::Search(query) => {
            *state = reduce(state.clone(), UiEvent::Search(query.clone()));
            format!("workbench: search={query}")
        }
    }
}

pub fn parse(line: &str) -> Result<Option<WorkbenchCommand>, String> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("/scroll") {
        return scroll(rest.trim()).map(Some);
    }
    if let Some(rest) = trimmed.strip_prefix("/page") {
        return page(rest.trim()).map(Some);
    }
    if let Some(rest) = trimmed.strip_prefix("/follow") {
        return follow(rest.trim()).map(Some);
    }
    if let Some(rest) = trimmed.strip_prefix("/search") {
        return search(rest.trim()).map(Some);
    }
    Ok(None)
}

fn scroll(value: &str) -> Result<WorkbenchCommand, String> {
    match value {
        "up" => Ok(WorkbenchCommand::Scroll(-1)),
        "down" => Ok(WorkbenchCommand::Scroll(1)),
        "top" => Ok(WorkbenchCommand::Top),
        _ => Err("/scroll requires up, down, or top".to_string()),
    }
}
fn follow(value: &str) -> Result<WorkbenchCommand, String> {
    match value {
        "on" => Ok(WorkbenchCommand::Follow(true)),
        "off" => Ok(WorkbenchCommand::Follow(false)),
        _ => Err("/follow requires on or off".to_string()),
    }
}
fn search(value: &str) -> Result<WorkbenchCommand, String> {
    if value.is_empty() {
        Err("/search requires text".to_string())
    } else {
        Ok(WorkbenchCommand::Search(value.to_string()))
    }
}
fn page(value: &str) -> Result<WorkbenchCommand, String> {
    match value {
        "up" => Ok(WorkbenchCommand::Scroll(-10)),
        "down" => Ok(WorkbenchCommand::Scroll(10)),
        _ => Err("/page requires up or down".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_workbench_commands() {
        assert_eq!(parse("/scroll down"), Ok(Some(WorkbenchCommand::Scroll(1))));
        assert_eq!(parse("/page up"), Ok(Some(WorkbenchCommand::Scroll(-10))));
        assert_eq!(
            parse("/follow off"),
            Ok(Some(WorkbenchCommand::Follow(false)))
        );
        assert_eq!(
            parse("/search daemon"),
            Ok(Some(WorkbenchCommand::Search("daemon".to_string())))
        );
        assert_eq!(parse("hello"), Ok(None));
    }

    #[test]
    fn tty_uses_ratatui_and_non_tty_uses_line_backend() {
        assert_eq!(backend_for(true), WorkbenchBackend::Ratatui);
        assert_eq!(backend_for(false), WorkbenchBackend::Line);
    }
}
