use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::config;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn rejects_unknown_composite_and_wrong_type_values() -> TestResult<()> {
    for (name, extra) in [
        ("unknown", r#""surprise":true"#),
        ("array", r#""endpoint_url":["http://127.0.0.1"]"#),
        ("wrong-type", r#""endpoint_timeout_seconds":"300""#),
    ] {
        let data = fixture(name, &format!("{{{base},{extra}}}", base = endpoint()))?;
        assert!(config::load_client(&data).is_err(), "accepted {name}");
    }
    Ok(())
}

#[test]
fn rejects_ranges_and_workspace_controls() -> TestResult<()> {
    for (name, extra) in [
        ("timeout-low", r#""endpoint_timeout_seconds":0"#),
        ("prompt-low", r#""prompt_context_tokens":2047"#),
        ("workspace-control", r#""workspace_root":"bad\nroot""#),
    ] {
        let data = fixture(name, &format!("{{{base},{extra}}}", base = endpoint()))?;
        assert!(config::load_client(&data).is_err(), "accepted {name}");
    }
    Ok(())
}

#[test]
fn missing_runtime_configuration_uses_defaults() -> TestResult<()> {
    let data = std::env::temp_dir().join(format!(
        "lkjagent-configuration-empty-{}",
        std::process::id()
    ));
    if data.exists() {
        fs::remove_dir_all(&data)?;
    }
    fs::create_dir_all(&data)?;
    config::load_client(&data)?;
    Ok(())
}

#[test]
fn tracked_example_matches_the_registry() -> TestResult<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = fs::read_to_string(root.join("config/lkjagent.example.json"))?;
    let example: serde_json::Value = serde_json::from_str(&text)?;
    let object = example.as_object().ok_or("example root is not an object")?;
    assert_eq!(object.len(), 6);
    assert!(object.values().all(is_scalar));
    config::validate_document(&text)?;
    Ok(())
}

fn endpoint() -> &'static str {
    r#""endpoint_url":"http://127.0.0.1:8080","endpoint_model":"local-model""#
}

fn is_scalar(value: &serde_json::Value) -> bool {
    matches!(
        value,
        serde_json::Value::String(_) | serde_json::Value::Number(_) | serde_json::Value::Bool(_)
    )
}

fn fixture(name: &str, body: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-configuration-contract-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    fs::write(path.join("lkjagent.json"), body)?;
    Ok(path)
}
