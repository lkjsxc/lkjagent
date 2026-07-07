#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordIntent {
    pub kind: String,
    pub title: String,
    pub body: String,
}

pub fn record_intent(text: &str) -> Option<RecordIntent> {
    let body = text.trim();
    if body.is_empty() {
        return None;
    }
    let lower = body.to_ascii_lowercase();
    if !explicit_record(body, &lower) && !typed_record(&lower) {
        return None;
    }
    let kind = record_kind(body, &lower).to_string();
    Some(RecordIntent {
        kind,
        title: title(body),
        body: body.to_string(),
    })
}

fn explicit_record(text: &str, lower: &str) -> bool {
    text.contains('記') && (text.contains("記録") || text.contains("メモ"))
        || has_any(
            lower,
            &[
                "record ",
                "record that",
                "record this",
                "remember ",
                "remember that",
                "note that",
                "log that",
                "write down",
            ],
        )
}

fn typed_record(lower: &str) -> bool {
    has_any(
        lower,
        &[
            "todo",
            "meeting",
            "calendar",
            "finance",
            "receipt",
            "project note",
            "artifact record",
        ],
    )
}

fn record_kind(text: &str, lower: &str) -> &'static str {
    if has_any(lower, &["todo", "to-do", "checklist"]) || text.contains("やること") {
        "todo"
    } else if has_any(lower, &["calendar", "meeting", "appointment", "schedule"])
        || text.contains("予定")
    {
        "calendar"
    } else if has_any(lower, &["finance", "receipt", "invoice", "paid", "budget"])
        || text.contains('円')
        || text.contains("支払")
    {
        "finance"
    } else if has_any(lower, &["project", "milestone"]) {
        "project"
    } else if has_any(lower, &["artifact", "deliverable"]) {
        "artifact"
    } else if has_any(lower, &["note", "memo"]) || text.contains("メモ") {
        "note"
    } else {
        "journal"
    }
}

fn title(text: &str) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars = compact.chars().take(48).collect::<String>();
    if chars.is_empty() {
        "record".to_string()
    } else {
        chars
    }
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
