use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use rusqlite::Connection;

use crate::workbench_state::WorkbenchMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkbenchBackend {
    Line,
    Ratatui,
}

pub fn run(conn: &Connection, data_dir: &Path, mode: WorkbenchMode) -> Result<String, String> {
    match backend_for(mode, crate::tui_terminal::can_run_tty()) {
        WorkbenchBackend::Ratatui => {
            crate::tui_terminal::run(conn, data_dir)?;
            Ok(String::new())
        }
        WorkbenchBackend::Line => crate::workbench_line::run(conn, mode),
    }
}

fn backend_for(mode: WorkbenchMode, tty: bool) -> WorkbenchBackend {
    match (mode, tty) {
        (WorkbenchMode::Pane, true) => WorkbenchBackend::Ratatui,
        _ => WorkbenchBackend::Line,
    }
}

pub fn run_with_input<W>(
    conn: &Connection,
    input: Receiver<String>,
    output: &mut W,
    mode: WorkbenchMode,
    refresh_every: Duration,
) -> Result<(), String>
where
    W: std::io::Write,
{
    crate::workbench_line::run_with_input(conn, input, output, mode, refresh_every)
}

pub fn render_once(conn: &Connection, mode: WorkbenchMode) -> Result<String, String> {
    crate::workbench_line::render_once(conn, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_mode_uses_line_backend_even_on_tty() {
        assert_eq!(
            backend_for(WorkbenchMode::Append, true),
            WorkbenchBackend::Line
        );
        assert_eq!(
            backend_for(WorkbenchMode::Append, false),
            WorkbenchBackend::Line
        );
    }

    #[test]
    fn pane_mode_uses_ratatui_only_on_tty() {
        assert_eq!(
            backend_for(WorkbenchMode::Pane, true),
            WorkbenchBackend::Ratatui
        );
        assert_eq!(
            backend_for(WorkbenchMode::Pane, false),
            WorkbenchBackend::Line
        );
    }
}
