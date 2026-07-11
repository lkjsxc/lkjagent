use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::config;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn rejects_unknown_composite_and_wrong_type_values() -> TestResult<()> {
    for (name, extra) in [
        ("unknown", r#""surprise":true"#),
        ("array", r#""context_retrieval_limit":[12]"#),
        ("wrong-type", r#""context_retrieval_limit":"12""#),
    ] {
        let data = fixture(name, &format!("{{{base},{extra}}}", base = endpoint()))?;
        assert!(config::load_client(&data).is_err(), "accepted {name}");
    }
    Ok(())
}

#[test]
fn rejects_ranges_and_cross_key_conflicts() -> TestResult<()> {
    for (name, extra) in [
        ("timeout-low", r#""endpoint_timeout_seconds":0"#),
        ("workspace-cap", r#""workspace_file_max_tokens":513"#),
        (
            "prompt-reserve",
            r#""prompt_context_tokens":2048,"prompt_output_reserve_tokens":2048"#,
        ),
        (
            "lane-overflow",
            r#""prompt_context_tokens":2048,"prompt_output_reserve_tokens":256,"context_objective_tokens":512,"context_evidence_tokens":8192,"context_history_tokens":2048,"context_recovery_tokens":1024"#,
        ),
    ] {
        let data = fixture(name, &format!("{{{base},{extra}}}", base = endpoint()))?;
        assert!(config::load_client(&data).is_err(), "accepted {name}");
    }
    Ok(())
}

#[test]
fn tracked_example_matches_the_registry() -> TestResult<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let example: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("data/lkjagent.json"))?)?;
    let object = example.as_object().ok_or("example root is not an object")?;
    assert_eq!(object.len(), 38);
    assert!(object.values().all(is_scalar));
    config::load_client(&root.join("data"))?;
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
