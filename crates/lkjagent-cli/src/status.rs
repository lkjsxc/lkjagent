use std::path::Path;

use lkjagent_context::budget::{initial_log_space, prefix_cap_total, WHOLE_WINDOW_TRIGGER};

use crate::error::CliError;
use crate::store::open_store;

pub fn status(data_dir: &Path) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let rows = lkjagent_store::queue::list(&conn)?;
    let queue_depth = rows.iter().filter(|row| row.status == "pending").count();
    let daemon_state = state_value(&conn, "daemon state", "stopped")?;
    let open_task = state_value(&conn, "open task", "none")?;
    let turns = state_value(&conn, "turn", "0")?;
    let last_compaction = last_compaction(&conn)?;
    Ok(format!(
        "daemon_state={daemon_state}\nqueue_depth={queue_depth}\nopen_task={open_task}\nturns={turns}\ncontext_prefix_cap={}\ncontext_log_space={}\ncontext_compaction_trigger={}\nlast_compaction={last_compaction}",
        prefix_cap_total(),
        initial_log_space(),
        WHOLE_WINDOW_TRIGGER
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
