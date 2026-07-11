use std::fs;

use rusqlite::Connection;

use crate::args::{parse, Command};
use crate::daemon::{run_daemon, run_until_idle};
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
    crate::workspace_scaffold::ensure_root(&crate::config::workspace_root(&invocation.data_dir)?)?;
    match invocation.command {
        Command::Run { once } if once => {
            let mut endpoint = crate::endpoint::LlmEndpoint::new(&invocation.data_dir);
            let snapshot = run_until_idle(&invocation.data_dir, &mut endpoint, 1)?;
            Ok(format!(
                "run-once: matter={} state={:?}",
                snapshot.task.id, snapshot.task.state
            ))
        }
        Command::Run { .. } => {
            run_daemon(&invocation.data_dir)?;
            Ok(String::new())
        }
        Command::Send { text, force_new } => {
            let now = crate::clock::utc_now();
            let id = lkjagent_store::plan_access::enqueue_with_force(&conn, &text, force_new, &now)
                .map_err(|error| error.to_string())?;
            crate::daemon_owner_routes::write_send_trace(
                &invocation.data_dir,
                id,
                &text,
                force_new,
                &now,
            )?;
            Ok(format!("queue: {id} new={force_new}"))
        }
        Command::Status => status(&conn),
        Command::Console => crate::console::run(&conn),
        Command::Workbench => crate::workbench::run(&conn, &invocation.data_dir),
        Command::Doctor { json } => crate::diagnostics::doctor(&conn, &invocation.data_dir, json),
        Command::Workspace { json, rebuild } => {
            if rebuild {
                crate::workspace_index::rebuild(
                    &conn,
                    &invocation.data_dir,
                    &crate::clock::utc_now(),
                )?;
            }
            crate::diagnostics::workspace(&conn, &invocation.data_dir, json)
        }
        Command::WorkspaceSearch {
            query,
            kind,
            state,
            project,
            date,
            mode,
        } => crate::workspace_search::search(
            &conn,
            &crate::config::workspace_root(&invocation.data_dir)?,
            &crate::workspace_search::Request {
                query,
                kind,
                state,
                project,
                date,
                mode,
            },
        ),
        Command::WorkspacePlanRebalance { json } => crate::workspace_rebalance::plan(
            &conn,
            &invocation.data_dir,
            json,
            &crate::clock::utc_now(),
        ),
        Command::WorkspaceApplyRebalance { json } => crate::workspace_rebalance::apply(
            &conn,
            &invocation.data_dir,
            json,
            &crate::clock::utc_now(),
        ),
        Command::WorkspaceValidate { json } => crate::workspace_rebalance::validate(
            &conn,
            &invocation.data_dir,
            json,
            &crate::clock::utc_now(),
        ),
        Command::Log { limit, follow } if follow => crate::inspect::follow_log(&conn, limit),
        Command::Log { limit, .. } => crate::inspect::log(&conn, limit),
        Command::MatterList => crate::inspect::matter_list(&conn),
        Command::MatterShow { id } => crate::inspect::matter_show(&conn, id),
        Command::QueueList => crate::inspect::queue_list(&conn),
        Command::QueueShow { id } => crate::inspect::queue_show(&conn, id),
        Command::ContextResolve {
            case_id,
            semantic_key,
            winning_item_id,
        } => crate::context_resolution_bridge::resolve_conflict(
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
        Command::RecordArchive { id } => crate::record_archive::archive(
            &conn,
            &invocation.data_dir,
            &id,
            &crate::clock::utc_now(),
        ),
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
        "  workbench",
        "  doctor [--json]",
        "  workspace [--json] [--rebuild] | search QUERY [--kind KIND] [--state STATE] [--project PROJECT] [--date DATE] [--mode lexical|trigram]",
        "  workspace plan-rebalance|apply-rebalance|validate [--json]",
        "  log [--limit N] [--follow]",
        "  matter list | matter show REF",
        "  queue list | queue show ID",
        "  context resolve CASE_ID KEY WINNING_ITEM_ID",
        "  record add KIND TITLE [--body TEXT] | list [KIND] | show ID | link ID REF | archive ID",
        "  today|journal|todo|calendar|finance|note|project|artifact|dev TEXT",
        "  memory QUERY",
        "  watch",
        "  help",
    ]
    .join("\n")
}
