use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

pub const DEFAULT_CONFIG: &str = "# lkjagent config
[endpoint]
url = \"http://endpoint:8080\"
# model = \"fill-this-in\"
api-key-env = \"LKJAGENT_API_KEY\"

[context]
window = 32768
reserve = 1024
trigger = 28672

[sampling]
temperature = 0.3
top-p = 0.9

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
    load_or_initialize_with_env(data_dir, process_env)
}

pub fn load_or_initialize_with_env<F>(data_dir: &Path, env: F) -> Result<ConfigLoad, CliError>
where
    F: Fn(&str) -> Option<String>,
{
    fs::create_dir_all(data_dir)?;
    let path = data_dir.join("lkjagent.toml");
    if !path.exists() {
        if let Some(config) = config_from_env(path.clone(), &env) {
            fs::write(&path, render_config(&config))?;
            return Ok(ConfigLoad::Ready(config));
        }
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
    let model = env_value(&env, "LKJAGENT_MODEL")
        .or_else(|| table_value(endpoint, "model"))
        .ok_or_else(|| CliError::failure("missing endpoint.model"))?;
    Ok(ConfigLoad::Ready(RuntimeConfig {
        path,
        endpoint_url: env_value(&env, "LKJAGENT_ENDPOINT_URL")
            .or_else(|| table_value(endpoint, "url"))
            .unwrap_or_else(|| "http://endpoint:8080".to_string()),
        endpoint_model: model,
        api_key_env: table_value(endpoint, "api-key-env")
            .unwrap_or_else(|| "LKJAGENT_API_KEY".to_string()),
    }))
}

fn config_from_env<F>(path: PathBuf, env: &F) -> Option<RuntimeConfig>
where
    F: Fn(&str) -> Option<String>,
{
    let endpoint_model = env_value(env, "LKJAGENT_MODEL")?;
    Some(RuntimeConfig {
        path,
        endpoint_url: env_value(env, "LKJAGENT_ENDPOINT_URL")
            .unwrap_or_else(|| "http://endpoint:8080".to_string()),
        endpoint_model,
        api_key_env: "LKJAGENT_API_KEY".to_string(),
    })
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

fn table_value(table: &toml::Table, key: &str) -> Option<String> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .and_then(|value| non_empty(value.to_string()))
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn render_config(config: &RuntimeConfig) -> String {
    let mut text = String::from("# lkjagent config\n[endpoint]\nurl = ");
    push_toml_string(&mut text, &config.endpoint_url);
    text.push_str("\nmodel = ");
    push_toml_string(&mut text, &config.endpoint_model);
    text.push_str("\napi-key-env = ");
    push_toml_string(&mut text, &config.api_key_env);
    text.push_str(
        "\n\n[context]\nwindow = 32768\nreserve = 1024\ntrigger = 28672\n\n\
         [sampling]\ntemperature = 0.3\ntop-p = 0.9\n\n[task]\nturn-budget = 64\n",
    );
    text
}

fn push_toml_string(text: &mut String, value: &str) {
    text.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => text.push_str("\\\\"),
            '"' => text.push_str("\\\""),
            '\n' => text.push_str("\\n"),
            '\r' => text.push_str("\\r"),
            '\t' => text.push_str("\\t"),
            other => text.push(other),
        }
    }
    text.push('"');
}
