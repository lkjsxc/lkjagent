use lkjagent_core::workspace_record::{date_compact, slug};

pub fn normalized_kind(kind: &str) -> &str {
    if kind == "today" {
        "journal"
    } else {
        kind
    }
}

pub fn record_id(kind: &str, now: &str, title: &str) -> String {
    if kind == "journal" {
        if let Some(date) = date_compact(now) {
            return format!("journal_{date}");
        }
    }
    let stamp = now
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>();
    let suffix = slug(title);
    format!("rec_{}_{}", stamp, suffix)
}
