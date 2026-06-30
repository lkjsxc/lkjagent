use lkjagent_context::budget::{prefix_cap_total, ContextPressure};
use rusqlite::Connection;

use super::{fact, Policy, StatusFact};
use crate::accounting::AccountingDeck;
use crate::error::CliError;

pub fn push(
    conn: &Connection,
    out: &mut Vec<StatusFact>,
    accounting: &AccountingDeck,
    policy: Policy,
) -> Result<(), CliError> {
    let used = state_value(conn, "context used tokens", "0")?;
    let used_tokens = used.parse::<usize>().unwrap_or_default();
    out.push(fact("context.usage", accounting.context_line.clone()));
    out.push(fact("context.prefix", accounting.prefix_line.clone()));
    out.push(fact("context.window", policy.window.to_string()));
    out.push(fact("context.reserve", policy.reserve.to_string()));
    out.push(fact("context.used_tokens", used));
    out.push(fact("context.prefix_cap", prefix_cap_total().to_string()));
    out.push(fact(
        "context.log_space",
        policy.available_log_space().to_string(),
    ));
    out.push(fact(
        "context.soft_trigger",
        policy.soft_trigger.to_string(),
    ));
    out.push(fact(
        "context.hard_trigger",
        policy.hard_trigger.to_string(),
    ));
    out.push(fact(
        "context.pressure",
        state_value(
            conn,
            "context pressure",
            pressure_name(policy.pressure(used_tokens, 0)),
        )?,
    ));
    Ok(())
}

fn state_value(conn: &Connection, key: &str, default: &str) -> Result<String, CliError> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
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
