use lkjagent_context::budget::{prefix_cap_total, ContextBudgetPolicy};
use lkjagent_context::format::{optional_count, ratio_percent, short_count};
use rusqlite::Connection;

use crate::error::RuntimeResult;

pub const MAX_LOG_CHARS: usize = 1_000_000;
pub const LOG_TAIL_RESERVE_CHARS: usize = 20_000;
pub const MAX_TABLE_CELL_CHARS: usize = 512;
pub const MAX_TRANSCRIPT_CELL_CHARS: usize = 4_000;

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
    cell_limited(value, MAX_TABLE_CELL_CHARS)
}

pub fn cell_limited(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let compact = value
        .replace('|', "\\|")
        .replace('\n', "<br>")
        .trim()
        .to_string();
    if compact.chars().count() <= max_chars {
        compact
    } else {
        let marker = "...";
        if max_chars <= marker.chars().count() {
            return marker.chars().take(max_chars).collect();
        }
        let body_chars = max_chars - marker.chars().count();
        format!(
            "{}{}",
            compact.chars().take(body_chars).collect::<String>(),
            marker
        )
    }
}

pub fn trim_to_char_budget(text: &mut String, max_chars: usize) {
    if text.chars().count() <= max_chars {
        return;
    }
    let marker = "\n\n<!-- truncated to configured GPT handoff budget -->\n";
    if max_chars <= marker.chars().count() {
        *text = marker.chars().take(max_chars).collect();
        return;
    }
    let keep = max_chars - marker.chars().count();
    let mut trimmed = text.chars().take(keep).collect::<String>();
    trimmed.push_str(marker);
    *text = trimmed;
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

#[cfg(test)]
mod tests {
    use super::{cell_limited, trim_to_char_budget};

    #[test]
    fn cell_limited_respects_char_budget() {
        assert_eq!(cell_limited("abcdef", 6), "abcdef");
        assert_eq!(cell_limited("abcdef", 5), "ab...");
        assert_eq!(cell_limited("a|b\nc", 20), "a\\|b<br>c");
    }

    #[test]
    fn trim_to_char_budget_keeps_output_within_limit() {
        let mut text = "x".repeat(200);
        trim_to_char_budget(&mut text, 100);
        assert!(text.chars().count() <= 100);
        assert!(text.contains("truncated to configured GPT handoff budget"));
    }
}
