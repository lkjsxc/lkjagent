use std::fs;
use std::path::Path;

use serde_json::Value;

pub fn configured(root: &Path) -> bool {
    env_pair_present() || flat_endpoint_present(&read_config(root))
}

pub fn missing_configured_key(root: &Path) -> bool {
    read_config(root)
        .get("endpoint_api_key_env")
        .and_then(Value::as_str)
        .is_some_and(|name| !env_present(name))
}

fn env_pair_present() -> bool {
    env_present("LKJAGENT_ENDPOINT_URL") && env_present("LKJAGENT_MODEL")
}

fn env_present(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

fn flat_endpoint_present(value: &Value) -> bool {
    flat_string(value, "endpoint_url") && flat_string(value, "endpoint_model")
}

fn flat_string(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
}

fn read_config(root: &Path) -> Value {
    let Ok(text) = fs::read_to_string(root.join("data/lkjagent.json")) else {
        return Value::Null;
    };
    let Ok(value) = serde_json::from_str::<Value>(&text) else {
        return Value::Null;
    };
    match value {
        Value::Object(_) => value,
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_endpoint_keys_configure_live_smoke() {
        let root = fixture("flat");
        write_config(
            &root,
            r#"{"endpoint_url":"http://localhost","endpoint_model":"local"}"#,
        );

        assert!(flat_endpoint_present(&read_config(&root)));
    }

    #[test]
    fn old_nested_endpoint_shape_is_ignored() {
        let root = fixture("nested");
        write_config(
            &root,
            r#"{"endpoint":{"url":"http://localhost","model":"local"}}"#,
        );

        assert!(!flat_endpoint_present(&read_config(&root)));
    }

    #[test]
    fn configured_api_key_env_requires_present_environment() {
        let root = fixture("missing-key");
        write_config(&root, r#"{"endpoint_api_key_env":"LKJAGENT_TEST_ABSENT"}"#);

        assert!(missing_configured_key(&root));
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "lkjagent-smoke-config-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("data")).expect("fixture data dir");
        root
    }

    fn write_config(root: &Path, body: &str) {
        fs::write(root.join("data/lkjagent.json"), body).expect("fixture config");
    }
}
