use std::path::{Path, PathBuf};
use std::time::Duration;

use lkjagent_llm::client::ClientConfig;
use serde_json::{Map, Value};

use crate::config_registry::{number, parse_document};

pub fn validate_document(text: &str) -> Result<(), String> {
    parse_document(text).map(|_| ())
}

pub fn load_client(data_dir: &Path) -> Result<ClientConfig, String> {
    let values = load_flat_config(data_dir)?;
    let url = env_text("LKJAGENT_ENDPOINT_URL")?
        .or_else(|| text(&values, "endpoint_url"))
        .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
    let model = env_text("LKJAGENT_MODEL")?
        .or_else(|| text(&values, "endpoint_model"))
        .unwrap_or_else(|| "local-model".to_string());
    let timeout = env_integer("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS", 1, 1800)?
        .unwrap_or_else(|| number(&values, "endpoint_timeout_seconds"));
    let mut config = ClientConfig::new(url, model);
    config.timeout = Duration::from_secs(timeout);
    config.api_key = api_key(&values)?;
    Ok(config)
}

pub fn workspace_root(data_dir: &Path) -> Result<PathBuf, String> {
    let values = load_flat_config(data_dir)?;
    let configured = env_text("LKJAGENT_WORKSPACE_ROOT")?
        .or_else(|| text(&values, "workspace_root"))
        .unwrap_or_else(|| crate::workspace_root::DEFAULT_WORKSPACE_ROOT.to_string());
    crate::workspace_root::resolve(data_dir, &configured)
}

#[rustfmt::skip]
pub(crate) fn prompt_max_context_tokens(data_dir: &Path) -> Result<Option<u64>, String> {
    let values = load_flat_config(data_dir)?;
    Ok(Some(env_integer("LKJAGENT_PROMPT_CONTEXT_TOKENS", 2048, 262144)?
        .unwrap_or_else(|| number(&values, "prompt_context_tokens"))))
}

pub(crate) fn load_flat_config(data_dir: &Path) -> Result<Map<String, Value>, String> {
    let path = data_dir.join("lkjagent.json");
    if !path.exists() {
        return Ok(Map::new());
    }
    let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    parse_document(&text)
}

fn text(values: &Map<String, Value>, key: &str) -> Option<String> {
    values.get(key)?.as_str().map(ToString::to_string)
}

fn env_text(name: &str) -> Result<Option<String>, String> {
    match std::env::var(name) {
        Ok(value) if value.trim().is_empty() => Err(format!("{name} must not be empty")),
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

fn env_integer(name: &str, min: u64, max: u64) -> Result<Option<u64>, String> {
    let Some(raw) = env_text(name)? else {
        return Ok(None);
    };
    let value = raw
        .parse::<u64>()
        .map_err(|_| format!("{name} must be an integer"))?;
    if value < min || value > max {
        return Err(format!("{name} is outside {min}..{max}"));
    }
    Ok(Some(value))
}

fn api_key(values: &Map<String, Value>) -> Result<Option<String>, String> {
    let name =
        text(values, "endpoint_api_key_env").unwrap_or_else(|| "LKJAGENT_API_KEY".to_string());
    env_text(&name)
}

pub(crate) fn endpoint_state(data_dir: &Path) -> String {
    let config = load_flat_config(data_dir).ok();
    let root = config.as_ref();
    let url = source("LKJAGENT_ENDPOINT_URL", root, "endpoint_url");
    let model = source("LKJAGENT_MODEL", root, "endpoint_model");
    let key_env = root
        .and_then(|value| value.get("endpoint_api_key_env"))
        .and_then(Value::as_str)
        .unwrap_or("LKJAGENT_API_KEY");
    let credential = if env_present(key_env) {
        "env"
    } else {
        "absent"
    };
    format!("url={url} model={model} credential={credential}")
}

pub(crate) fn workspace_state(data_dir: &Path) -> Result<(PathBuf, bool), String> {
    let root = workspace_root(data_dir)?;
    Ok((root.clone(), root.is_dir()))
}

fn source(env: &str, section: Option<&Map<String, Value>>, key: &str) -> &'static str {
    if env_present(env) {
        return "env";
    }
    if section
        .and_then(|value| value.get(key))
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        "config"
    } else {
        "absent"
    }
}

fn env_present(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}
