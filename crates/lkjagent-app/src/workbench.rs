use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use rusqlite::Connection;

use crate::workbench_state::WorkbenchMode;

pub fn run(conn: &Connection, data_dir: &Path, mode: WorkbenchMode) -> Result<String, String> {
    if crate::tui_terminal::can_run_tty() {
        crate::tui_terminal::run(conn, data_dir)?;
        Ok(String::new())
    } else {
        crate::workbench_line::run(conn, mode)
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
