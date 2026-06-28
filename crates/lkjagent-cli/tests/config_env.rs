use std::fs;
use std::path::{Path, PathBuf};
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
    assert_eq!(config.endpoint_timeout_seconds, 180);

    let text = fs::read_to_string(data.join("lkjagent.json"))?;
    assert!(text.contains("\"model\": \"local-model\""));
    assert!(text.contains("\"api-key-env\": \"LKJAGENT_API_KEY\""));
    assert!(text.contains("\"timeout-seconds\": 180"));
    assert!(text.contains("\"window\": 24576"));
    assert!(text.contains("\"reserve\": 512"));
    assert!(text.contains("\"trigger\": 21504"));
    Ok(())
}

#[test]
fn context_length_env_overrides_default_window() -> TestResult<()> {
    let data = temp_data("context-env")?;
    let loaded = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_MODEL" => Some("local-model".to_string()),
        "LKJAGENT_CONTEXT_LENGTH" => Some("16384".to_string()),
        _ => None,
    })?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("env context config did not load".into());
    };
    assert_eq!(config.context_policy.window, 16_384);
    assert_eq!(config.context_policy.hard_trigger, 14_336);

    let text = fs::read_to_string(data.join("lkjagent.json"))?;
    assert!(text.contains("\"window\": 16384"));
    assert!(text.contains("\"trigger\": 14336"));
    Ok(())
}

#[test]
fn invalid_context_window_fails_loudly() -> TestResult<()> {
    let data = temp_data("context-invalid")?;
    let result = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_MODEL" => Some("local-model".to_string()),
        "LKJAGENT_CONTEXT_LENGTH" => Some("12000".to_string()),
        _ => None,
    });
    match result {
        Ok(_) => Err("invalid context length loaded".into()),
        Err(error) => {
            assert!(error.to_string().contains("at least 16384"));
            Ok(())
        }
    }
}

#[test]
fn old_explicit_thirty_two_k_config_stays_valid() -> TestResult<()> {
    let data = temp_data("context-old")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"model\":\"local-test\"},\"context\":{\"window\":32768,\"reserve\":2048,\"trigger\":28672}}",
    )?;
    let loaded = load_or_initialize_with_env(&data, |_| None)?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("old context config did not load".into());
    };
    assert_eq!(config.context_policy.window, 32_768);
    assert_eq!(config.context_policy.hard_trigger, 28_672);
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
    assert_eq!(config.endpoint_timeout_seconds, 180);
    assert_eq!(config.context_policy.window, 24_576);
    Ok(())
}

#[test]
fn endpoint_timeout_uses_env_config_and_default_order() -> TestResult<()> {
    let data = temp_data("timeout")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"model\":\"local-test\",\"timeout-seconds\":45}}",
    )?;
    let loaded = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_ENDPOINT_TIMEOUT_SECONDS" => Some("12".to_string()),
        _ => None,
    })?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("timeout config did not load".into());
    };
    assert_eq!(config.endpoint_timeout_seconds, 12);

    let loaded = load_or_initialize_with_env(&data, |_| None)?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("timeout config did not load".into());
    };
    assert_eq!(config.endpoint_timeout_seconds, 45);

    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"model\":\"local-test\"}}",
    )?;
    let loaded = load_or_initialize_with_env(&data, |_| None)?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("timeout config did not load".into());
    };
    assert_eq!(config.endpoint_timeout_seconds, 180);
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

fn write_config(data: &Path) -> TestResult<()> {
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"url\":\"http://endpoint:8080\",\"model\":\"local-test\"}}",
    )?;
    Ok(())
}
