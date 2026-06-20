pub fn short_count(n: u64) -> String {
    const K: u64 = 1_000;
    const M: u64 = 1_000_000;
    const B: u64 = 1_000_000_000;
    match n {
        0..=999 => n.to_string(),
        K..=999_999 => scaled(n, K, "K"),
        M..=999_999_999 => scaled(n, M, "M"),
        _ => scaled(n, B, "B"),
    }
}

pub fn ratio_percent(used: u64, total: u64) -> String {
    if total == 0 {
        return "unknown".to_string();
    }
    format!("{:.2}%", (used as f64 / total as f64) * 100.0)
}

pub fn optional_count(value: Option<u64>) -> String {
    value.map_or_else(|| "unknown".to_string(), short_count)
}

fn scaled(n: u64, unit: u64, suffix: &str) -> String {
    format!("{:.2}{suffix}", n as f64 / unit as f64)
}
