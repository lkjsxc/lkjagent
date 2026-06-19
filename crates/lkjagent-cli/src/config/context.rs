use serde_json::{Map, Value};

use lkjagent_context::budget::{
    ContextBudgetError, ContextBudgetPolicy, DEFAULT_GENERATION_RESERVE, DEFAULT_WINDOW_TOKENS,
};

use crate::error::CliError;

use super::json;

pub fn policy_from_config_and_env<F>(
    context: Option<&Map<String, Value>>,
    env: &F,
) -> Result<ContextBudgetPolicy, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    let window = env_usize(env, "LKJAGENT_CONTEXT_LENGTH")?
        .or_else(|| context.and_then(|table| json::usize(table, "window")))
        .unwrap_or(DEFAULT_WINDOW_TOKENS);
    let reserve = context
        .and_then(|table| json::usize(table, "reserve"))
        .unwrap_or(DEFAULT_GENERATION_RESERVE);
    let trigger = context.and_then(|table| json::usize(table, "trigger"));
    let policy = ContextBudgetPolicy::derive(window, reserve, trigger).map_err(policy_error)?;
    if policy.reserve > u16::MAX as usize {
        return Err(CliError::failure(
            "context.reserve must fit endpoint max_tokens",
        ));
    }
    Ok(policy)
}

fn env_usize<F>(env: &F, key: &str) -> Result<Option<usize>, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = env(key).and_then(non_empty) else {
        return Ok(None);
    };
    value
        .parse::<usize>()
        .map(Some)
        .map_err(|error| CliError::failure(format!("{key}: {error}")))
        .and_then(|parsed| match parsed {
            Some(0) => Err(CliError::failure(format!("{key}: must be positive"))),
            other => Ok(other),
        })
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn policy_error(error: ContextBudgetError) -> CliError {
    CliError::failure(error.to_string())
}
