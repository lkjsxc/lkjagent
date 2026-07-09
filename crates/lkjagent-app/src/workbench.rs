use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use rusqlite::Connection;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tty_uses_ratatui_and_non_tty_uses_line_backend() {
        assert_eq!(backend_for(true), WorkbenchBackend::Ratatui);
        assert_eq!(backend_for(false), WorkbenchBackend::Line);
    }
}
