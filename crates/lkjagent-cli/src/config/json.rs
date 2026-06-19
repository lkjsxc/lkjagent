use serde_json::{json, Map, Value};

use crate::config::RuntimeConfig;
use crate::error::CliError;

pub fn parse(text: &str) -> Result<Value, CliError> {
    serde_json::from_str(text).map_err(|error| CliError::failure(error.to_string()))
}

pub fn required_object<'a>(
    value: &'a Value,
    key: &str,
) -> Result<&'a Map<String, Value>, CliError> {
    object(value, key).ok_or_else(|| CliError::failure(format!("missing {key} object")))
}

pub fn object<'a>(value: &'a Value, key: &str) -> Option<&'a Map<String, Value>> {
    value.get(key).and_then(Value::as_object)
}

pub fn string(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .and_then(|value| non_empty(value.to_string()))
}

pub fn u64(object: &Map<String, Value>, key: &str) -> Option<u64> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .filter(|value| *value > 0)
}

pub fn render_config(config: &RuntimeConfig) -> Result<String, CliError> {
    let value = json!({
        "endpoint": {
            "url": config.endpoint_url,
            "model": config.endpoint_model,
            "api-key-env": config.api_key_env,
            "timeout-seconds": config.endpoint_timeout_seconds
        },
        "context": {
            "window": 32768,
            "reserve": 2048,
            "trigger": 28672
        },
        "sampling": {
            "temperature": 0.3,
            "top-p": 0.9
        },
        "task": {
            "turn-budget": 64
        },
        "daemon": {
            "lock-stale-seconds": config.daemon_lock_stale_seconds
        }
    });
    let mut text = serde_json::to_string_pretty(&value)
        .map_err(|error| CliError::failure(error.to_string()))?;
    text.push('\n');
    Ok(text)
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
