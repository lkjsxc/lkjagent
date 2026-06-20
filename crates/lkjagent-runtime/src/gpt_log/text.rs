use lkjagent_context::budget::{prefix_cap_total, ContextBudgetPolicy};
use lkjagent_context::format::{optional_count, ratio_percent, short_count};
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub fn section(out: &mut String, title: &str) {
    out.push_str("## ");
    out.push_str(title);
    out.push_str("\n\n");
}

pub fn line(out: &mut String, key: &str, value: &str) {
    out.push_str("- ");
    out.push_str(key);
    out.push_str(": ");
    out.push_str(value);
    out.push('\n');
}

pub fn bullets(out: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        out.push_str("* ");
        out.push_str(label);
        out.push_str(": none\n");
        return;
    }
    for value in values {
        out.push_str("* ");
        out.push_str(label);
        out.push_str(": ");
        out.push_str(value);
        out.push('\n');
    }
}

pub fn cell(value: &str) -> String {
    let compact = value
        .replace('|', "\\|")
        .replace('\n', "<br>")
        .trim()
        .to_string();
    if compact.chars().count() <= 96 {
        compact
    } else {
        format!("{}...", compact.chars().take(93).collect::<String>())
    }
}

pub fn context_line(used: u64, budget: ContextBudgetPolicy) -> String {
    let prefix = prefix_cap_total() as u64;
    let log = budget.available_log_space() as u64;
    let reserve = budget.reserve as u64;
    let headroom = budget.window.saturating_sub(used as usize) as u64;
    format!(
        "{}/{} {} prefix={} log={} reserve={} headroom={}",
        short_count(used),
        short_count(budget.window as u64),
        ratio_percent(used, budget.window as u64),
        short_count(prefix),
        short_count(log),
        short_count(reserve),
        short_count(headroom)
    )
}

pub fn token_line(usage: Option<&lkjagent_store::token_usage::TokenUsageEvent>) -> String {
    format!(
        "in={} out={} cache={} total={}",
        optional_count(usage.and_then(|row| row.input_tokens)),
        optional_count(usage.and_then(|row| row.output_tokens)),
        optional_count(usage.and_then(|row| row.cached_input_tokens)),
        optional_count(usage.and_then(|row| row.total_tokens))
    )
}

pub fn state_value(conn: &Connection, key: &str, default: &str) -> RuntimeResult<String> {
    Ok(lkjagent_store::state::get(conn, key)?.unwrap_or_else(|| default.to_string()))
}

pub fn state_u64(conn: &Connection, key: &str) -> RuntimeResult<u64> {
    Ok(state_value(conn, key, "0")?
        .parse::<u64>()
        .unwrap_or_default())
}
