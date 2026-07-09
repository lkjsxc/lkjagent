use std::path::Path;

use serde_json::Value;

pub fn missing_endpoint(data_dir: &Path) -> Vec<String> {
    let config = read_json(&data_dir.join("lkjagent.json"));
    [
        ("LKJAGENT_ENDPOINT_URL", "endpoint_url"),
        ("LKJAGENT_MODEL", "endpoint_model"),
    ]
    .into_iter()
    .filter(|(env, flat)| !env_present(env) && !config_present(&config, flat))
    .map(|(env, flat)| format!("{env} or {flat}"))
    .collect()
}

pub fn smoke_configured(root: &Path) -> bool {
    let config = read_json(&root.join("data/lkjagent.json"));
    (env_present("LKJAGENT_ENDPOINT_URL") && env_present("LKJAGENT_MODEL"))
        || (config_present(&config, "endpoint_url") && config_present(&config, "endpoint_model"))
}

pub fn smoke_missing_configured_key(root: &Path) -> bool {
    let config = read_json(&root.join("data/lkjagent.json"));
    config
        .as_ref()
        .and_then(|value| value.get("endpoint_api_key_env"))
        .and_then(Value::as_str)
        .is_some_and(|name| !env_present(name))
}

pub fn force_missing() -> Vec<String> {
    vec![
        "LKJAGENT_ENDPOINT_URL or endpoint_url".to_string(),
        "LKJAGENT_MODEL or endpoint_model".to_string(),
    ]
}

pub fn install_profile_config(root_data: &Path, profile_data: &Path) -> Result<(), String> {
    let root_config = root_data.join("lkjagent.json");
    let profile_config = profile_data.join("lkjagent.json");
    if !root_config.exists() || profile_config.exists() {
        return Ok(());
    }
    std::fs::copy(root_config, profile_config)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn read_json(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn env_present(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

fn config_present(config: &Option<Value>, flat: &str) -> bool {
    config
        .as_ref()
        .and_then(|value| value.get(flat))
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::{config_present, smoke_missing_configured_key};
    use serde_json::json;
    use std::fs;

    #[test]
    fn endpoint_detection_uses_flat_config_only() {
        let nested = Some(json!({
            "endpoint": {
                "url": "http://127.0.0.1",
                "model": "local"
            }
        }));
        let flat = Some(json!({
            "endpoint_url": "http://127.0.0.1",
            "endpoint_model": "local"
        }));

        assert!(!config_present(&nested, "endpoint_url"));
        assert!(!config_present(&nested, "endpoint_model"));
        assert!(config_present(&flat, "endpoint_url"));
        assert!(config_present(&flat, "endpoint_model"));
    }

    #[test]
    fn smoke_key_detection_uses_flat_api_key_env() {
        let root =
            std::env::temp_dir().join(format!("lkjagent-smoke-flat-key-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("data")).expect("data dir");
        fs::write(
            root.join("data/lkjagent.json"),
            r#"{"endpoint_api_key_env":"LKJAGENT_TEST_ABSENT"}"#,
        )
        .expect("config");

        assert!(smoke_missing_configured_key(&root));
    }
}
