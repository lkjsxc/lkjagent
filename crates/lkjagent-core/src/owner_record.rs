#[rustfmt::skip]
const EXPLICIT_WORDS: &[&str] = &["record ", "record that", "record this", "remember ", "remember that", "note that", "log that", "write down", "journal this", "diary entry", "save this note", "save this", "keep this"];
#[rustfmt::skip]
const TYPED_WORDS: &[&str] = &["todo", "to-do", "meeting", "calendar", "finance", "receipt", "project note", "artifact record"];
#[rustfmt::skip]
const PREFIXES: &[&str] = &["record that ", "record this ", "record ", "remember that ", "remember ", "note that ", "log that ", "write down ", "journal this ", "diary entry ", "save this note ", "save this ", "keep this ", "todo ", "to-do ", "project note ", "artifact record "];

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
    record_intent_from_parts(body, &lower)
}

pub(crate) fn record_intent_from_parts(body: &str, lower: &str) -> Option<RecordIntent> {
    if ambiguous_inbox(body, lower) {
        return None;
    }
    if !explicit_record(body, lower) && !typed_record(body, lower) {
        return None;
    }
    let kind = record_kind(body, lower).to_string();
    Some(RecordIntent {
        title: record_title(&kind, body, lower),
        body: record_body(&kind, body, lower),
        kind,
    })
}

pub(crate) fn ambiguous_inbox(text: &str, lower: &str) -> bool {
    let compact = lower.trim_matches(|ch: char| ch.is_ascii_punctuation() || ch.is_whitespace());
    matches!(
        compact,
        "remember this" | "save this" | "keep this" | "remember it" | "save it" | "keep it"
    ) || matches!(text.trim(), "覚えておいて" | "保存して" | "残して")
}

fn explicit_record(text: &str, lower: &str) -> bool {
    japanese_record(text) || has_any(lower, EXPLICIT_WORDS)
}

fn typed_record(text: &str, lower: &str) -> bool {
    has_any(lower, TYPED_WORDS)
        || has_any(
            text,
            &["やること", "予定", "支払", "レシート", "メモ", "日記"],
        )
}

fn japanese_record(text: &str) -> bool {
    has_any(text, &["記録", "メモ"])
        || text.contains("日記") && has_any(text, &["書", "保存", "残", "つけ"])
        || has_any(text, &["保存して", "残して", "残しといて", "覚えておいて"])
}

fn record_kind(text: &str, lower: &str) -> &'static str {
    if has_any(lower, &["todo", "to-do", "checklist"]) || text.contains("やること") {
        "todo"
    } else if has_any(lower, &["calendar", "meeting", "appointment", "schedule"])
        || has_any(text, &["予定", "会議", "予約"])
    {
        "calendar"
    } else if has_any(lower, &["finance", "receipt", "invoice", "paid", "budget"])
        || text.contains('円')
        || has_any(text, &["支払", "レシート", "家計"])
    {
        "finance"
    } else if has_any(lower, &["project", "milestone"]) || text.contains("プロジェクト") {
        "project"
    } else if has_any(lower, &["artifact", "deliverable"]) || text.contains("成果物") {
        "artifact"
    } else if has_any(lower, &["note", "memo"]) || text.contains("メモ") {
        "note"
    } else {
        "journal"
    }
}

fn record_title(kind: &str, text: &str, lower: &str) -> String {
    let cleaned = cleaned_content(text, lower);
    let seed = if cleaned.is_empty() {
        default_title(kind).to_string()
    } else {
        cleaned
    };
    take_title(&seed)
}

fn record_body(kind: &str, text: &str, lower: &str) -> String {
    if explicit_verbatim(lower) {
        return format!("Verbatim\n\n{}", text.trim());
    }
    let mut cleaned = cleaned_content(text, lower);
    if cleaned.is_empty() || diary_command_only(text) {
        cleaned = "No specific diary details were provided.".to_string();
    }
    match kind {
        "journal" => format!(
            "Summary\n\n{}\n\nReflection\n\nRecorded as a dated journal entry from owner-provided context.",
            cleaned
        ),
        "todo" => format!("Action item\n\n- {}", cleaned),
        "calendar" => format!("Event\n\n{}\n\nDate\n\nUse the owner-provided date or this record date.", cleaned),
        "finance" => format!("Finance note\n\n{}\n\nReview\n\nKeep for monthly finance review.", cleaned),
        "project" => format!("Project note\n\n{}", cleaned),
        "artifact" => format!("Artifact note\n\n{}", cleaned),
        "note" => format!("Note\n\n{}", cleaned),
        _ => cleaned,
    }
}

fn cleaned_content(text: &str, lower: &str) -> String {
    for prefix in PREFIXES {
        if lower.starts_with(prefix) {
            return clean_suffix(text[prefix.len()..].trim()).to_string();
        }
    }
    clean_suffix(text.trim()).to_string()
}

fn clean_suffix(value: &str) -> &str {
    value
        .trim_end_matches("と記録してほしい")
        .trim_end_matches("と記録したい")
        .trim_end_matches("と保存して")
        .trim()
}

fn explicit_verbatim(lower: &str) -> bool {
    has_any(lower, &["verbatim", "exact text", "as written", "そのまま"])
}

fn diary_command_only(text: &str) -> bool {
    text.contains("日記") && text.chars().count() <= 24
}

fn default_title(kind: &str) -> &'static str {
    match kind {
        "journal" => "journal entry",
        "todo" => "todo item",
        "calendar" => "calendar item",
        "finance" => "finance note",
        "project" => "project note",
        "artifact" => "artifact note",
        "note" => "note",
        _ => "record",
    }
}

fn take_title(text: &str) -> String {
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
