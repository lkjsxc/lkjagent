use std::path::Path;

use lkjagent_context::budget::{prefix_cap_total, ContextPressure};

use crate::accounting;
use crate::config::load_context_policy_for_status;
use crate::error::CliError;
use crate::store::open_store;

pub fn status(data_dir: &Path) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let rows = lkjagent_store::queue::list(&conn)?;
    let queue_depth = rows.iter().filter(|row| row.status == "pending").count();
    let daemon_state = state_value(&conn, "daemon state", "stopped")?;
    let open_task = state_value(&conn, "open task", "none")?;
    let daemon_question = state_value(&conn, "daemon question", "none")?;
    let daemon_error = state_value(&conn, "daemon error", "none")?;
    let turns = state_value(&conn, "turn", "0")?;
    let last_compaction = last_compaction(&conn)?;
    let active_states = active_states(&conn)?;
    let gpt_log = lkjagent_runtime::gpt_log::current_log_path(data_dir);
    let policy = load_context_policy_for_status(data_dir)?;
    let accounting = accounting::deck(&conn, policy)?;
    let used = state_value(&conn, "context used tokens", "0")?;
    let used_tokens: usize = used.parse::<usize>().unwrap_or_default();
    let pressure = state_value(
        &conn,
        "context pressure",
        pressure_name(policy.pressure(used_tokens, 0)),
    )?;
    Ok(format!(
        "{}\n{}\n{}\ndaemon_state={daemon_state}\nqueue_depth={queue_depth}\nopen_task={open_task}\ndaemon_question={daemon_question}\ndaemon_error={daemon_error}\nturns={turns}\nactive_states={active_states}\ngpt_log={}\ncontext_window={}\ncontext_reserve={}\ncontext_used_tokens={used}\ncontext_prefix_cap={}\ncontext_log_space={}\ncontext_soft_trigger={}\ncontext_hard_trigger={}\ncontext_post_compaction_target={}\ncontext_pressure={pressure}\ncontext_compaction_trigger={}\nlast_compaction={last_compaction}",
        accounting.context_line,
        accounting.token_line,
        accounting.prefix_line,
        gpt_log.to_string_lossy(),
        policy.window,
        policy.reserve,
        prefix_cap_total(),
        policy.available_log_space(),
        policy.soft_trigger,
        policy.hard_trigger,
        policy.post_compaction_target,
        policy.hard_trigger
    ))
}

fn active_states(conn: &rusqlite::Connection) -> Result<String, CliError> {
    let Some(case) = lkjagent_store::graph::active_case(conn)? else {
        return Ok("none".to_string());
    };
    let rows = lkjagent_store::graph::state_tracks::state_tracks_for_case(conn, case.id)?;
    if rows.is_empty() {
        return Ok("none".to_string());
    }
    Ok(rows
        .iter()
        .take(3)
        .enumerate()
        .map(|(index, row)| {
            lkjagent_runtime::graph_state_tracks::format_state_track_row(index + 1, row)
        })
        .collect::<Vec<_>>()
        .join("; "))
}

fn state_value(conn: &rusqlite::Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

fn last_compaction(conn: &rusqlite::Connection) -> Result<String, CliError> {
    let events = lkjagent_store::events::read_events(conn)?;
    Ok(events
        .iter()
        .rev()
        .find(|event| event.kind == "compaction")
        .map_or_else(|| "none".to_string(), |event| event.created_at.clone()))
}

fn pressure_name(pressure: ContextPressure) -> &'static str {
    match pressure {
        ContextPressure::Green => "green",
        ContextPressure::Yellow => "yellow",
        ContextPressure::Orange => "orange",
        ContextPressure::Red => "red",
        ContextPressure::BlackInvalid => "black-invalid",
    }
}
