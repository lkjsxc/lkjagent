use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

pub const DEFAULT_CONFIG: &str = "# lkjagent config
[endpoint]
url = \"http://endpoint:8080\"
# model = \"fill-this-in\"
api-key-env = \"LKJAGENT_API_KEY\"

[task]
turn-budget = 64
";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub path: PathBuf,
    pub endpoint_url: String,
    pub endpoint_model: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigLoad {
    Ready(RuntimeConfig),
    WroteDefault { path: PathBuf },
}

pub fn load_or_initialize(data_dir: &Path) -> Result<ConfigLoad, CliError> {
    fs::create_dir_all(data_dir)?;
    let path = data_dir.join("lkjagent.toml");
    if !path.exists() {
        fs::write(&path, DEFAULT_CONFIG)?;
        return Ok(ConfigLoad::WroteDefault { path });
    }
    let text = fs::read_to_string(&path)?;
    let value = text
        .parse::<toml::Value>()
        .map_err(|error| CliError::failure(error.to_string()))?;
    let endpoint = value
        .get("endpoint")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| CliError::failure("missing endpoint table"))?;
    let model = endpoint
        .get("model")
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| CliError::failure("missing endpoint.model"))?;
    Ok(ConfigLoad::Ready(RuntimeConfig {
        path,
        endpoint_url: string_value(endpoint, "url", "http://endpoint:8080"),
        endpoint_model: model.to_string(),
        api_key_env: string_value(endpoint, "api-key-env", "LKJAGENT_API_KEY"),
    }))
}

fn string_value(table: &toml::Table, key: &str, default: &str) -> String {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or(default)
        .to_string()
}
