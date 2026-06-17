use serde_json::{Map, Value};

use crate::wire::CacheMetric;

pub(super) fn collect_cache_metrics(value: &Value) -> Vec<CacheMetric> {
    let mut metrics = Vec::new();
    collect_metrics_at("", value, &mut metrics);
    metrics
}

fn collect_metrics_at(path: &str, value: &Value, metrics: &mut Vec<CacheMetric>) {
    if let Value::Object(map) = value {
        collect_object(path, map, metrics);
    }
}

fn collect_object(path: &str, map: &Map<String, Value>, metrics: &mut Vec<CacheMetric>) {
    for (key, value) in map {
        let next_path = metric_path(path, key);
        if should_collect(path, key) {
            if let Some(text) = metric_value(value) {
                metrics.push(CacheMetric {
                    name: next_path.clone(),
                    value: text,
                });
            }
        }
        collect_metrics_at(&next_path, value, metrics);
    }
}

fn should_collect(path: &str, key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower.contains("cache") || path == "timings" || path.starts_with("timings.")
}

fn metric_path(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

fn metric_value(value: &Value) -> Option<String> {
    match value {
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        Value::String(value) => Some(value.clone()),
        _ => None,
    }
}
