use std::path::Path;
use std::time::Duration;

use lkjagent_llm::client::{ClientConfig, DEFAULT_TIMEOUT_SECONDS};
use serde_json::Value;

pub fn load_client(data_dir: &Path) -> Result<ClientConfig, String> {
    let value = config_value(data_dir)?;
    let endpoint = value.get("endpoint").unwrap_or(&Value::Null);
    let url = env_or_value("LKJAGENT_ENDPOINT_URL", endpoint, "url")?
        .ok_or_else(|| "endpoint url missing".to_string())?;
    let model = env_or_value("LKJAGENT_MODEL", endpoint, "model")?
        .ok_or_else(|| "endpoint model missing".to_string())?;
    let mut config = ClientConfig::new(url, model);
    config.timeout = Duration::from_secs(
        env_number("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS")
            .or_else(|| number(endpoint, "timeout-seconds"))
            .unwrap_or(DEFAULT_TIMEOUT_SECONDS),
    );
    config.api_key = api_key(endpoint);
    Ok(config)
}

fn config_value(data_dir: &Path) -> Result<Value, String> {
    let path = data_dir.join("lkjagent.json");
    if !path.exists() {
        return Ok(Value::Null);
    }
    let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
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

fn api_key(endpoint: &Value) -> Option<String> {
    let env_name = endpoint
        .get("api-key-env")
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
