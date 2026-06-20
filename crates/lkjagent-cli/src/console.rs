mod display;
mod event_view;
mod input;
mod render;
mod size;
mod style;
mod terminal_input;

use std::io::{self, BufRead, IsTerminal, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use crate::error::CliError;
use crate::store::{now_stamp, open_store};
use input::{spawn_line_input, InputEvent};
use render::{render_screen, render_screen_for_size, ScreenSize};
use terminal_input::{spawn_terminal_input, TerminalMode};

const REFRESH_INTERVAL: Duration = Duration::from_millis(1000);
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const PROMPT_INPUT_COLUMNS: usize = 6;

pub fn console(data_dir: &Path) -> Result<String, CliError> {
    let mut stdout = io::stdout();
    if io::stdin().is_terminal() && stdout.is_terminal() {
        run_terminal_console(data_dir, &mut stdout)?;
    } else {
        let stdin = io::BufReader::new(io::stdin());
        run_console(data_dir, stdin, &mut stdout)?;
    }
    Ok(String::new())
}

pub fn run_console<R, W>(data_dir: &Path, reader: R, writer: &mut W) -> Result<(), CliError>
where
    R: BufRead + Send + 'static,
    W: Write,
{
    run_console_events(data_dir, spawn_line_input(reader), writer)
}

fn run_terminal_console<W>(data_dir: &Path, writer: &mut W) -> Result<(), CliError>
where
    W: Write,
{
    let _terminal = TerminalMode::activate()?;
    run_console_events(data_dir, spawn_terminal_input(), writer)
}

fn run_console_events<W>(
    data_dir: &Path,
    input: Receiver<InputEvent>,
    writer: &mut W,
) -> Result<(), CliError>
where
    W: Write,
{
    let mut typed = String::new();
    let mut notice = "ready".to_string();
    loop {
        draw(data_dir, writer, &notice, &typed)?;
        match input.recv_timeout(REFRESH_INTERVAL) {
            Ok(InputEvent::Line(line)) => match handle_input(data_dir, &line)? {
                ConsoleFlow::Continue(next) => {
                    typed.clear();
                    notice = next;
                }
                ConsoleFlow::Quit => {
                    typed.clear();
                    writeln!(writer, "\nconsole closed")?;
                    return Ok(());
                }
            },
            Ok(InputEvent::Buffer(buffer)) => typed = buffer,
            Ok(InputEvent::Eof) => return Ok(()),
            Ok(InputEvent::Error(error)) => return Err(CliError::failure(error)),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                notice = "watching daemon".to_string();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

enum ConsoleFlow {
    Continue(String),
    Quit,
}

fn draw<W>(data_dir: &Path, writer: &mut W, notice: &str, typed: &str) -> Result<(), CliError>
where
    W: Write,
{
    let screen = render_screen(data_dir, notice)?;
    let input = prompt_input(typed, screen.columns);
    write!(
        writer,
        "{CLEAR_SCREEN}{}\n{} {}",
        screen.body, screen.prompt, input
    )?;
    writer.flush()?;
    Ok(())
}

pub fn render_snapshot(
    data_dir: &Path,
    notice: &str,
    columns: usize,
    rows: usize,
) -> Result<String, CliError> {
    Ok(render_screen_for_size(data_dir, notice, ScreenSize { columns, rows })?.body)
}

fn handle_input(data_dir: &Path, input: &str) -> Result<ConsoleFlow, CliError> {
    match input {
        "" | "/refresh" => Ok(ConsoleFlow::Continue("refreshed".to_string())),
        "/help" => Ok(ConsoleFlow::Continue(
            "commands: /refresh, /help, /quit; any other line sends".to_string(),
        )),
        "/quit" | "/exit" => Ok(ConsoleFlow::Quit),
        text if text.starts_with('/') => {
            Ok(ConsoleFlow::Continue(format!("unknown command: {text}")))
        }
        text => enqueue(data_dir, text).map(ConsoleFlow::Continue),
    }
}

fn enqueue(data_dir: &Path, text: &str) -> Result<String, CliError> {
    let mut conn = open_store(data_dir)?;
    let daemon_state = state_value(&conn, "daemon state", "stopped")?;
    let id = lkjagent_store::queue::enqueue(&mut conn, text, "console-send", &now_stamp())?;
    if daemon_state == "stopped" {
        return Ok(format!("queued id={id}; daemon is not running"));
    }
    Ok(format!("queued id={id}; watching daemon"))
}

fn state_value(conn: &rusqlite::Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

fn prompt_input(typed: &str, columns: usize) -> String {
    display::truncate(typed, columns.saturating_sub(PROMPT_INPUT_COLUMNS))
}

#[cfg(test)]
mod tests {
    use super::{display, prompt_input};

    #[test]
    fn typed_prompt_input_is_truncated_to_terminal_width() {
        let typed = "abcdefghijklmnopqrstuvwxyz日本語入力の長い下書き";
        let fitted = prompt_input(typed, 40);

        assert!(display::visible_width(&fitted) <= 34);
        assert!(fitted.ends_with(".."));
    }
}
