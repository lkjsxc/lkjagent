use serde_json::{Map, Value};

#[derive(Clone, Copy)]
enum Rule {
    Text,
    Integer { min: u64, max: u64, default: u64 },
}

const RULES: &[(&str, Rule)] = &[
    ("endpoint_url", Rule::Text),
    ("endpoint_model", Rule::Text),
    ("endpoint_api_key_env", Rule::Text),
    ("endpoint_timeout_seconds", integer(1, 1800, 300)),
    ("live_campaign_seconds", integer(840, 7200, 900)),
    ("prompt_context_tokens", integer(2048, 262144, 16384)),
    ("workspace_root", Rule::Text),
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
            (Rule::Integer { min, max, .. }, Value::Number(number))
                if number
                    .as_u64()
                    .is_some_and(|number| number >= *min && number <= *max) => {}
            _ => {
                return Err(format!(
                    "lkjagent.json key '{key}' has invalid type or value"
                ))
            }
        }
    }
    if values
        .get("workspace_root")
        .and_then(Value::as_str)
        .is_some_and(|root| root.chars().any(char::is_control))
    {
        return Err("workspace root must not contain control characters".to_string());
    }
    Ok(())
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
