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
            let now = crate::clock::utc_now();
            let id = lkjagent_store::plan_access::enqueue_with_force(&conn, &text, force_new, &now)
                .map_err(|error| error.to_string())?;
            Ok(format!("queue: {id} new={force_new}"))
        }
        Command::Status => status(&conn),
        Command::Console => crate::console::run(&conn),
        Command::Doctor { json } => crate::diagnostics::doctor(&conn, &invocation.data_dir, json),
        Command::Workspace { json } => {
            crate::diagnostics::workspace(&conn, &invocation.data_dir, json)
        }
        Command::Log { limit, follow } if follow => crate::inspect::follow_log(&conn, limit),
        Command::Log { limit, .. } => crate::inspect::log(&conn, limit),
        Command::TaskList => crate::inspect::task_list(&conn),
        Command::TaskShow { id } => crate::inspect::task_show(&conn, id),
        Command::QueueList => crate::inspect::queue_list(&conn),
        Command::QueueShow { id } => crate::inspect::queue_show(&conn, id),
        Command::ContextResolve {
            case_id,
            semantic_key,
            winning_item_id,
        } => crate::context_admin::resolve_conflict(
            &conn,
            &case_id,
            &semantic_key,
            &winning_item_id,
            &crate::clock::utc_now(),
        ),
        Command::RecordAdd { kind, title, body } => crate::record_files::add(
            &conn,
            &invocation.data_dir,
            &kind,
            &title,
            &body,
            &crate::clock::utc_now(),
        ),
        Command::RecordList { kind } => crate::record_files::list(&conn, kind.as_deref()),
        Command::RecordShow { id } => crate::record_files::show(&conn, &invocation.data_dir, &id),
        Command::RecordLink { id, target } => crate::record_files::link(
            &conn,
            &invocation.data_dir,
            &id,
            &target,
            &crate::clock::utc_now(),
        ),
        Command::RecordArchive { id } => {
            crate::record_files::archive(&conn, &invocation.data_dir, &id, &crate::clock::utc_now())
        }
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
        "  console",
        "  doctor [--json]",
        "  workspace [--json]",
        "  log [--limit N] [--follow]",
        "  task list | task show ID",
        "  queue list | queue show ID",
        "  context resolve CASE_ID KEY WINNING_ITEM_ID",
        "  record add KIND TITLE [--body TEXT] | list [KIND] | show ID | link ID REF | archive ID",
        "  today|journal|todo|calendar|finance|project|dev TEXT",
        "  memory QUERY",
        "  watch",
        "  help",
    ]
    .join("\n")
}
