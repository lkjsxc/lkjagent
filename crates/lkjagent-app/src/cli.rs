use std::fs;

use rusqlite::Connection;

use crate::args::{parse, Command};
use crate::daemon::run_daemon;
use crate::status::status;

pub fn run<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let invocation = parse(args)?;
    if invocation.command == Command::Help {
        return Ok(help());
    }
    fs::create_dir_all(&invocation.data_dir).map_err(|error| error.to_string())?;
    let db = invocation.data_dir.join("lkjagent.sqlite3");
    let conn = Connection::open(&db).map_err(|error| error.to_string())?;
    lkjagent_store::plan_schema::setup(&conn).map_err(|error| error.to_string())?;
    match invocation.command {
        Command::Run => {
            run_daemon(&invocation.data_dir)?;
            Ok(String::new())
        }
        Command::Send { text, force_new } => {
            let id =
                lkjagent_store::plan_access::enqueue_with_force(&conn, &text, force_new, "now")
                    .map_err(|error| error.to_string())?;
            Ok(format!("queue: {id} new={force_new}"))
        }
        Command::Status => status(&conn),
        Command::Log { limit } => crate::inspect::log(&conn, limit),
        Command::TaskList => crate::inspect::task_list(&conn),
        Command::TaskShow { id } => crate::inspect::task_show(&conn, id),
        Command::QueueList => crate::inspect::queue_list(&conn),
        Command::QueueShow { id } => crate::inspect::queue_show(&conn, id),
        Command::Memory { query } => crate::inspect::memory(&conn, &query),
        Command::Watch => crate::inspect::watch(&conn),
        Command::Help => Ok(help()),
    }
}

pub fn help() -> String {
    [
        "lkjagent commands:",
        "  run",
        "  send TEXT [--new]",
        "  status",
        "  log [--limit N]",
        "  task list | task show ID",
        "  queue list | queue show ID",
        "  memory QUERY",
        "  watch",
        "  help",
    ]
    .join("\n")
}
