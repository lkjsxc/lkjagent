mod support;

use std::fs;

use lkjagent_cli::config::{load_or_initialize_with_env, ConfigLoad};
use support::{temp_data, TestResult};

#[test]
fn config_loads_explicit_task_turn_budget() -> TestResult<()> {
    let data = temp_data("task-budget")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"model\":\"local-test\"},\"task\":{\"turn-budget\":128}}",
    )?;
    let loaded = load_or_initialize_with_env(&data, |_| None)?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("task budget config did not load".into());
    };
    assert_eq!(config.task_turn_budget, 128);
    Ok(())
}

#[test]
fn first_start_config_renders_default_task_turn_budget() -> TestResult<()> {
    let data = temp_data("task-budget-default")?;
    let loaded = load_or_initialize_with_env(&data, |key| match key {
        "LKJAGENT_MODEL" => Some("local-model".to_string()),
        _ => None,
    })?;
    let ConfigLoad::Ready(config) = loaded else {
        return Err("env-backed config did not load".into());
    };
    assert_eq!(config.task_turn_budget, 64);

    let text = fs::read_to_string(data.join("lkjagent.json"))?;
    assert!(text.contains("\"turn-budget\": 64"));
    Ok(())
}

#[test]
fn too_large_task_turn_budget_fails_loudly() -> TestResult<()> {
    let data = temp_data("task-budget-too-large")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"model\":\"local-test\"},\"task\":{\"turn-budget\":70000}}",
    )?;
    let result = load_or_initialize_with_env(&data, |_| None);
    match result {
        Ok(_) => Err("oversized task budget loaded".into()),
        Err(error) => {
            assert!(error.to_string().contains("task.turn-budget"));
            Ok(())
        }
    }
}
