use std::path::{Path, PathBuf};
use std::time::Duration;

use lkjagent_llm::client::{ClientConfig, DEFAULT_TIMEOUT_SECONDS};
use serde_json::Value;

pub fn load_client(data_dir: &Path) -> Result<ClientConfig, String> {
    let value = load_flat_config(data_dir)?;
    let url = env_or_value("LKJAGENT_ENDPOINT_URL", &value, "endpoint_url")?
        .ok_or_else(|| "endpoint url missing".to_string())?;
    let model = env_or_value("LKJAGENT_MODEL", &value, "endpoint_model")?
        .ok_or_else(|| "endpoint model missing".to_string())?;
    let mut config = ClientConfig::new(url, model);
    config.timeout = Duration::from_secs(
        env_number("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS")
            .or_else(|| number(&value, "endpoint_timeout_seconds"))
            .unwrap_or(DEFAULT_TIMEOUT_SECONDS),
    );
    config.api_key = api_key(&value);
    Ok(config)
}

pub(crate) fn workspace_root(data_dir: &Path) -> Result<PathBuf, String> {
    let value = load_flat_config(data_dir)?;
    let root = env_or_value("LKJAGENT_WORKSPACE_ROOT", &value, "workspace_root")?
        .unwrap_or_else(|| "workspace".to_string());
    let path = PathBuf::from(root);
    Ok(if path.is_absolute() {
        path
    } else {
        data_dir.join(path)
    })
}

pub(crate) fn prompt_max_context_tokens(data_dir: &Path) -> Result<Option<u64>, String> {
    let value = load_flat_config(data_dir)?;
    Ok(env_number("LKJAGENT_PROMPT_MAX_CONTEXT_TOKENS")
        .or_else(|| number(&value, "prompt_max_context_tokens")))
}

pub(crate) fn live_campaign_seconds(data_dir: &Path) -> Result<Option<u64>, String> {
    let value = load_flat_config(data_dir)?;
    Ok(env_number("LKJAGENT_LIVE_CAMPAIGN_SECONDS")
        .or_else(|| number(&value, "live_campaign_seconds")))
}

pub(crate) fn load_flat_config(data_dir: &Path) -> Result<Value, String> {
    let path = data_dir.join("lkjagent.json");
    if !path.exists() {
        return Ok(Value::Null);
    }
    let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let parsed: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    validate_flat_config(&parsed)?;
    Ok(parsed)
}

fn validate_flat_config(value: &Value) -> Result<(), String> {
    let Value::Object(map) = value else {
        return Err("lkjagent.json must be a flat JSON object".to_string());
    };
    for (key, value) in map {
        if key.trim().is_empty() {
            return Err("lkjagent.json keys must not be empty".to_string());
        }
        match value {
            Value::Object(_) => {
                return Err(format!("lkjagent.json key '{key}' must not be nested"));
            }
            Value::Array(items) if items.iter().any(|item| item.is_object() || item.is_array()) => {
                return Err(format!("lkjagent.json key '{key}' array must be flat"));
            }
            _ => {}
        }
    }
    Ok(())
}

fn env_or_value(env: &str, section: &Value, key: &str) -> Result<Option<String>, String> {
    if let Ok(value) = std::env::var(env) {
        if !value.trim().is_empty() {
            return Ok(Some(value));
        }
    }
    Ok(section
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string))
}

fn api_key(config: &Value) -> Option<String> {
    let env_name = config
        .get("endpoint_api_key_env")
        .and_then(Value::as_str)
        .unwrap_or("LKJAGENT_API_KEY");
    std::env::var(env_name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn env_number(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

fn number(section: &Value, key: &str) -> Option<u64> {
    section.get(key).and_then(Value::as_u64)
}
