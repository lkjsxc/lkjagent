use rusqlite::Connection;

use super::{fact, status_context, CoreValues, Policy, StatusFact};
use crate::accounting::AccountingDeck;
use crate::error::CliError;

pub fn push_all(
    conn: &Connection,
    out: &mut Vec<StatusFact>,
    active: Option<&lkjagent_store::graph::GraphCaseRow>,
    values: &CoreValues,
    accounting: &AccountingDeck,
    policy: Policy,
) -> Result<(), CliError> {
    push_runtime(conn, out, values)?;
    out.push(fact("queue.pending", values.pending.to_string()));
    push_task(out, active, values);
    push_authority(conn, out)?;
    push_artifact(conn, out)?;
    status_context::push(conn, out, accounting, policy)?;
    out.push(fact("tokens.usage", accounting.token_line.clone()));
    out.push(fact("model.log", values.model_log.clone()));
    out.push(fact("next.action", values.next_action.clone()));
    out.push(fact(
        "next.missing",
        state_value(conn, "authority evidence gaps", "none")?,
    ));
    out.push(fact("diagnostic.question", values.question.clone()));
    out.push(fact("diagnostic.error", values.error.clone()));
    out.push(fact("runtime.last_compaction", last_compaction(conn)?));
    Ok(())
}

pub fn state_value(conn: &Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

pub fn active_states(conn: &Connection, case_id: Option<i64>) -> Result<String, CliError> {
    let Some(case_id) = case_id else {
        return Ok("none".to_string());
    };
    let rows = lkjagent_store::graph::state_tracks::state_tracks_for_case(conn, case_id)?;
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

fn push_runtime(
    conn: &Connection,
    out: &mut Vec<StatusFact>,
    values: &CoreValues,
) -> Result<(), CliError> {
    out.push(fact("runtime.daemon_state", values.daemon_state.clone()));
    out.push(fact("runtime.turns", values.turns.clone()));
    for (key, state_key, default) in [
        ("runtime.continuation_epoch", "continuation epoch", "0"),
        ("runtime.continuation_turns", "continuation turns used", "0"),
        ("runtime.checkpoint_turns", "checkpoint turns", "0"),
        (
            "runtime.last_checkpoint_reason",
            "last checkpoint reason",
            "none",
        ),
        (
            "runtime.continuation_decision",
            "continuation decision",
            "none",
        ),
    ] {
        out.push(fact(key, state_value(conn, state_key, default)?));
    }
    Ok(())
}

fn push_task(
    out: &mut Vec<StatusFact>,
    active: Option<&lkjagent_store::graph::GraphCaseRow>,
    values: &CoreValues,
) {
    out.push(fact("task.open", values.open_task.clone()));
    out.push(fact(
        "task.active_case",
        opt_case(active, |row| row.id.to_string()),
    ));
    out.push(fact(
        "task.phase",
        opt_case(active, |row| row.phase.clone()),
    ));
    out.push(fact(
        "task.node",
        opt_case(active, |row| row.active_node.clone()),
    ));
    out.push(fact(
        "task.status",
        opt_case(active, |row| row.status.clone()),
    ));
    out.push(fact("task.states", values.active_states.clone()));
}

fn push_authority(conn: &Connection, out: &mut Vec<StatusFact>) -> Result<(), CliError> {
    for (key, state_key) in [
        ("authority.active_mode", "authority active mode"),
        ("authority.case", "authority case id"),
        ("authority.phase", "authority phase"),
        ("authority.node", "authority node"),
        ("authority.allowed_tools", "authority allowed tools"),
        ("authority.blocked_tools", "authority blocked tools"),
        ("authority.resolver_plan", "authority resolver plan"),
        ("authority.progress_key", "authority progress key"),
    ] {
        out.push(fact(key, state_value(conn, state_key, "none")?));
    }
    Ok(())
}

fn push_artifact(conn: &Connection, out: &mut Vec<StatusFact>) -> Result<(), CliError> {
    for (key, state_key) in [
        ("artifact.root", "authority artifact root"),
        ("artifact.recovery_route", "authority recovery route"),
        (
            "artifact.last_observation",
            "authority last successful observation",
        ),
        (
            "artifact.last_failed_action",
            "authority last failed action",
        ),
    ] {
        out.push(fact(key, state_value(conn, state_key, "none")?));
    }
    super::status_artifact::push_progress(conn, out)?;
    Ok(())
}

fn last_compaction(conn: &Connection) -> Result<String, CliError> {
    if let Some(summary) = latest_compaction_snapshot(conn)? {
        return Ok(summary);
    }
    let events = lkjagent_store::events::read_events(conn)?;
    Ok(events
        .iter()
        .rev()
        .find(|event| event.kind == "compaction")
        .map_or_else(|| "none".to_string(), |event| event.created_at.clone()))
}

fn latest_compaction_snapshot(conn: &Connection) -> Result<Option<String>, CliError> {
    let Some(case) = lkjagent_store::graph::active_case(conn)? else {
        return Ok(None);
    };
    let Some(row) = lkjagent_store::graph::snapshots::latest_compaction_snapshot(conn, case.id)?
    else {
        return Ok(None);
    };
    Ok(Some(format!(
        "snapshot:{} phase={} node={} fields={}",
        row.created_at,
        row.phase,
        row.active_node,
        row.preserved_fields.replace('\n', ";")
    )))
}

fn opt_case<F>(active: Option<&lkjagent_store::graph::GraphCaseRow>, f: F) -> String
where
    F: FnOnce(&lkjagent_store::graph::GraphCaseRow) -> String,
{
    active.map_or_else(|| "none".to_string(), f)
}
