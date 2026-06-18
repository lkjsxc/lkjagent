mod json;

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

pub const DEFAULT_ENDPOINT_TIMEOUT_SECONDS: u64 = 180;
pub const DEFAULT_LOCK_STALE_SECONDS: u64 = 300;
pub const CONFIG_FILE: &str = "lkjagent.json";

pub const DEFAULT_CONFIG: &str = r#"{
  "endpoint": {
    "url": "http://endpoint:8080",
    "model": "",
    "api-key-env": "LKJAGENT_API_KEY",
    "timeout-seconds": 180
  },
  "context": {
    "window": 32768,
    "reserve": 1024,
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
    "lock-stale-seconds": 300
  }
}
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub path: PathBuf,
    pub endpoint_url: String,
    pub endpoint_model: String,
    pub api_key_env: String,
    pub endpoint_timeout_seconds: u64,
    pub daemon_lock_stale_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigLoad {
    Ready(RuntimeConfig),
    WroteDefault { path: PathBuf },
}

pub fn load_or_initialize(data_dir: &Path) -> Result<ConfigLoad, CliError> {
    load_or_initialize_with_env(data_dir, process_env)
}

pub fn load_or_initialize_with_env<F>(data_dir: &Path, env: F) -> Result<ConfigLoad, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    fs::create_dir_all(data_dir)?;
    let path = data_dir.join(CONFIG_FILE);
    if !path.exists() {
        if let Some(config) = config_from_env(path.clone(), &env)? {
            fs::write(&path, json::render_config(&config)?)?;
            return Ok(ConfigLoad::Ready(config));
        }
        fs::write(&path, DEFAULT_CONFIG)?;
        return Ok(ConfigLoad::WroteDefault { path });
    }
    let text = fs::read_to_string(&path)?;
    let value = json::parse(&text)?;
    let endpoint = json::required_object(&value, "endpoint")?;
    let daemon = json::object(&value, "daemon");
    let model = env_value(&env, "LKJAGENT_MODEL")
        .or_else(|| json::string(endpoint, "model"))
        .ok_or_else(|| CliError::failure("missing endpoint.model"))?;
    Ok(ConfigLoad::Ready(RuntimeConfig {
        path,
        endpoint_url: env_value(&env, "LKJAGENT_ENDPOINT_URL")
            .or_else(|| json::string(endpoint, "url"))
            .unwrap_or_else(|| "http://endpoint:8080".to_string()),
        endpoint_model: model,
        api_key_env: json::string(endpoint, "api-key-env")
            .unwrap_or_else(|| "LKJAGENT_API_KEY".to_string()),
        endpoint_timeout_seconds: env_u64(&env, "LKJAGENT_ENDPOINT_TIMEOUT_SECONDS")?
            .or_else(|| json::u64(endpoint, "timeout-seconds"))
            .unwrap_or(DEFAULT_ENDPOINT_TIMEOUT_SECONDS),
        daemon_lock_stale_seconds: daemon
            .and_then(|table| json::u64(table, "lock-stale-seconds"))
            .unwrap_or(DEFAULT_LOCK_STALE_SECONDS),
    }))
}

fn config_from_env<F>(path: PathBuf, env: &F) -> Result<Option<RuntimeConfig>, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(endpoint_model) = env_value(env, "LKJAGENT_MODEL") else {
        return Ok(None);
    };
    Ok(Some(RuntimeConfig {
        path,
        endpoint_url: env_value(env, "LKJAGENT_ENDPOINT_URL")
            .unwrap_or_else(|| "http://endpoint:8080".to_string()),
        endpoint_model,
        api_key_env: "LKJAGENT_API_KEY".to_string(),
        endpoint_timeout_seconds: env_u64(env, "LKJAGENT_ENDPOINT_TIMEOUT_SECONDS")?
            .unwrap_or(DEFAULT_ENDPOINT_TIMEOUT_SECONDS),
        daemon_lock_stale_seconds: DEFAULT_LOCK_STALE_SECONDS,
    }))
}

fn process_env(key: &str) -> Option<String> {
    std::env::var(key).ok().and_then(non_empty)
}

fn env_value<F>(env: &F, key: &str) -> Option<String>
where
    F: Fn(&str) -> Option<String>,
{
    env(key).and_then(non_empty)
}

fn env_u64<F>(env: &F, key: &str) -> Result<Option<u64>, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = env_value(env, key) else {
        return Ok(None);
    };
    value
        .parse::<u64>()
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
