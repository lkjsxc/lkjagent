use std::path::Path;

use lkjagent_context::budget::{prefix_cap_total, ContextPressure};

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
    let policy = load_context_policy_for_status(data_dir)?;
    let used = state_value(&conn, "context used tokens", "0")?;
    let used_tokens: usize = used.parse::<usize>().unwrap_or_default();
    let pressure = state_value(
        &conn,
        "context pressure",
        pressure_name(policy.pressure(used_tokens, 0)),
    )?;
    Ok(format!(
        "daemon_state={daemon_state}\nqueue_depth={queue_depth}\nopen_task={open_task}\ndaemon_question={daemon_question}\ndaemon_error={daemon_error}\nturns={turns}\ncontext_window={}\ncontext_reserve={}\ncontext_used_tokens={used}\ncontext_prefix_cap={}\ncontext_log_space={}\ncontext_soft_trigger={}\ncontext_hard_trigger={}\ncontext_post_compaction_target={}\ncontext_pressure={pressure}\ncontext_compaction_trigger={}\nlast_compaction={last_compaction}",
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
