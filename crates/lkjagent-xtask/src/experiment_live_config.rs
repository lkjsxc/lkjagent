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
    use super::config_present;
    use serde_json::json;

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
}
