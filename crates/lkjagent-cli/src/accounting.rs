use std::path::Path;

use lkjagent_context::budget::{prefix_cap_total, ContextBudgetPolicy, ContextPressure};
use lkjagent_context::format::{ratio_percent, short_count};
use lkjagent_store::token_usage::{TokenUsageAggregate, TokenUsageFieldAggregate};
use rusqlite::Connection;

use crate::config::load_context_policy_for_status;
use crate::error::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountingDeck {
    pub context_line: String,
    pub token_line: String,
    pub prefix_line: String,
}

pub fn deck_for_data(data_dir: &Path, conn: &Connection) -> Result<AccountingDeck, CliError> {
    deck(conn, load_context_policy_for_status(data_dir)?)
}

pub fn deck(conn: &Connection, policy: ContextBudgetPolicy) -> Result<AccountingDeck, CliError> {
    let used = state_u64(conn, "context used tokens")?;
    let pressure = state_value(
        conn,
        "context pressure",
        pressure_name(policy.pressure(used as usize, 0)),
    )?;
    Ok(AccountingDeck {
        context_line: format!(
            "ctx={}/{} {} pressure={pressure}",
            short_count(used),
            short_count(policy.window as u64),
            ratio_percent(used, policy.window as u64)
        ),
        token_line: token_line(conn)?,
        prefix_line: prefix_line(policy, used),
    })
}

fn token_line(conn: &Connection) -> Result<String, CliError> {
    let latest = lkjagent_store::token_usage::aggregate_latest(conn)?;
    let task = active_task_aggregate(conn)?;
    let session = lkjagent_store::token_usage::aggregate_session(conn)?;
    let all = lkjagent_store::token_usage::aggregate_all(conn)?;
    Ok(format!(
        "tokens latest={} task={} session={} all={}",
        format_aggregate(latest),
        format_aggregate(task),
        format_aggregate(session),
        format_aggregate(all)
    ))
}

fn active_task_aggregate(conn: &Connection) -> Result<TokenUsageAggregate, CliError> {
    let Some(case) = lkjagent_store::graph::active_case(conn)? else {
        return Ok(TokenUsageAggregate::default());
    };
    Ok(lkjagent_store::token_usage::aggregate_task(conn, case.id)?)
}

fn format_aggregate(aggregate: TokenUsageAggregate) -> String {
    if aggregate.rows == 0 {
        return "none".to_string();
    }
    format!(
        "in:{} out:{} cache:{} total:{} unknown:{} cache_ratio:{}",
        format_field(aggregate.input_tokens),
        format_field(aggregate.output_tokens),
        format_field(aggregate.cached_input_tokens),
        format_field(aggregate.total_tokens),
        aggregate.rows_with_unknown,
        cache_ratio(aggregate)
    )
}

fn format_field(field: TokenUsageFieldAggregate) -> String {
    if field.known == 0 && field.unknown > 0 {
        "unknown".to_string()
    } else {
        short_count(field.sum)
    }
}

fn cache_ratio(aggregate: TokenUsageAggregate) -> String {
    if aggregate.input_tokens.unknown > 0
        || aggregate.cached_input_tokens.unknown > 0
        || aggregate.input_tokens.sum == 0
    {
        return "unknown".to_string();
    }
    format!(
        "{:.2}",
        aggregate.cached_input_tokens.sum as f64 / aggregate.input_tokens.sum as f64
    )
}

fn prefix_line(policy: ContextBudgetPolicy, used: u64) -> String {
    let headroom = policy.window.saturating_sub(used as usize);
    format!(
        "prefix={} log={} reserve={} headroom={}",
        short_count(prefix_cap_total() as u64),
        short_count(policy.available_log_space() as u64),
        short_count(policy.reserve as u64),
        short_count(headroom as u64)
    )
}

fn state_u64(conn: &Connection, key: &str) -> Result<u64, CliError> {
    Ok(state_value(conn, key, "0")?
        .parse::<u64>()
        .unwrap_or_default())
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
