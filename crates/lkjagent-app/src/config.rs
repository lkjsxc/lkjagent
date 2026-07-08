use std::path::{Path, PathBuf};
use std::time::Duration;

use lkjagent_llm::client::{ClientConfig, DEFAULT_TIMEOUT_SECONDS};
use serde_json::{Map, Value};

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
    let flat = flatten(parsed);
    if flat != serde_json::from_str::<Value>(&text).map_err(|error| error.to_string())? {
        let body = serde_json::to_string_pretty(&flat).map_err(|error| error.to_string())?;
        std::fs::write(&path, format!("{body}\n")).map_err(|error| error.to_string())?;
    }
    Ok(flat)
}

fn flatten(value: Value) -> Value {
    let Value::Object(mut map) = value else {
        return value;
    };
    if let Some(Value::Object(endpoint)) = map.remove("endpoint") {
        copy_string(&mut map, &endpoint, "url", "endpoint_url");
        copy_string(&mut map, &endpoint, "model", "endpoint_model");
        copy_string(&mut map, &endpoint, "api-key-env", "endpoint_api_key_env");
        copy_number(
            &mut map,
            &endpoint,
            "timeout-seconds",
            "endpoint_timeout_seconds",
        );
    }
    Value::Object(map)
}

fn copy_string(map: &mut Map<String, Value>, old: &Map<String, Value>, from: &str, to: &str) {
    if map.contains_key(to) {
        return;
    }
    if let Some(value) = old.get(from).and_then(Value::as_str) {
        map.insert(to.to_string(), Value::String(value.to_string()));
    }
}

fn copy_number(map: &mut Map<String, Value>, old: &Map<String, Value>, from: &str, to: &str) {
    if map.contains_key(to) {
        return;
    }
    if let Some(value) = old.get(from).and_then(Value::as_u64) {
        map.insert(to.to_string(), Value::Number(value.into()));
    }
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
