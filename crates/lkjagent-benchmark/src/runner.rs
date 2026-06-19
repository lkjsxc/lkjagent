#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointConfig {
    pub model_label: String,
    pub endpoint_host: String,
}

pub fn endpoint_config<F>(env: F, dotenv: Option<&str>) -> Result<EndpointConfig, String>
where
    F: Fn(&str) -> Option<String>,
{
    let model = env_value(&env, dotenv, "LKJAGENT_MODEL");
    let endpoint = env_value(&env, dotenv, "LKJAGENT_ENDPOINT_URL");
    match (model, endpoint) {
        (Some(model_label), Some(endpoint_url)) => Ok(EndpointConfig {
            model_label,
            endpoint_host: summarize_host(&endpoint_url),
        }),
        _ => Err(
            "endpoint configuration missing: set LKJAGENT_MODEL and LKJAGENT_ENDPOINT_URL"
                .to_string(),
        ),
    }
}

fn env_value<F>(env: &F, dotenv: Option<&str>, key: &str) -> Option<String>
where
    F: Fn(&str) -> Option<String>,
{
    env(key)
        .and_then(non_empty)
        .or_else(|| dotenv.and_then(|text| dotenv_value(text, key)))
}

fn dotenv_value(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines().find_map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            None
        } else {
            trimmed
                .strip_prefix(&prefix)
                .map(str::to_string)
                .and_then(non_empty)
        }
    })
}

fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn summarize_host(url: &str) -> String {
    let without_scheme = url
        .split_once("://")
        .map_or(url, |(_, rest)| rest)
        .split('/')
        .next()
        .unwrap_or(url);
    without_scheme
        .rsplit('@')
        .next()
        .unwrap_or(without_scheme)
        .to_string()
}
