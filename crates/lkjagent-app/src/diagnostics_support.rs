use std::path::Path;

pub(crate) fn endpoint_state(data_dir: &Path) -> String {
    let config = crate::config::load_flat_config(data_dir).ok();
    let root = config.as_ref();
    let url = source("LKJAGENT_ENDPOINT_URL", root, "endpoint_url");
    let model = source("LKJAGENT_MODEL", root, "endpoint_model");
    let key_env = root
        .and_then(|value| value.get("endpoint_api_key_env"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("LKJAGENT_API_KEY");
    let credential = if env_present(key_env) {
        "env"
    } else {
        "absent"
    };
    format!("url={url} model={model} credential={credential}")
}

pub(crate) fn missing_dirs(data_dir: &Path) -> Vec<String> {
    let root = data_dir.join("workspace");
    [".", "records", "artifacts", "indexes"]
        .iter()
        .filter(|rel| !root.join(rel).exists())
        .map(|rel| format!("workspace/{rel}"))
        .collect()
}

pub(crate) fn file_count(path: &Path) -> usize {
    std::fs::read_dir(path).map_or(0, |entries| entries.filter_map(Result::ok).count())
}

pub(crate) fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

pub(crate) fn join_counts(values: &[(String, i64)]) -> String {
    values
        .iter()
        .map(|(name, count)| format!("{name}={count}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn join_bools(values: &[(&str, bool)]) -> String {
    values
        .iter()
        .map(|(name, present)| format!("{name}={present}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn source(env: &str, section: Option<&serde_json::Value>, key: &str) -> &'static str {
    if env_present(env) {
        return "env";
    }
    if section
        .and_then(|value| value.get(key))
        .and_then(serde_json::Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        "config"
    } else {
        "absent"
    }
}

fn env_present(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}
