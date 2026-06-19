use crate::count_profile::Language;

pub(crate) fn anchor_block(language: Language, objective: &str) -> String {
    let anchors = objective_anchors(objective);
    let lines = if anchors.is_empty() {
        match language {
            Language::Japanese => "- 明示された依頼文を優先します。".to_string(),
            Language::English => "- Prioritize the stated owner objective.".to_string(),
        }
    } else {
        anchors
            .iter()
            .map(|anchor| format!("- {anchor}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    match language {
        Language::Japanese => format!("## 要求アンカー\n\n{lines}\n"),
        Language::English => format!("## Requirement Anchors\n\n{lines}\n"),
    }
}

pub(crate) fn anchor_for_part(language: Language, objective: &str, index: usize) -> String {
    let anchors = objective_anchors(objective);
    if anchors.is_empty() {
        return match language {
            Language::Japanese => "明示された依頼文".to_string(),
            Language::English => "the stated owner objective".to_string(),
        };
    }
    let slot = index.saturating_sub(1) % anchors.len();
    anchors[slot].clone()
}

fn objective_anchors(objective: &str) -> Vec<String> {
    let normalized = normalize(objective);
    let mut anchors = clause_anchors(&normalized);
    if anchors.len() < 2 {
        anchors.extend(word_anchors(&normalized));
    }
    dedupe_limit(anchors, 6)
}

fn clause_anchors(objective: &str) -> Vec<String> {
    let mut anchors = Vec::new();
    let mut current = String::new();
    let chars = objective.chars().collect::<Vec<_>>();
    for (index, ch) in chars.iter().enumerate() {
        if is_clause_separator(&chars, index) {
            push_anchor(&mut anchors, &current);
            current.clear();
        } else {
            current.push(*ch);
        }
    }
    push_anchor(&mut anchors, &current);
    anchors
}

fn word_anchors(objective: &str) -> Vec<String> {
    objective
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|word| word.chars().count() >= 4)
        .filter(|word| !is_stop_word(word))
        .map(truncate_anchor)
        .collect()
}

fn push_anchor(anchors: &mut Vec<String>, text: &str) {
    let trimmed = text.trim();
    if trimmed.chars().count() >= 4 {
        anchors.push(truncate_anchor(trimmed));
    }
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_anchor(text: &str) -> String {
    let mut value: String = text.chars().take(72).collect();
    if text.chars().count() > 72 {
        value.push('…');
    }
    value
}

fn dedupe_limit(values: Vec<String>, limit: usize) -> Vec<String> {
    let mut deduped = Vec::new();
    for value in values {
        if !deduped.iter().any(|seen| seen == &value) {
            deduped.push(value);
        }
        if deduped.len() >= limit {
            break;
        }
    }
    deduped
}

fn is_clause_separator(chars: &[char], index: usize) -> bool {
    let ch = chars[index];
    if ch == '.' && decimal_point(chars, index) {
        return false;
    }
    matches!(
        ch,
        '\n' | '\r' | ',' | ';' | '.' | '!' | '?' | '、' | '。' | '；' | '！' | '？'
    )
}

fn decimal_point(chars: &[char], index: usize) -> bool {
    index > 0
        && index.saturating_add(1) < chars.len()
        && chars[index - 1].is_ascii_digit()
        && chars[index + 1].is_ascii_digit()
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "about" | "after" | "before" | "should" | "the" | "that" | "this" | "with"
    )
}
