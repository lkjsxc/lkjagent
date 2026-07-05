use std::io::{BufRead, Write};

use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleReply {
    pub output: String,
    pub quit: bool,
}

pub fn run(conn: &Connection) -> Result<String, String> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    run_with_io(conn, stdin.lock(), &mut stdout)?;
    Ok(String::new())
}

pub fn run_with_io<R, W>(conn: &Connection, input: R, output: &mut W) -> Result<(), String>
where
    R: BufRead,
    W: Write,
{
    writeln!(
        output,
        "lkjagent console: type text to send, /status, /watch, /new TEXT, /quit"
    )
    .map_err(|error| error.to_string())?;
    output.flush().map_err(|error| error.to_string())?;
    for line in input.lines() {
        let line = line.map_err(|error| error.to_string())?;
        let reply = handle_line(conn, &line, &crate::clock::utc_now())?;
        if !reply.output.is_empty() {
            writeln!(output, "{}", reply.output).map_err(|error| error.to_string())?;
            output.flush().map_err(|error| error.to_string())?;
        }
        if reply.quit {
            break;
        }
    }
    Ok(())
}

pub fn handle_line(conn: &Connection, line: &str, now: &str) -> Result<ConsoleReply, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return reply("", false);
    }
    match command(trimmed) {
        Some(("/quit", _)) | Some(("/exit", _)) => reply("console: bye", true),
        Some(("/status", _)) => reply(crate::status::status(conn)?, false),
        Some(("/watch", _)) => reply(crate::inspect::watch(conn)?, false),
        Some(("/log", _)) => reply(crate::inspect::log(conn, 20)?, false),
        Some(("/queue", _)) => reply(crate::inspect::queue_list(conn)?, false),
        Some(("/task", _)) => reply(crate::inspect::task_list(conn)?, false),
        Some(("/new", text)) => enqueue(conn, text, true, now),
        Some(("/send", text)) => enqueue(conn, text, false, now),
        Some((other, _)) => reply(format!("console: unknown command {other}"), false),
        None => enqueue(conn, trimmed, false, now),
    }
}

fn enqueue(
    conn: &Connection,
    text: &str,
    force_new: bool,
    now: &str,
) -> Result<ConsoleReply, String> {
    if text.trim().is_empty() {
        return reply("console: message text required", false);
    }
    let id = lkjagent_store::plan_access::enqueue_with_force(conn, text, force_new, now)
        .map_err(|error| error.to_string())?;
    reply(format!("queue: {id} new={force_new}"), false)
}

fn command(line: &str) -> Option<(&str, &str)> {
    if !line.starts_with('/') {
        return None;
    }
    match line.split_once(' ') {
        Some((name, rest)) => Some((name, rest.trim())),
        None => Some((line, "")),
    }
}

fn reply(output: impl Into<String>, quit: bool) -> Result<ConsoleReply, String> {
    Ok(ConsoleReply {
        output: output.into(),
        quit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_store::plan_schema::setup;
    use rusqlite::Connection;
    use std::io::{Cursor, Write};

    #[test]
    fn run_with_io_writes_and_flushes_each_reply() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        setup(&conn)?;
        let input = Cursor::new(b"hello\n/quit\n");
        let mut output = FlushOutput::default();

        run_with_io(&conn, input, &mut output)?;

        let text = String::from_utf8(output.bytes)?;
        assert!(text.starts_with("lkjagent console:"));
        assert!(text.contains("queue: 1 new=false\n"));
        assert!(text.ends_with("console: bye\n"));
        assert_eq!(output.flushes, 3);
        Ok(())
    }

    #[derive(Default)]
    struct FlushOutput {
        bytes: Vec<u8>,
        flushes: usize,
    }

    impl Write for FlushOutput {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }
}
