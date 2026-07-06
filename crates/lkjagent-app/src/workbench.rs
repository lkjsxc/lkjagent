use std::io::{BufRead, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use rusqlite::Connection;

use crate::workbench_state::{reduce, UiEvent, UiState, WorkbenchMode};

pub fn run(conn: &Connection, mode: WorkbenchMode) -> Result<String, String> {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            if sender.send(line).is_err() {
                break;
            }
        }
    });
    let mut stdout = std::io::stdout();
    run_with_input(conn, receiver, &mut stdout, mode, Duration::from_secs(2))?;
    Ok(String::new())
}

pub fn run_with_input<W>(
    conn: &Connection,
    input: Receiver<String>,
    output: &mut W,
    mode: WorkbenchMode,
    refresh_every: Duration,
) -> Result<(), String>
where
    W: Write,
{
    let mut state = UiState::new(mode);
    writeln!(
        output,
        "lkjagent workbench: type text, /mode append, /mode pane, /quit"
    )
    .map_err(|error| error.to_string())?;
    loop {
        state = refresh(conn, state, output)?;
        match input.recv_timeout(refresh_every) {
            Ok(line) => {
                if handle_line(conn, output, &mut state, &line)? {
                    break;
                }
                while let Ok(next) = input.try_recv() {
                    if handle_line(conn, output, &mut state, &next)? {
                        return Ok(());
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

pub fn render_once(conn: &Connection, mode: WorkbenchMode) -> Result<String, String> {
    let state = reduce(
        UiState::new(mode),
        UiEvent::Refresh(crate::inspect::watch(conn)?),
    );
    Ok(crate::workbench_render::render(&state))
}

fn refresh<W>(conn: &Connection, state: UiState, output: &mut W) -> Result<UiState, String>
where
    W: Write,
{
    let state = reduce(state, UiEvent::Refresh(crate::inspect::watch(conn)?));
    writeln!(output, "{}", crate::workbench_render::render(&state))
        .and_then(|_| output.flush())
        .map_err(|error| error.to_string())?;
    Ok(state)
}

fn handle_line<W>(
    conn: &Connection,
    output: &mut W,
    state: &mut UiState,
    line: &str,
) -> Result<bool, String>
where
    W: Write,
{
    if let Some(command) = crate::workbench_commands::parse(line)? {
        let message = crate::workbench_commands::apply(state, command);
        writeln!(
            output,
            "{message}\n{}",
            crate::workbench_render::render(state)
        )
        .and_then(|_| output.flush())
        .map_err(|error| error.to_string())?;
        return Ok(false);
    }
    let reply = crate::console::handle_line(conn, line, &crate::clock::utc_now())?;
    if !reply.output.is_empty() {
        writeln!(output, "{}", reply.output).map_err(|error| error.to_string())?;
        output.flush().map_err(|error| error.to_string())?;
    }
    Ok(reply.quit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_store::plan_schema::setup;
    use rusqlite::Connection;

    #[test]
    fn closed_input_renders_once_and_exits() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        let (sender, receiver) = mpsc::channel();
        drop(sender);
        let mut output = Vec::new();

        run_with_input(
            &conn,
            receiver,
            &mut output,
            WorkbenchMode::Append,
            Duration::from_millis(1),
        )?;

        let text = String::from_utf8(output)?;
        assert!(text.contains("lkjagent workbench"));
        assert!(text.contains("== workbench refresh"));
        assert!(text.contains("== status =="));
        Ok(())
    }

    #[test]
    fn owner_input_uses_console_handler() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        let (sender, receiver) = mpsc::channel();
        sender.send("hello".to_string())?;
        sender.send("/quit".to_string())?;
        drop(sender);
        let mut output = Vec::new();

        run_with_input(
            &conn,
            receiver,
            &mut output,
            WorkbenchMode::Append,
            Duration::from_millis(1),
        )?;

        let text = String::from_utf8(output)?;
        assert!(text.contains("queue: 1 new=false"));
        assert!(text.contains("console: bye"));
        Ok(())
    }

    #[test]
    fn mode_command_switches_to_pane_renderer() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        let (sender, receiver) = mpsc::channel();
        sender.send("/mode pane".to_string())?;
        sender.send("/scroll down".to_string())?;
        sender.send("/follow on".to_string())?;
        sender.send("/quit".to_string())?;
        drop(sender);
        let mut output = Vec::new();

        run_with_input(
            &conn,
            receiver,
            &mut output,
            WorkbenchMode::Append,
            Duration::from_millis(1),
        )?;

        let text = String::from_utf8(output)?;
        assert!(text.contains("workbench: mode=pane"));
        assert!(text.contains("workbench: scroll=1"));
        assert!(text.contains("workbench: follow=true"));
        assert!(text.contains("== workbench pane refresh"));
        Ok(())
    }
}
