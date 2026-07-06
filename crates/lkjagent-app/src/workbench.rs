use std::io::{BufRead, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use rusqlite::Connection;

pub fn run(conn: &Connection) -> Result<String, String> {
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
    run_with_input(conn, receiver, &mut stdout, Duration::from_secs(2))?;
    Ok(String::new())
}

pub fn run_with_input<W>(
    conn: &Connection,
    input: Receiver<String>,
    output: &mut W,
    refresh_every: Duration,
) -> Result<(), String>
where
    W: Write,
{
    writeln!(
        output,
        "lkjagent workbench: type text, /status, /watch, /log, /quit"
    )
    .map_err(|error| error.to_string())?;
    loop {
        writeln!(output, "{}", render_once(conn)?)
            .and_then(|_| output.flush())
            .map_err(|error| error.to_string())?;
        match input.recv_timeout(refresh_every) {
            Ok(line) => {
                if handle_line(conn, output, &line)? {
                    break;
                }
                while let Ok(next) = input.try_recv() {
                    if handle_line(conn, output, &next)? {
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

pub fn render_once(conn: &Connection) -> Result<String, String> {
    Ok(format!(
        "== workbench refresh ==\n{}\ninput: plain text enqueues; /quit exits workbench",
        crate::inspect::watch(conn)?
    ))
}

fn handle_line<W>(conn: &Connection, output: &mut W, line: &str) -> Result<bool, String>
where
    W: Write,
{
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

        run_with_input(&conn, receiver, &mut output, Duration::from_millis(1))?;

        let text = String::from_utf8(output)?;
        assert!(text.contains("lkjagent workbench"));
        assert!(text.contains("== workbench refresh =="));
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

        run_with_input(&conn, receiver, &mut output, Duration::from_millis(1))?;

        let text = String::from_utf8(output)?;
        assert!(text.contains("queue: 1 new=false"));
        assert!(text.contains("console: bye"));
        Ok(())
    }
}
