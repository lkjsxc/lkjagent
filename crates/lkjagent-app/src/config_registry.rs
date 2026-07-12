use std::path::{Component, Path};

use serde_json::{Map, Value};

const STABLE_PREFIX_TOKENS: u64 = 1024;

#[derive(Clone, Copy)]
enum Rule {
    Text,
    Integer { min: u64, max: u64, default: u64 },
    Boolean,
}

const RULES: &[(&str, Rule)] = &[
    ("endpoint_url", Rule::Text),
    ("endpoint_model", Rule::Text),
    ("endpoint_api_key_env", Rule::Text),
    ("endpoint_timeout_seconds", integer(1, 1800, 300)),
    ("endpoint_retry_limit", integer(0, 8, 3)),
    ("endpoint_backoff_milliseconds", integer(50, 60000, 500)),
    ("queue_wake_milliseconds", integer(50, 60000, 500)),
    ("no_progress_window", integer(1, 10, 3)),
    (
        "case_token_budget",
        integer(1024, 1_000_000_000, 10_000_000),
    ),
    (
        "case_active_milliseconds",
        integer(1000, 31_536_000_000, 604_800_000),
    ),
    ("case_effect_budget", integer(1, 1_000_000, 10_000)),
    ("case_recovery_budget", integer(1, 100_000, 100)),
    ("live_campaign_seconds", integer(840, 7200, 900)),
    ("prompt_context_tokens", integer(2048, 262144, 16384)),
    ("prompt_output_reserve_tokens", integer(256, 32768, 2048)),
    ("context_objective_tokens", integer(64, 4096, 512)),
    ("context_evidence_tokens", integer(128, 32768, 8192)),
    ("context_history_tokens", integer(64, 16384, 2048)),
    ("context_recovery_tokens", integer(64, 8192, 1024)),
    ("context_retrieval_limit", integer(1, 50, 12)),
    ("tool_view_max_items", integer(1, 4, 4)),
    ("workspace_root", Rule::Text),
    ("timezone", Rule::Text),
    ("workspace_file_max_tokens", integer(64, 512, 512)),
    (
        "workspace_scan_debounce_milliseconds",
        integer(50, 10000, 500),
    ),
    ("workspace_reconcile_seconds", integer(30, 86400, 900)),
    ("navigation_page_max_items", integer(10, 200, 80)),
    ("archive_after_days", integer(30, 36500, 3650)),
    ("activity_retention_days", integer(30, 36500, 3650)),
    ("maintenance_interval_seconds", integer(60, 86400, 900)),
    ("effect_output_max_bytes", integer(1024, 1048576, 131072)),
    ("recovery_parse_attempts", integer(0, 3, 1)),
    ("recovery_output_limit_attempts", integer(0, 8, 4)),
    ("recovery_effect_attempts", integer(0, 3, 2)),
    ("shell_enabled", Rule::Boolean),
    ("shell_timeout_seconds", integer(1, 1800, 300)),
    ("tui_refresh_milliseconds", integer(16, 2000, 100)),
    ("tui_history_messages", integer(50, 10000, 1000)),
];

const fn integer(min: u64, max: u64, default: u64) -> Rule {
    Rule::Integer { min, max, default }
}

pub(crate) fn parse_document(text: &str) -> Result<Map<String, Value>, String> {
    let parsed: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let Value::Object(values) = parsed else {
        return Err("lkjagent.json must be a flat JSON object".to_string());
    };
    validate(&values)?;
    Ok(values)
}

pub(crate) fn validate(values: &Map<String, Value>) -> Result<(), String> {
    for (key, value) in values {
        if value.is_object() {
            return Err(format!("lkjagent.json key '{key}' must not be nested"));
        }
        if value.is_array() {
            return Err(format!("lkjagent.json key '{key}' must be scalar"));
        }
        let Some((_, rule)) = RULES.iter().find(|(name, _)| name == key) else {
            return Err(format!("lkjagent.json contains unknown key '{key}'"));
        };
        match (rule, value) {
            (Rule::Text, Value::String(text)) if !text.trim().is_empty() => {}
            (Rule::Boolean, Value::Bool(_)) => {}
            (Rule::Integer { min, max, .. }, Value::Number(value))
                if value
                    .as_u64()
                    .is_some_and(|value| value >= *min && value <= *max) => {}
            _ => {
                return Err(format!(
                    "lkjagent.json key '{key}' has invalid type or value"
                ))
            }
        }
    }
    validate_cross_keys(values)
}

pub(crate) fn number(values: &Map<String, Value>, key: &str) -> u64 {
    values.get(key).and_then(Value::as_u64).unwrap_or_else(|| {
        RULES
            .iter()
            .find_map(|(name, rule)| match rule {
                Rule::Integer { default, .. } if *name == key => Some(*default),
                _ => None,
            })
            .unwrap_or(0)
    })
}

fn validate_cross_keys(values: &Map<String, Value>) -> Result<(), String> {
    let context = number(values, "prompt_context_tokens");
    let reserve = number(values, "prompt_output_reserve_tokens");
    if reserve >= context {
        return Err("prompt output reserve must be smaller than prompt context".to_string());
    }
    let lanes = [
        "context_objective_tokens",
        "context_evidence_tokens",
        "context_history_tokens",
        "context_recovery_tokens",
    ]
    .iter()
    .map(|key| number(values, key))
    .sum::<u64>();
    if lanes + STABLE_PREFIX_TOKENS > context - reserve {
        return Err("context lane caps exceed the prompt remainder".to_string());
    }
    if let Some(zone) = values.get("timezone").and_then(Value::as_str) {
        validate_timezone(zone)?;
    }
    if let Some(root) = values.get("workspace_root").and_then(Value::as_str) {
        if root.chars().any(char::is_control) {
            return Err("workspace root must not contain control characters".to_string());
        }
    }
    Ok(())
}

fn validate_timezone(zone: &str) -> Result<(), String> {
    let path = Path::new(zone);
    if path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir))
    {
        return Err("timezone must be an IANA name".to_string());
    }
    if !Path::new("/usr/share/zoneinfo").join(path).is_file() {
        return Err(format!(
            "timezone is not present in the IANA database: {zone}"
        ));
    }
    Ok(())
}
