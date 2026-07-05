use std::io::BufRead;

use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleReply {
    pub output: String,
    pub quit: bool,
}

pub fn run(conn: &Connection) -> Result<String, String> {
    let stdin = std::io::stdin();
    let mut lines = Vec::new();
    lines
        .push("lkjagent console: type text to send, /status, /watch, /new TEXT, /quit".to_string());
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        let reply = handle_line(conn, &line, &crate::clock::utc_now())?;
        if !reply.output.is_empty() {
            lines.push(reply.output);
        }
        if reply.quit {
            break;
        }
    }
    Ok(lines.join("\n"))
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
        Some((other, _)) => reply(&format!("console: unknown command {other}"), false),
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
    reply(&format!("queue: {id} new={force_new}"), false)
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
