use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_cli::config::{load_or_initialize_with_env, ConfigLoad};
use lkjagent_cli::env_file;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn env_model_initializes_first_config() -> TestResult<()> {
    let data = temp_data("env-first")?;
    let loaded = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_ENDPOINT_URL" => Some("http://host.docker.internal:8080".to_string()),
        "LKJAGENT_MODEL" => Some("local-model".to_string()),
        _ => None,
    })?;

    let ConfigLoad::Ready(config) = loaded else {
        return Err("env-backed first start did not produce ready config".into());
    };
    assert_eq!(config.endpoint_url, "http://host.docker.internal:8080");
    assert_eq!(config.endpoint_model, "local-model");

    let text = fs::read_to_string(data.join("lkjagent.toml"))?;
    assert!(text.contains("model = \"local-model\""));
    assert!(text.contains("api-key-env = \"LKJAGENT_API_KEY\""));
    Ok(())
}

#[test]
fn env_values_override_existing_endpoint_config() -> TestResult<()> {
    let data = temp_data("env-override")?;
    write_config(&data)?;

    let loaded = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_ENDPOINT_URL" => Some("http://127.0.0.1:9000".to_string()),
        "LKJAGENT_MODEL" => Some("env-model".to_string()),
        _ => None,
    })?;

    let ConfigLoad::Ready(config) = loaded else {
        return Err("existing config did not load".into());
    };
    assert_eq!(config.endpoint_url, "http://127.0.0.1:9000");
    assert_eq!(config.endpoint_model, "env-model");
    Ok(())
}

#[test]
fn dotenv_parser_accepts_export_and_quotes() -> TestResult<()> {
    let parsed = env_file::parse(
        "# local deployment\n\
         export LKJAGENT_MODEL=\"demo-model\"\n\
         LKJAGENT_ENDPOINT_URL='http://localhost:8080'\n",
    )?;

    assert_eq!(parsed[0].key, "LKJAGENT_MODEL");
    assert_eq!(parsed[0].value, "demo-model");
    assert_eq!(parsed[1].key, "LKJAGENT_ENDPOINT_URL");
    assert_eq!(parsed[1].value, "http://localhost:8080");
    Ok(())
}

fn temp_data(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-cli-config-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn write_config(data: &PathBuf) -> TestResult<()> {
    fs::write(
        data.join("lkjagent.toml"),
        "[endpoint]\nurl = \"http://endpoint:8080\"\nmodel = \"local-test\"\n",
    )?;
    Ok(())
}
