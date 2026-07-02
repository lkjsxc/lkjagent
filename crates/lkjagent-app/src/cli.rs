use std::fs;

use rusqlite::Connection;

use crate::args::{parse, Command};
use crate::daemon::{run_until_idle, ScriptedEndpoint};
use crate::state::load_snapshot;
use crate::status::{status, task_show, watch};

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
            drop(conn);
            let mut endpoint = ScriptedEndpoint {
                outputs: vec!["<finish>idle</finish>".to_string()],
                index: 0,
            };
            let snapshot = run_until_idle(&invocation.data_dir, &mut endpoint, 1)?;
            Ok(format!("daemon: {:?}", snapshot.task.state))
        }
        Command::Send { text, force_new } => {
            let id = lkjagent_store::plan_access::enqueue(&conn, &text, "now")
                .map_err(|error| error.to_string())?;
            Ok(format!("queue: {id} new={force_new}"))
        }
        Command::Status => status(&conn),
        Command::Log { limit } => Ok(format!("log: limit={limit}")),
        Command::TaskList => Ok("tasks: see task show ID".to_string()),
        Command::TaskShow { id } => {
            let snapshot = load_snapshot(&conn).map_err(|error| error.to_string())?;
            snapshot.map_or_else(
                || Ok(format!("task {id}: not found")),
                |snap| Ok(task_show(&snap)),
            )
        }
        Command::QueueList => Ok("queue: use queue show ID".to_string()),
        Command::QueueShow { id } => Ok(format!("queue {id}")),
        Command::Memory { query } => Ok(format!("memory: {query}")),
        Command::Watch => {
            let snapshot = load_snapshot(&conn).map_err(|error| error.to_string())?;
            snapshot.map_or_else(|| Ok("watch: idle".to_string()), |snap| Ok(watch(&snap)))
        }
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
